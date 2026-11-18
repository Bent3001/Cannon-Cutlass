# Cannon & Cutlass

Cannon & Cutlass ist ein Piratenspiel als WebApp. Zwei Flotten treten im Schiffeversenken gegeneinander an. Die Spieler sammeln Totenschädel für spätere Spezialschüsse. Der nächste Meilenstein ergänzt nach jeder Versenkung ein Echtzeitgefecht aus der Vogelperspektive. Später kommen Flottendecks, Kapitäne, Wortduelle und eine Kampagne dazu.

Das erste Match gegen die KI ist spielbar. Du platzierst fünf Schiffe von Hand oder automatisch, suchst die gegnerische Flotte auf einem 10 × 10 Raster und sammelst einen Totenschädel pro eigenem Zug. Das Match endet, sobald eine Flotte vollständig versenkt ist.

## Lokal starten

Voraussetzungen sind Rust mit Cargo, [just](https://just.systems/man/en/packages.html) und die WebAssembly-Buildwerkzeuge. Unter Windows braucht die GNU-Toolchain passende MinGW-w64-Binutils im `PATH`. Für die MSVC-Toolchain werden die C++-Buildtools von Visual Studio benötigt.

```sh
just setup
just start
```

`just setup` installiert einmalig das Rust-Ziel `wasm32-unknown-unknown` und die zum Projekt passende CLI `wasm-bindgen`. `just start` oder einfach `just` baut den Browser-Spielkern und startet Axum. Die App läuft unter [localhost:3000](http://127.0.0.1:3000/). Mit `CANNON_CUTLASS_ADDR` lässt sich die Bind-Adresse setzen. `GET /healthz` prüft, ob der Axum-Server antwortet.

## Für ChatGPT Sites exportieren

```sh
just export
```

Der Befehl schreibt `dist/index.html` mit CSS, JavaScript und WebAssembly. Der Export ist über einen statischen HTTP-Server spielbar. Für eine Veröffentlichung wird bei ChatGPT Sites ein Projekt registriert und dessen echte Projekt-ID zusammen mit `static.directory = "dist"` in `.openai/hosting.json` hinterlegt. Die Registrierung und Veröffentlichung stehen noch aus. Der [Hosting-Abschnitt](docs/architektur.md#hosting) beschreibt den Unterschied zwischen der statischen Site und dem Axum-Server.

## Projektstruktur

| Pfad | Inhalt |
| --- | --- |
| [Spielkonzept.md](Spielkonzept.md) | Ursprünglicher Entwurf mit der langfristigen Spielvision |
| [docs/spielregeln.md](docs/spielregeln.md) | Erster spielbarer Umfang, vorläufige Regeln und offene Entscheidungen |
| [docs/architektur.md](docs/architektur.md) | Zuständigkeiten, Matchablauf, Schnittstellen und Hosting |
| [docs/roadmap.md](docs/roadmap.md) | Meilensteine mit Abnahmekriterien |
| [docs/tickets/README.md](docs/tickets/README.md) | Backlog mit Tickets, Abnahmekriterien und Abhängigkeiten |
| [TICKETS.md](TICKETS.md) | Ticketwahl, Status und verpflichtender Review vor Abschluss |
| [Justfile](Justfile) | Befehle für Einrichtung, Start, Tests und Export |
| [crates/game-core](crates/game-core/) | Spielregeln, gefilterte Spielersichten und einfache KI |
| [crates/game-wasm](crates/game-wasm/) | Bindings zwischen Rust-Spielkern und Browser-JavaScript |
| [crates/web](crates/web/) | Axum-Server, Maud-Ansichten, Browseroberfläche und statischer Export |
| [AGENTS.md](AGENTS.md) | Hinweise zur Zusammenarbeit im Repository |

## Änderungen prüfen

```sh
just check
```

`just check` prüft die Rust-Formatierung, führt die Tests aus und baut den statischen Export. Mit `just test` laufen nur die Rust-Tests. `just --list` zeigt alle Befehle.
