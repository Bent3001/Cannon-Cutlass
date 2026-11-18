const element = (id) => document.getElementById(id);
const phase = element("phase");
const message = element("message");
const reply = element("reply");
const fleet = [...document.querySelectorAll("[data-ship]")];
const boards = {
    own: [...element("own").querySelectorAll("button")],
    enemy: [...element("enemy").querySelectorAll("button")],
};
const size = Number(element("own").dataset.size);
const marks = { water: "", ship: "■", miss: "·", hit: "×", sunk: "⊗" };
const labels = {
    water: "offen",
    ship: "eigenes Schiff",
    miss: "Fehlschuss",
    hit: "Treffer",
    sunk: "versenkt",
};
const outcomes = { miss: "Fehlschuss", hit: "Treffer", sunk: "Schiff versenkt" };

try {
    const { default: init, BrowserMatch } = await import("./game.js");
    // Pass bytes to the loader so the static host can serve WASM with either MIME type.
    const response = await fetch(new URL("./game_bg.wasm", import.meta.url));
    if (!response.ok) throw new Error("WASM konnte nicht geladen werden.");
    await init({ module_or_path: await response.arrayBuffer() });
    setup(BrowserMatch);
} catch (error) {
    phase.textContent = "Das Match konnte nicht gestartet werden";
    message.textContent = "Lade die Seite erneut. Lokal baut der Befehl just start alle Spieldateien.";
    console.error(error);
}

function setup(BrowserMatch) {
    let game;
    let state;
    let selected = 0;
    let vertical = false;

    function newMatch() {
        game?.free();
        game = new BrowserMatch(crypto.getRandomValues(new Uint32Array(1))[0]);
        selected = 0;
        vertical = false;
        message.textContent = "Stelle deine Flotte auf oder verteile sie automatisch.";
        reply.textContent = "";
        render();
    }

    function render() {
        state = JSON.parse(game.view());
        const { phase: current, player } = state.state;
        const placing = current === "placement";
        const finished = current === "finished";

        element("placement").hidden = !placing;
        element("battle").hidden = placing;
        element("restart").hidden = !finished;
        element("start").disabled = !state.placed.every(Boolean);
        element("orientation").textContent = vertical ? "Ausrichtung: senkrecht" : "Ausrichtung: waagerecht";
        element("skulls").textContent = state.skulls;
        element("enemy-skulls").textContent = state.enemy_skulls;
        element("own-remaining").textContent = placing
            ? `${state.own_remaining} / ${fleet.length} platziert`
            : `${state.own_remaining} übrig`;
        element("enemy-remaining").textContent = placing ? "" : `${state.enemy_remaining} übrig`;
        phase.textContent = placing ? "Aufstellung"
            : finished ? (player === "first" ? "Du hast gewonnen" : "Die KI hat gewonnen")
            : "Du bist am Zug";

        fleet.forEach((button, index) => {
            button.setAttribute("aria-pressed", index === selected);
            button.querySelector(".placed").hidden = !state.placed[index];
        });
        updateBoard("own", placing);
        updateBoard("enemy", current === "turn");
    }

    function updateBoard(side, active) {
        for (const [index, button] of boards[side].entries()) {
            const cell = state[side][index];
            const { x, y } = button.dataset;
            const available = active && (side === "own" || cell === "water");
            button.dataset.cell = cell;
            button.textContent = marks[cell];
            button.setAttribute("aria-label", `${coordinate(x, y)}: ${labels[cell]}`);
            button.setAttribute("aria-disabled", !available);
        }
    }

    function act(action) {
        try {
            action();
            render();
        } catch (error) {
            message.textContent = error.message;
            reply.textContent = "";
        }
    }

    function place(x, y) {
        game.place(selected, x, y, vertical);
        message.textContent = `Schiff ${selected + 1} auf ${coordinate(x, y)} gesetzt.`;
        const next = state.placed.findIndex((placed, index) => !placed && index !== selected);
        if (next >= 0) selected = next;
    }

    function fire(x, y) {
        const [player, computer] = JSON.parse(game.fire(x, y));
        message.textContent = `Du auf ${coordinate(player.target.x, player.target.y)}: ${outcomes[player.outcome]}.`;
        reply.textContent = computer
            ? `KI auf ${coordinate(computer.target.x, computer.target.y)}: ${outcomes[computer.outcome]}.`
            : "";
    }

    for (const [side, buttons] of Object.entries(boards)) {
        buttons[0].tabIndex = 0;
        element(side).addEventListener("click", ({ target }) => {
            if (!target.matches("button") || target.getAttribute("aria-disabled") === "true") return;
            focusCell(side, target);
            const { x, y } = target.dataset;
            act(() => (side === "own" ? place : fire)(Number(x), Number(y)));
            if (state.state.phase === "finished") element("restart").focus();
        });
        element(side).addEventListener("keydown", (event) => {
            if (!event.target.matches("button")) return;
            const { x, y } = event.target.dataset;
            const offset = { ArrowLeft: [-1, 0], ArrowRight: [1, 0], ArrowUp: [0, -1], ArrowDown: [0, 1] }[event.key];
            if (!offset) return;
            event.preventDefault();
            const [nextX, nextY] = [Number(x) + offset[0], Number(y) + offset[1]];
            if (nextX >= 0 && nextX < size && nextY >= 0 && nextY < size) {
                focusCell(side, buttons[nextY * size + nextX]);
            }
        });
    }

    function focusCell(side, button) {
        element(side).querySelector("[tabindex='0']").tabIndex = -1;
        button.tabIndex = 0;
        button.focus();
    }

    for (const [index, button] of fleet.entries()) {
        button.addEventListener("click", () => {
            selected = index;
            message.textContent = `Schiff ${index + 1} gewählt (${button.dataset.length} Felder).`;
            render();
        });
    }
    element("orientation").addEventListener("click", () => {
        vertical = !vertical;
        render();
    });
    element("auto").addEventListener("click", () => act(() => {
        game.auto_place();
        message.textContent = "Die Flotte ist verteilt. Du kannst einzelne Schiffe noch umsetzen.";
    }));
    element("start").addEventListener("click", () => {
        act(() => {
            game.start();
            message.textContent = "Wähle ein Feld im gegnerischen Raster.";
        });
        if (state.state.phase === "turn") focusCell("enemy", boards.enemy[0]);
    });
    element("restart").addEventListener("click", () => {
        newMatch();
        fleet[0].focus();
    });

    newMatch();
    element("match").hidden = false;
}

function coordinate(x, y) {
    return String.fromCharCode(65 + Number(x)) + (Number(y) + 1);
}
