# Technische Architektur

## Komponenten

| Komponente | Verantwortung | Projektstand |
| --- | --- | --- |
| `crates/game-core` | Rust-Domäne für Raster, Flotten, Treffer, Ressourcen und Matchphasen. Verarbeitet Spielaktionen und liefert die Spielersicht. | Rastermatch mit einfacher KI und zuschaltbarem Gefechtsübergang |
| `crates/web` | Axum stellt Seiten und Assets bereit. Maud rendert die Ansicht für den lokalen Server und den statischen Export. | Matchansicht, Server und Export |
| `crates/game-wasm` | Bindet den Spielkern für browserlokale Matches ein. Übersetzt Eingaben und sichtbaren Zustand an der Grenze zu JavaScript. | WebAssembly-Bindings für das Rastermatch |
| Browser-JavaScript | Reicht Eingaben an den Spielkern weiter und aktualisiert die Ansicht. Die Arena bekommt später einen eigenen Canvas. | Platzierung, Spielraster und Ergebnisanzeige |
| Online-Matchdienst | Hält den verbindlichen Matchzustand für PvP und verteilt die jeweilige Spielersicht. | Geplant nach dem Einzelspieler-Prototyp |

Maud und der WASM-Adapter nutzen denselben Spielkern. Die Maud-Ansicht liefert das HTML, der Browser ruft die Rust-Regeln über die von `wasm-bindgen --target web` erzeugten Bindings auf. `just wasm` baut die Bindings nach `target/browser/`. [Axum](https://docs.rs/axum/latest/axum/fn.serve.html) · [Maud-Integration](https://docs.rs/crate/maud/latest/features) · [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)

## Ablauf und Datenfluss

1. **Flotte platzieren.** Der Spielkern prüft Grenzen und Überschneidungen. Beim späteren Flottenbau lassen sich Schiffstypen als belegte relative Rasterkoordinaten beschreiben, die beim Platzieren gedreht und verschoben werden.
2. **Rasterzug beginnen.** Der Spielkern vergibt den Totenschädel einmal beim Beginn eines regulären Zugs. Die Oberfläche liest den verfügbaren Vorrat aus dem Matchzustand.
3. **Schuss ausführen.** Eine Spieleraktion übergibt das Zielfeld. Der Spielkern prüft die Aktion und liefert Fehlschuss, Treffer oder Versenkung. Schiffsfähigkeiten können später zusätzliche Aktionen bereitstellen.
4. **Schiffsgefecht ausspielen.** Ab dem Arena-Meilenstein hält eine Versenkung den Rasterzug an. Der Übergang hält die beteiligten Schiffe und die Fortsetzung fest. Die Arena aktualisiert Bewegung und Schaden in festen Zeitschritten. Die Ansicht zeichnet zwischen den Simulationsschritten.
5. **Match fortsetzen.** Der Spielkern ordnet ein Gefechtsergebnis der Kennung des laufenden Gefechts zu und verarbeitet es genau einmal. Der Solo-Adapter führt einen ausgesetzten KI-Zug nach der Freigabe durch den Spielkern aus. Ein Wortduell kann später als weitere Matchphase folgen. Danach beginnt der nächste reguläre Zug oder der Matchsieg wird angezeigt.

Die KI wählt zufällig aus unbeschossenen Feldern und verwendet dieselbe Schussaktion wie der menschliche Spieler. `SoloMatch::with_encounters` aktiviert den Gefechtsübergang im Kern. Der Browser nutzt bis zur Anbindung der Arena in [CC-204](tickets/CC-204.md) weiter den Rastermodus. Der WASM-Adapter reicht die geordnete Liste bestätigter Schüsse an die Oberfläche weiter. Für Platzierung und Blueprint bleibt dieselbe Feldberechnung im Kern maßgeblich. Im Browser-Prototyp liegt die Partie im Arbeitsspeicher des Tabs. Speicherstände, Accounts und eine Datenbank gehören zu späteren Meilensteinen.

## Hosting

| Ziel | Ausführung |
| --- | --- |
| Lokale Entwicklung | Axum stellt die Maud-Seite und Assets auf `127.0.0.1:3000` bereit. |
| ChatGPT Sites | ChatGPT Sites liefert `dist/` statisch aus. Die Einzelspieler-Partie läuft als Rust-WebAssembly mit einer JavaScript-Oberfläche im Browser. |
| Späteres Online-PvP | Zur statischen Site kann ein separat gehosteter Axum-Matchdienst hinzukommen. Alternativ wäre eine eigene Worker-Integration zu prüfen. Die Entscheidung fällt mit dem Multiplayer-Prototyp. |

ChatGPT Sites unterstützt statische Dateien und für serverseitige Sites einen Cloudflare Worker mit `fetch`-Handler. Der native Axum-Server verwendet einen TCP-Listener und läuft lokal oder auf einem Host mit Prozesslaufzeit. Deshalb ist der statische Maud-Export das Sites-Ziel für den ersten Spielmodus. [Axum-Server](https://docs.rs/axum/latest/axum/fn.serve.html) · [Cloudflare Workers](https://developers.cloudflare.com/workers/runtime-apis/handlers/fetch/)

Bei der späteren Registrierung erhält die Site eine Projekt-ID. Die statische Hosting-Konfiguration verwendet `dist`:

```json
{
  "project_id": "<Projekt-ID von ChatGPT Sites>",
  "static": { "directory": "dist" }
}
```

Der Export und die HTTP-Routen verwenden dieselbe Maud-Ansicht und dieselben Browser-Assets.
