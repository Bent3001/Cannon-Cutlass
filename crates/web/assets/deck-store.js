const storageKey = "cannon-cutlass.decks";
const schemaVersion = 1;
const standardDeckId = "standard";

function hasText(value) {
    return typeof value === "string" && value.trim().length > 0;
}

function isShipList(ships) {
    return Array.isArray(ships) && ships.length > 0 && Array.from(ships).every(hasText);
}

function copyShips(ships) {
    if (!isShipList(ships)) throw new TypeError("Eine Schiffsliste braucht gültige Schiffskennungen.");
    return [...ships];
}

function deckName(name) {
    if (!hasText(name)) throw new TypeError("Ein Deck braucht einen Namen.");
    return name.trim();
}

function findDeck(state, id) {
    const deck = state.decks.find((entry) => entry.id === id);
    if (!deck) throw new RangeError("Das Deck wurde nicht gefunden.");
    return deck;
}

export function openDeckStore(standardShips, storage = globalThis.localStorage) {
    const standard = copyShips(standardShips);

    function fallback() {
        return {
            version: schemaVersion,
            activeId: standardDeckId,
            decks: [{ id: standardDeckId, name: "Standardflotte", ships: [...standard] }],
        };
    }

    function read() {
        const raw = storage.getItem(storageKey);
        if (raw === null) return fallback();

        let data;
        try {
            data = JSON.parse(raw);
        } catch (error) {
            if (error instanceof SyntaxError) return fallback();
            throw error;
        }

        if (data?.version !== schemaVersion || !Array.isArray(data.decks) || data.decks.length === 0) {
            return fallback();
        }

        const ids = new Set();
        const decks = [];
        for (const deck of data.decks) {
            if (!hasText(deck?.id) || ids.has(deck.id) || !hasText(deck.name) || !isShipList(deck.ships)) {
                return fallback();
            }
            ids.add(deck.id);
            decks.push({ id: deck.id, name: deck.name, ships: deck.ships });
        }

        const activeId = ids.has(data.activeId) ? data.activeId : decks[0].id;
        return { version: schemaVersion, activeId, decks };
    }

    function write(state) {
        storage.setItem(storageKey, JSON.stringify(state));
    }

    function append(state, name, ships) {
        const deck = { id: crypto.randomUUID(), name: deckName(name), ships: copyShips(ships) };
        state.decks.push(deck);
        write(state);
        return deck.id;
    }

    return {
        list: read,
        create(name) {
            return append(read(), name, standard);
        },
        duplicate(id, name) {
            const state = read();
            return append(state, name, findDeck(state, id).ships);
        },
        rename(id, name) {
            const state = read();
            findDeck(state, id).name = deckName(name);
            write(state);
        },
        save(id, ships) {
            const state = read();
            findDeck(state, id).ships = copyShips(ships);
            write(state);
        },
        remove(id) {
            const state = read();
            findDeck(state, id);
            state.decks = state.decks.filter((deck) => deck.id !== id);
            if (state.decks.length === 0) {
                write(fallback());
                return;
            }
            if (state.activeId === id) state.activeId = state.decks[0].id;
            write(state);
        },
        activate(id) {
            const state = read();
            state.activeId = findDeck(state, id).id;
            write(state);
        },
        getActiveDeck() {
            const state = read();
            return findDeck(state, state.activeId);
        },
    };
}
