# Spielregeln und offene Entscheidungen

Die jüngste Projektbeschreibung legt die Reihenfolge fest. Wir beginnen mit den fünf Standardschiffen, Totenschädeln und dem kostenlosen Standardschuss. Das Echtzeitgefecht nach dem Versenken ist Teil des Spielkonzepts. Spezialschüsse und individuelle Flottendecks werden danach ausgestaltet. Der [ursprüngliche Entwurf](../Spielkonzept.md) enthält weitere Ideen und alternative Regeln.

## Erstes spielbares Match

| Thema | Arbeitsstand |
| --- | --- |
| Gegner | Eine Person spielt im Browser gegen eine einfache KI. |
| Raster und Flotte | Zwei 10 × 10 Raster. Pro Flotte fünf gerade Testboote der Längen 2, 3, 4, 5 und 6 mit insgesamt 20 Feldern. |
| Platzierung | Horizontal oder vertikal, vollständig im Raster. Schiffe dürfen sich berühren. Belegte Felder dürfen sich nicht überschneiden. |
| Zug | Abwechselnd ein gültiger Schuss pro Spieler. Auch nach einem Treffer wechselt der Zug. Ein bereits beschossenes Feld verbraucht keinen Zug. |
| Totenschädel | Jeder Spieler beginnt mit null und erhält zu Beginn jedes eigenen regulären Zugs einen, einschließlich des ersten. |
| Standardschuss | Kostet null Totenschädel und greift genau ein Feld an. |
| Versenken | Ein Schiff sinkt, sobald alle zugehörigen Felder getroffen sind. |
| Matchsieg | Wer die gegnerische Flotte vollständig versenkt, gewinnt. Sobald die Arena umgesetzt ist, wird das letzte Zusatzgefecht vorher abgeschlossen. |

Rastergröße, Schiffslängen, Platzierung, Zugwechsel und der genaue Zeitpunkt der Gutschrift sind vorläufige Regeln für den Prototyp. Im ersten Match wirken sich Totenschädel auf die Anzeige aus.

## Testflotte und spätere Schiffstypen

Die Nummern der Testboote entsprechen ihrer Länge. Bei der automatischen Platzierung kommt das längste Boot zuerst. Gespeicherte Standarddecks behalten ihre Kennungen: `standard-0` steht jetzt für Testboot 6 und die weiteren Kennungen bis `standard-4` stehen in dieser Reihenfolge für Testboot 5, 4, 3 und 2. Deckname und aktive Auswahl bleiben so erhalten. [CC-801](tickets/CC-801.md)

Alle Testboote verwenden den kostenlosen Standardschuss und haben keine eigene Spezialfähigkeit. Für die spätere Arena-Anbindung liegt ein gemeinsames Bewegungsprofil im Spielkern. Es verwendet 70 Einheiten/s² Beschleunigung, 120 Einheiten/s² Bremswirkung, 120 Einheiten/s Höchsttempo und 2,1 rad/s Drehrate.

Für die Grundsystem-Abnahme können Testszenarien später Spezialangriffe und abweichende Arenaprofile gezielt erproben. Dingy, Schaluppe, Briggantin, Galeone, Katamaran und Sturmbark folgen nach dieser Abnahme. Ihre Formen, Kanonen und Angriffskosten stehen in den einzelnen Schiffstickets. [CC-802](tickets/CC-802.md) · [Schiffe und Testflotte](tickets/README.md#schiffe-und-testflotte)

## Beschlossener UI-Ausbau

Der Vorrat wird pro Spieler auf **acht Totenschädel** begrenzt. Jeder eigene reguläre Zug schreibt wie bisher einen Schädel gut, bis der Vorrat voll ist. Die Grenze gilt auch für spätere Ressourcenbelohnungen. [CC-709](tickets/CC-709.md) verfolgt die Umsetzung im Spielkern.

Die Skull-Bar zeigt acht sichtbare Schädelplätze mit gefüllten Symbolen und leeren Konturen. Über der Leiste stehen die Angriffe gruppiert nach Schiff. Künftige Angriffe erscheinen zunächst als Platzhalter. Der vorhandene Standardschuss bleibt kostenlos. [CC-710](tickets/CC-710.md) · [CC-711](tickets/CC-711.md)

Bei der Aufstellung zeigt ein Blueprint vor dem Setzen alle betroffenen Felder. Für die Tastenbelegung ist **X waagerecht** und **Y senkrecht** vorgesehen. [CC-707](tickets/CC-707.md)

## Schiffskampf

Sobald der Arena-Meilenstein umgesetzt ist, pausiert das Rasterspiel nach jeder Versenkung. Das auslösende Schiff des Angreifers trifft in einer separaten Ansicht auf das gerade versenkte Schiff des Verteidigers. Der Rasterzustand behält die Versenkung bei. Im Solo-Match steuert der Mensch immer das eigene Schiff. Nach Abschluss beginnt der ausgesetzte Rasterzug des Verteidigers oder das Match endet nach der letzten Versenkung. Ist der Verteidiger die KI, führt sie ihren ausgesetzten Zug automatisch aus. Eine weitere Versenkung in diesem Zug löst ein neues Gefecht aus. Bewegung, Kanonenschüsse, Treffer und eine kurze Zeitgrenze bilden den ersten Umfang der Arena.

Bis zur aktiven Schiffsauswahl in [CC-302](tickets/CC-302.md) fährt der Angreifer das eigene intakte Schiff mit der niedrigsten Kennung. Belohnungen, Wind und Entermanöver sind noch offen. Der Entwurf enthält unterschiedliche Vorschläge dazu, welche Gefechtsausgänge ein Wortduell auslösen. Diese Entscheidung fällt bei der Planung der Wortduelle.

## Spätere Mechaniken

- **Spezialschüsse.** Sonar und Flächenschaden sind Kandidaten für den ersten Testangriff. Die Kosten der späteren schiffsspezifischen Angriffe stehen in den Schiffstickets. Der Totenschädel-Vorrat ist auf acht begrenzt.
- **Flottendecks.** Spieler stellen später eine Flotte aus freigeschalteten Schiffen zusammen. Schiffstypen können eigene Fähigkeiten und Rasterformen wie L oder U haben.
- **Wortduelle.** Kapitäne wählen aus Piratenbeleidigungen und passenden Kontern. Punkte, Auslöser und Belohnungen werden gemeinsam festgelegt.
- **Fortschritt.** Kapitäne, Fähigkeiten, sammelbare Karten, Freischaltungen und Kampagne sind spätere Meilensteine.
