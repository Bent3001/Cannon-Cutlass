"use strict";

const columns = "ABCDEFGHIJ";
const positions = {
    ship: ["B3", "C3", "D3", "E3", "F3"],
    own: ["B3", "C3", "D3", "E3", "F3", "H3", "H4", "H5", "H6",
        "B6", "B7", "B8", "D9", "E9", "F9", "I9", "J9"],
};
const previews = {
    valid: { cells: ["C6", "D6", "E6", "F6"], note: "Schiff 2 auf C6 setzen." },
    overlap: { cells: ["A3", "B3", "C3", "D3"], note: "Überlappung mit Schiff 1 auf B3 bis D3." },
    edge: { cells: ["I6", "J6"], note: "Start auf I6 · 2 Felder ragen rechts über das Raster hinaus." },
};
const symbols = {
    ship: ["■", "Schiff"],
    miss: ["·", "Fehlschuss"],
    hit: ["×", "Treffer"],
    sunk: ["⊗", "Versenkt"],
    blueprint: ["", "Platzierungsvorschau"],
    invalid: ["!", "Ungültige Platzierung"],
    "invalid ship": ["■", "Schiff · Überlappung"],
};

function makeBoard(id) {
    const table = document.getElementById(id);
    const header = table.createTHead().insertRow();
    const body = table.createTBody();
    const fields = new Map();
    header.insertCell();
    for (const column of columns) {
        const heading = document.createElement("th");
        heading.scope = "col";
        heading.textContent = column;
        header.append(heading);
    }

    for (let rowNumber = 1; rowNumber <= 10; rowNumber += 1) {
        const row = body.insertRow();
        const heading = document.createElement("th");
        heading.scope = "row";
        heading.textContent = rowNumber;
        row.append(heading);
        for (const column of columns) {
            const coordinate = column + rowNumber;
            const field = document.createElement("span");
            field.className = "cell";
            field.setAttribute("aria-hidden", "true");
            row.insertCell().append(field);
            fields.set(coordinate, field);
        }
    }
    return fields;
}

function setState(fields, coordinate, state) {
    const field = fields.get(coordinate);
    field.className = state ? "cell " + state : "cell";
    field.textContent = state ? symbols[state][0] : "";
    field.parentElement.setAttribute("aria-label", state ? symbols[state][1] : "Wasser");
}

function addFocusExample(fields, coordinate, label) {
    const field = fields.get(coordinate);
    const button = document.createElement("button");
    button.type = "button";
    button.className = field.className;
    button.setAttribute("aria-label", label);
    field.replaceWith(button);
    fields.set(coordinate, button);
}

const placement = makeBoard("placement-board");
const placementNote = document.getElementById("placement-note");
const previewButtons = document.querySelectorAll("[data-preview]");
let activePreview;

function showPreview(name) {
    if (activePreview) {
        for (const coordinate of previews[activePreview].cells) {
            setState(placement, coordinate, positions.ship.includes(coordinate) ? "ship" : "");
        }
    }

    const preview = previews[name];
    for (const coordinate of preview.cells) {
        const blocked = positions.ship.includes(coordinate);
        const state = name === "valid" ? "blueprint" : blocked ? "invalid ship" : "invalid";
        setState(placement, coordinate, state);
    }
    activePreview = name;
    placementNote.textContent = preview.note;
    for (const button of previewButtons) {
        button.setAttribute("aria-pressed", button.dataset.preview === name);
    }
}

for (const coordinate of positions.ship) {
    setState(placement, coordinate, "ship");
}
for (const button of previewButtons) {
    button.addEventListener("click", () => showPreview(button.dataset.preview));
}
showPreview("valid");

const enemy = makeBoard("enemy-board");
setState(enemy, "A2", "miss");
setState(enemy, "D3", "hit");
setState(enemy, "G7", "sunk");
setState(enemy, "H7", "sunk");
addFocusExample(enemy, "D4", "D4 · mögliches Ziel für den Standardschuss");

const own = makeBoard("own-board");
for (const coordinate of positions.own) {
    setState(own, coordinate, "ship");
}
setState(own, "D3", "hit");
setState(own, "C4", "miss");
setState(own, "H8", "miss");
setState(own, "A9", "miss");
