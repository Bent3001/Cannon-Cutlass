# Cannon & Cutlass

## Einstieg

- Lies bei Änderungen an Spielmechaniken zuerst [docs/spielregeln.md](docs/spielregeln.md) und bei technischen Änderungen [docs/architektur.md](docs/architektur.md).
- Verwende **Cannon & Cutlass** in der WebApp und den Projektdokumenten. [Spielkonzept.md](Spielkonzept.md) enthält den ursprünglichen Entwurf mit seinen Arbeitstiteln und früheren Vorschlägen.
- Halte neue Spielentscheidungen in den Spielregeln fest. Aktualisiere die Roadmap, wenn sich die Reihenfolge oder der Umfang eines Meilensteins ändert.
- Wähle Aufgaben aus dem [Ticket-Backlog](docs/tickets/README.md) nach Projektstand und Abhängigkeiten. Befolge den Ablauf in [TICKETS.md](TICKETS.md). Ein Ticket gilt erst nach dokumentiertem Review durch den Projektinhaber oder eine KI als abgeschlossen.

## Zuständigkeiten

- Spielzustand, Validierung und Übergänge gehören in `crates/game-core`. Der Spielkern bleibt unabhängig von Axum, Tokio, Maud und Browser-APIs. Führe Zufall und Zeit bei Bedarf als Eingaben zu, damit sich ein Ablauf reproduzieren lässt.
- HTML und gemeinsame Ansichten liegen in `crates/web`. Verwende dieselben Maud-Ansichten für den Axum-Server und den Sites-Export. Browser-JavaScript übernimmt Eingabe, Darstellung und bei Bedarf den Aufruf von WebAssembly.
- Erhalte den statischen Startweg bei Änderungen an Assets und Ansichten. Verwende relative Asset-URLs und prüfe, dass der Export alle vom Browser geladenen Dateien enthält.
- Wenn Online-PvP dazukommt, validiert der Server Spielaktionen. Jede Partei erhält nur ihre Sicht auf das Match.

## Vor Abschluss

- Prüfe Rust-Änderungen mit `just check`. Der Befehl prüft Formatierung und Tests und baut den statischen Export.
- Prüfe bei Änderungen am Browserablauf, dass Axum und der statische Export dieselbe Partie bis zum Ende spielen können.
- Behandle `target/` und `dist/` als generierte Verzeichnisse. Pflege Abhängigkeiten über Cargo und versioniere `Cargo.lock` zusammen mit den Manifesten.
