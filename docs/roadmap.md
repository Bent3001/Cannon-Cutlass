# Entwicklungsplan

Die einzelnen Aufgaben und ihre Abhängigkeiten stehen im [Ticket-Backlog](tickets/README.md).

Der [UI- und UX-Ausbau](tickets/README.md#ui-und-ux) umfasst Hauptmenü, Captain-Erstellung, gespeicherte Decks, Platzierung mit Blueprint, Schussrückmeldung und die Skull-Bar mit acht Plätzen. Diese Arbeiten bauen auf dem spielbaren Rastermatch auf und können parallel zur Arena abgenommen werden. Ist die Arena bereits integriert, umfasst die UI-Abnahme auch die Rückkehr ins Raster und das Matchende nach dem letzten Gefecht. Kampagne und PvP erhalten mit ihrer Umsetzung eigene Einstiege im Menü.

Der [Schiffsausbau](tickets/README.md#schiffe-und-testflotte) beginnt mit fünf neutralen Testbooten. Die sechs benannten Schiffstypen folgen, sobald ihre Grundsysteme mit den Testbooten und gezielten Testszenarien abgenommen sind.

## 0 · Projektstart

Ein Cargo-Workspace enthält den Spielkern und die WebApp. Axum startet lokal, Maud rendert die Startseite und der Export erzeugt die statischen Dateien für ChatGPT Sites. Spielregeln und offene Entscheidungen sind dokumentiert.

## 1 · Erstes spielbares Match (umgesetzt)

- Rust-Spielkern für Raster, Platzierung, Zugwechsel, Treffer, Versenkung und Sieg.
- Totenschädel am Zuganfang und ein kostenloser Standardschuss.
- Browseroberfläche mit manueller und automatischer Platzierung, zwei Rastern und Rückmeldung zu jedem Schuss.
- Einfache KI und WASM-Adapter für die lokale WebApp und den statischen Export.

**Abnahme** Eine Person kann eine Partie mit fünf Standardschiffen im Browser aufstellen und gegen die KI bis zum Sieg oder zur Niederlage spielen. Dieselbe Partie lässt sich über den statischen Export starten.

## 2 · Schiffskampf

Eine Versenkung öffnet eine separate Arena. Beide Schiffe können sich bewegen und schießen. Schaden, Zeitlimit und Gefechtsausgang sind sichtbar.

**Abnahme** Nach dem Gefecht beginnt der ausgesetzte Rasterzug genau einmal. Eine ausgesetzte KI-Antwort wird automatisch ausgeführt und kann ihrerseits ein Gefecht auslösen. Bei der letzten Versenkung erscheint nach dem Gefecht der Matchsieg.

**Späterer Ausbau** Arenaprofile für verschiedene Schiffstypen bauen auf dem Schiffskatalog auf. Wind, Manöver und Energiekosten gehen nach Freigabe in das Match ein. Entern wird als eigener Übergang zum Wortduell erprobt.

## 3 · Spezialschüsse und Flottenbau

Eine zusätzliche Angriffsart und ihre Kosten festlegen und mit der Standardflotte erproben. Flottenauswahl, Fähigkeiten und neue Formen erhalten eine eigene Abnahme. Die zweite Angriffsart und ein Deckvergleich mit beiden Varianten können nach eigener Freigabe folgen.

**Abnahme Spezialschuss** Eine Standardflotte spielt eine vollständige Partie mit dem ausgewählten Angriff. Der Spielkern prüft Kosten und Wirkung, der Spieltest dokumentiert den taktischen Nutzen.

**Abnahme Flottenbau** Eine gültige eigene Flotte mit interner Testform lässt sich aufstellen und bis zum Matchende spielen. Der Spielkern prüft Deck und Platzierung.

**Späterer Ausbau** Nach der Grundsystem-Abnahme erhält der Schiffskatalog die sechs benannten Typen und ihre individuellen Profile. Freigegebene Durchschlagskugeln und legendäre Kanonen werden umgesetzt. Ein eigener Spieltest prüft Standardflotte, Aufklärung und Flächenschaden, sobald beide Angriffsvarianten verfügbar sind.

## 4 · Wortduelle und Kapitäne

Zunächst ein kurzer, fester Satz aus eigenen Piratenbeleidigungen und Kontern. Die ausgelösten Duelle führen mit der vereinbarten Belohnung wieder in die Partie. Die Abnahme prüft das freie Solo-Match mit allen drei Kampfphasen und misst, wie lange das Rasterspiel unterbrochen wird. Danach folgen erweiterte Charaktergestaltung, mehrere auswählbare Kapitäne, vier spielbare Spezialisierungen und sammelbare Duellkarten. Ein gültiger Kartensatz kann vor dem Match ausgerüstet und im Duell verwendet werden.

## 5 · Online-PvP und Kampagne

**Online-PvP** Der erste Umfang wird mit verbindlichem Matchzustand und getrennten Spielersichten festgelegt. Der Ausbau führt zu einem vollständigen Match mit eigenen Flotten, Arena und Wortduell. Ein Spieltest vergleicht die Optionen von Einsteigern mit freigeschalteten Kombinationen.

**Kampagne und Fortschritt** Auf die erste Begegnung folgen eine zusammenhängende Meereskarte, taktisch erweiterte KI, entdeckbare Orte und Regionen sowie ein Boss. Die Kampagne schaltet Schiffe, Kapitäne, Karten und Kosmetik frei. Kapitänserfahrung und ein erster Skilltree erhalten eine eigene Abnahme. Der Kampagnenspieltest reicht vom ersten Aufbruch bis zum Boss mit einer unterwegs verdienten und ausgerüsteten Belohnung.

## Gestaltung und Veröffentlichung

Die Arena erhält eigene Schiffe und lesbare Wasser- und Kampfeffekte. Wortduelle erhalten Kapitänsporträts, Reaktionen und Übergänge. Ein separater Vergleich klärt, ob eine schräge Kamera oder 2,5D für Arena und Kampagnenkarte einen Mehrwert bringt. Eine private Veröffentlichung kann bereits mit einem früheren spielbaren Stand erfolgen.
