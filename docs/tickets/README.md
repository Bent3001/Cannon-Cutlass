# Tickets für Cannon & Cutlass

Dieses Backlog konkretisiert die [Roadmap](../roadmap.md) und das [Spielkonzept](../../Spielkonzept.md). Der Projektstart und das erste Rastermatch gegen die KI sind umgesetzt. Die Tickets beginnen mit Meilenstein 2. Aus dem Konzept übernommene Zahlen und Varianten gelten als Vorschläge, bis sie im jeweiligen Designticket entschieden sind.

## Als Nächstes

Der Gefechtsübergang im Spielkern ist umgesetzt und liegt in [CC-201](CC-201.md) zum Review vor. [CC-202 · Schiffsbewegung für die Arena simulieren](CC-202.md) kann unabhängig davon beginnen. Der Meilenstein ist nach dem Review des End-to-End-Spieltests in [CC-206](CC-206.md) abgeschlossen. [CC-207](CC-207.md) untersucht spätere Erweiterungen der Arena. Parallel können im UI-Strang [CC-701](CC-701.md), [CC-705](CC-705.md) und [CC-709](CC-709.md) beginnen.

Die Spalte **Vorgänger** nennt die direkt benötigten Tickets. Status und Abnahmekriterien stehen in der jeweiligen Ticketdatei. Spielregeln und Designvorgaben stehen in den [Spielregeln](../spielregeln.md), technische Entscheidungen in der [Architektur](../architektur.md).

Für Auswahl, Bearbeitung und Abschluss gilt der Ablauf in [TICKETS.md](../../TICKETS.md). Vorgänger zählen nach bestandenem Review als abgeschlossen.

## UI und UX

Die Arbeiten an Menü, Deckverwaltung, Aufstellung und Rasterdarstellung können auf dem aktuellen Prototyp beginnen. [CC-701](CC-701.md) legt die visuelle Richtung fest. [CC-705](CC-705.md) und [CC-709](CC-709.md) bereiten unabhängig davon die gespeicherten Decks und die Ressourcengrenze vor. [CC-715](CC-715.md) prüft den Rasterablauf unabhängig vom Arena-Meilenstein. Die Abnahme kann parallel zur Arena-Entwicklung erfolgen. Ist die Arena zum Testzeitpunkt bereits integriert, prüft CC-715 auch den Ansichtswechsel und das verzögerte Matchende. [CC-206](CC-206.md) bleibt die Abnahme für den vollständigen Arena-Ablauf. [CC-716](CC-716.md) macht später die jeweils verfügbaren Spielmodi über das Hauptmenü erreichbar.

| Ticket | Typ | Vorgänger |
| --- | --- | --- |
| [CC-701 · Visuelle Grundlage für Menü und Match definieren](CC-701.md) | Gestaltung | — |
| [CC-702 · Hauptmenü mit Captain, Spielen und Flotte bauen](CC-702.md) | Entwicklung | [CC-701](CC-701.md) |
| [CC-703 · Piratiges Platzhalterbild für das Hauptmenü vorsehen](CC-703.md) | Gestaltung | [CC-702](CC-702.md) |
| [CC-704 · Captain-Karte und einfache Erstellung im Menü anbinden](CC-704.md) | Entwicklung | [CC-702](CC-702.md) |
| [CC-705 · Mehrere Flottendecks und das aktive Deck speichern](CC-705.md) | Entwicklung | — |
| [CC-706 · Decks im Flottenbereich auswählen und verwalten](CC-706.md) | Entwicklung | [CC-702](CC-702.md), [CC-705](CC-705.md) |
| [CC-707 · Schiffsplatzierung mit Hover-Blueprint und X/Y bauen](CC-707.md) | Entwicklung | [CC-701](CC-701.md) |
| [CC-708 · Aufstellung mit Schiffsübersicht und Korrekturen verbessern](CC-708.md) | Entwicklung | [CC-707](CC-707.md) |
| [CC-709 · Totenschädel im Spielkern auf acht begrenzen](CC-709.md) | Entwicklung | — |
| [CC-710 · Skull-Bar mit acht sichtbaren Schädelplätzen gestalten](CC-710.md) | Entwicklung | [CC-701](CC-701.md), [CC-709](CC-709.md) |
| [CC-711 · Angriffsleiste über der Skull-Bar mit Platzhaltern bauen](CC-711.md) | Entwicklung | [CC-710](CC-710.md) |
| [CC-712 · Schuss, Treffer, Fehlschuss und Versenkung sichtbar machen](CC-712.md) | Entwicklung | [CC-701](CC-701.md) |
| [CC-713 · Zugstatus, Matchende und Rückkehr ins Menü gestalten](CC-713.md) | Entwicklung | [CC-702](CC-702.md), [CC-712](CC-712.md) |
| [CC-714 · Platzierung und Match für Touch und Tastatur nutzbar machen](CC-714.md) | Entwicklung | [CC-706](CC-706.md), [CC-707](CC-707.md), [CC-711](CC-711.md), [CC-712](CC-712.md), [CC-713](CC-713.md) |
| [CC-715 · Menü, Aufstellung und Match als zusammenhängenden Ablauf testen](CC-715.md) | Spieltest | [CC-703](CC-703.md), [CC-704](CC-704.md), [CC-708](CC-708.md), [CC-713](CC-713.md), [CC-714](CC-714.md) |
| [CC-716 · Solo-Match, Kampagne und Online-PvP über das Hauptmenü erreichbar machen](CC-716.md) | Entwicklung | [CC-702](CC-702.md) |

## Schiffe und Testflotte

[CC-801](CC-801.md) legt die fünf neutralen Testboote an. [CC-802](CC-802.md) nimmt die Grundsysteme mit dieser Flotte und gezielten Testszenarien ab. Erst danach werden Dingy, Schaluppe, Briggantin, Galeone, Katamaran und Sturmbark umgesetzt. [CC-311](CC-311.md) integriert die sechs fertigen Schiffstypen in den spielbaren Katalog.

| Ticket | Typ | Vorgänger |
| --- | --- | --- |
| [CC-801 · Fünf neutrale Testboote 2 bis 6 bereitstellen](CC-801.md) | Entwicklung | — |
| [CC-802 · Grundsysteme mit Testbooten abnehmen](CC-802.md) | Spieltest | [CC-801](CC-801.md), [CC-208](CC-208.md), [CC-210](CC-210.md), [CC-302](CC-302.md), [CC-306](CC-306.md), [CC-412](CC-412.md), [CC-715](CC-715.md) |
| [CC-811 · Dingy als windunabhängiges Ruderboot umsetzen](CC-811.md) | Entwicklung | [CC-802](CC-802.md) |
| [CC-812 · Schaluppe mit treffendem Sonarschuss umsetzen](CC-812.md) | Entwicklung | [CC-802](CC-802.md) |
| [CC-813 · Briggantin mit Ankerwende und Dreifachschuss umsetzen](CC-813.md) | Entwicklung | [CC-802](CC-802.md) |
| [CC-814 · Galeone mit Kreuzschuss umsetzen](CC-814.md) | Entwicklung | [CC-802](CC-802.md) |
| [CC-815 · Katamaran mit Richtungsaufklärung umsetzen](CC-815.md) | Entwicklung | [CC-802](CC-802.md) |
| [CC-816 · Sturmbark als Viermaster mit Sperrfeuer umsetzen](CC-816.md) | Entwicklung | [CC-802](CC-802.md) |

## 2 · Schiffskampf

Grundlage ist das [Spielkonzept](../../Spielkonzept.md) in den Abschnitten 2.2, 3 und 11. [CC-206](CC-206.md) nimmt den ersten Arena-Ablauf ab. Unterschiedliche Schiffs- und Kanonenprofile folgen mit dem Schiffskatalog in [CC-208](CC-208.md). [CC-207](CC-207.md) erprobt Wind, Manöver und Energiekosten. [CC-210](CC-210.md) setzt die freigegebenen Varianten um. Entern wird separat in [CC-209](CC-209.md) untersucht.

| Ticket | Typ | Vorgänger |
| --- | --- | --- |
| [CC-201 · Rasterspiel bei einer Versenkung pausieren](CC-201.md) | Entwicklung | — |
| [CC-202 · Schiffsbewegung für die Arena simulieren](CC-202.md) | Entwicklung | — |
| [CC-203 · Arenakanonen, Treffer und Gefechtsausgang umsetzen](CC-203.md) | Entwicklung | [CC-202](CC-202.md) |
| [CC-204 · Separate Gefechtsansicht im Browser anbinden](CC-204.md) | Entwicklung | [CC-201](CC-201.md), [CC-203](CC-203.md) |
| [CC-205 · Eine KI für das Echtzeitgefecht ergänzen](CC-205.md) | Entwicklung | [CC-203](CC-203.md) |
| [CC-206 · Den gesamten Raster- und Arenakreislauf abnehmen](CC-206.md) | Spieltest | [CC-204](CC-204.md), [CC-205](CC-205.md) |
| [CC-207 · Wind und Manöver für die Arena erproben](CC-207.md) | Design und Prototyp | [CC-206](CC-206.md) |
| [CC-208 · Arena-Schiffstypen und Kanonenprofile ausarbeiten](CC-208.md) | Entwicklung | [CC-305](CC-305.md) |
| [CC-209 · Entermanöver und Übergang zum Wortduell erproben](CC-209.md) | Design und Prototyp | [CC-401](CC-401.md) |
| [CC-210 · Freigegebene Windregeln und Arenamanöver ins Match übernehmen](CC-210.md) | Entwicklung | [CC-207](CC-207.md), [CC-208](CC-208.md) |

## 3 · Spezialschüsse und Flottenbau

Grundlage ist das [Spielkonzept](../../Spielkonzept.md) in den Abschnitten 5, 6 und 11. [CC-301](CC-301.md) gibt genau einen ersten Spezialschuss frei. [CC-307](CC-307.md) nimmt den gewählten Angriff mit der Standardflotte ab. [CC-305](CC-305.md) und [CC-306](CC-306.md) bilden den eigenen Flottenbau. Nach Freigabe und Umsetzung beider Varianten folgt der Deckvergleich in [CC-310](CC-310.md). [CC-309](CC-309.md) setzt die in CC-308 freigegebenen weiteren Kanonen um.

| Ticket | Typ | Vorgänger |
| --- | --- | --- |
| [CC-301 · Ersten Spezialschuss und seine Kosten entscheiden](CC-301.md) | Design | [CC-206](CC-206.md) |
| [CC-302 · Aktives Schiff und Kostenprüfung für Schüsse einführen](CC-302.md) | Entwicklung | [CC-301](CC-301.md), [CC-709](CC-709.md) |
| [CC-303 · Sonarschuss als Aufklärungsaktion bauen](CC-303.md) | Entwicklung | [CC-302](CC-302.md) |
| [CC-304 · Große Bombe und mehrere Versenkungen abwickeln](CC-304.md) | Entwicklung | [CC-302](CC-302.md) |
| [CC-305 · Schiffskatalog und drehbare Schiffsformen modellieren](CC-305.md) | Entwicklung | [CC-206](CC-206.md), [CC-801](CC-801.md) |
| [CC-306 · Flottendeck vor dem Match zusammenstellen](CC-306.md) | Entwicklung | [CC-305](CC-305.md), [CC-706](CC-706.md), [CC-707](CC-707.md) |
| [CC-307 · Den ersten Spezialschuss im Spieltest balancieren](CC-307.md) | Spieltest | Das in [CC-301](CC-301.md) ausgewählte Angriffsticket ([CC-303](CC-303.md) oder [CC-304](CC-304.md)) |
| [CC-308 · Durchschlagskugel und legendäre Kanonen entwerfen](CC-308.md) | Design | [CC-307](CC-307.md) |
| [CC-309 · Freigegebene Durchschlagskugel und legendäre Angriffe umsetzen](CC-309.md) | Entwicklung | [CC-308](CC-308.md), [CC-305](CC-305.md) |
| [CC-310 · Schiffsformen und Spezialschüsse im Flottenvergleich balancieren](CC-310.md) | Spieltest | [CC-303](CC-303.md), [CC-304](CC-304.md), [CC-306](CC-306.md), [CC-307](CC-307.md) |
| [CC-311 · Schiffstypen und spezielle Formen in den Flottenkatalog aufnehmen](CC-311.md) | Inhalt und Entwicklung | [CC-811](CC-811.md), [CC-812](CC-812.md), [CC-813](CC-813.md), [CC-814](CC-814.md), [CC-815](CC-815.md), [CC-816](CC-816.md) |

## 4 · Wortduelle und Kapitäne

Grundlage ist das [Spielkonzept](../../Spielkonzept.md) in den Abschnitten 2.3, 3, 4 und 7. Der erste Wortduell-Ablauf nutzt einen festen Satz. [CC-412](CC-412.md) nimmt den vollständigen Ablauf mit Belohnungen im freien Solo-Match ab. Aussehen, Kapitänssammlung und ausrüstbare Karten folgen in [CC-408](CC-408.md), [CC-409](CC-409.md) und [CC-410](CC-410.md). [CC-411](CC-411.md) setzt die vier entworfenen Spezialisierungen um und prüft ihre Kombinationen mit Schiffen.

| Ticket | Typ | Vorgänger |
| --- | --- | --- |
| [CC-401 · Auslöser und Regeln für Wortduelle entscheiden](CC-401.md) | Design | [CC-206](CC-206.md) |
| [CC-402 · Einen festen Satz Piratenbeleidigungen und Konter schreiben](CC-402.md) | Inhalt | [CC-401](CC-401.md) |
| [CC-403 · Wortduell als spielbare Matchphase anbinden](CC-403.md) | Entwicklung | [CC-401](CC-401.md), [CC-402](CC-402.md) |
| [CC-404 · Regeln und Grenzen für Gefechtsbelohnungen festlegen](CC-404.md) | Design | [CC-401](CC-401.md) |
| [CC-405 · Bestätigte Gefechtsbelohnungen mit dem Raster verknüpfen](CC-405.md) | Entwicklung | [CC-403](CC-403.md), [CC-404](CC-404.md), [CC-709](CC-709.md) |
| [CC-406 · Captain-Fähigkeiten und Zuweisung zu Schiffen ergänzen](CC-406.md) | Entwicklung | [CC-306](CC-306.md), [CC-403](CC-403.md), [CC-704](CC-704.md) |
| [CC-407 · Kapitänsspezialisierungen und Duellkartendecks entwerfen](CC-407.md) | Design und Prototyp | [CC-405](CC-405.md), [CC-406](CC-406.md) |
| [CC-408 · Kapitänsfigur mit Aussehen und Kosmetik gestalten](CC-408.md) | Gestaltung und Entwicklung | [CC-704](CC-704.md), [CC-406](CC-406.md) |
| [CC-409 · Mehrere Kapitäne sammeln und einer Flotte zuweisen](CC-409.md) | Entwicklung | [CC-406](CC-406.md), [CC-407](CC-407.md) |
| [CC-410 · Sammelbare Beleidigungs- und Konterkarten ausrüsten](CC-410.md) | Entwicklung | [CC-407](CC-407.md) |
| [CC-411 · Vier Kapitänsspezialisierungen spielbar machen und Kombinationen prüfen](CC-411.md) | Entwicklung und Spieltest | [CC-409](CC-409.md), [CC-410](CC-410.md), [CC-208](CC-208.md) |
| [CC-412 · Freies Solo-Match mit allen Kampfphasen und Belohnungen abnehmen](CC-412.md) | Spieltest | [CC-405](CC-405.md), [CC-307](CC-307.md), [CC-306](CC-306.md) |

## 5 · Online-PvP und Kampagne

Grundlage ist das [Spielkonzept](../../Spielkonzept.md) in den Abschnitten 8, 10 und 13. [CC-501](CC-501.md) legt den ersten PvP-Umfang fest. [CC-504](CC-504.md) übernimmt Online-Arenen nach Freigabe. [CC-512](CC-512.md) bündelt den vollständigen PvP-Ablauf und [CC-513](CC-513.md) dessen Balance. Die Kampagne schließt ihren ersten vollständigen Kreislauf in [CC-510](CC-510.md) ab.

| Ticket | Typ | Vorgänger |
| --- | --- | --- |
| [CC-501 · Hosting und Regeln für Online-PvP festlegen](CC-501.md) | Technikentscheidung | [CC-206](CC-206.md) |
| [CC-502 · Privaten Matchraum mit Einladung bereitstellen](CC-502.md) | Entwicklung | [CC-501](CC-501.md), [CC-716](CC-716.md) |
| [CC-503 · Online-Rasteraktionen und getrennte Spielersichten umsetzen](CC-503.md) | Entwicklung | [CC-502](CC-502.md) |
| [CC-504 · Online-Gefechte synchronisieren](CC-504.md) | Entwicklung | [CC-206](CC-206.md), [CC-503](CC-503.md) |
| [CC-505 · Erste Kampagnenbegegnung mit einem KI-Kapitän bauen](CC-505.md) | Entwicklung | [CC-206](CC-206.md), [CC-406](CC-406.md), [CC-716](CC-716.md) |
| [CC-506 · Kampagnenfortschritt speichern und Inhalte freischalten](CC-506.md) | Entwicklung | [CC-505](CC-505.md) |
| [CC-507 · Inseln und einen ersten Bosskapitän ergänzen](CC-507.md) | Inhalt und Entwicklung | [CC-506](CC-506.md) |
| [CC-508 · Raster-, Arena- und Duell-KI taktisch ausbauen](CC-508.md) | Entwicklung | [CC-307](CC-307.md), [CC-205](CC-205.md), [CC-403](CC-403.md) |
| [CC-509 · Kampagnenbelohnungen mit Sammlung und Ausrüstung verbinden](CC-509.md) | Entwicklung | [CC-506](CC-506.md), [CC-408](CC-408.md), [CC-409](CC-409.md), [CC-410](CC-410.md), [CC-311](CC-311.md) |
| [CC-510 · Kampagne von der Erkundung bis zum Boss vollständig abnehmen](CC-510.md) | Spieltest | [CC-514](CC-514.md), [CC-509](CC-509.md), [CC-508](CC-508.md), [CC-405](CC-405.md) |
| [CC-511 · Kapitänserfahrung, Skilltrees und Verbesserungen umsetzen](CC-511.md) | Design und Entwicklung | [CC-506](CC-506.md), [CC-409](CC-409.md) |
| [CC-512 · Online-PvP mit individuellen Flotten und allen Kampfphasen abnehmen](CC-512.md) | Entwicklung und Spieltest | [CC-504](CC-504.md), [CC-306](CC-306.md), [CC-307](CC-307.md), [CC-405](CC-405.md), [CC-409](CC-409.md) |
| [CC-513 · Freischaltungen und Flottenkombinationen im PvP balancieren](CC-513.md) | Entwicklung und Spieltest | [CC-512](CC-512.md), [CC-501](CC-501.md) |
| [CC-514 · Orte und Regionen auf der Seekarte entdecken](CC-514.md) | Entwicklung | [CC-507](CC-507.md) |

## Gestaltung und Veröffentlichung

Grundlage ist das [Spielkonzept](../../Spielkonzept.md) in Abschnitt 9 und die [Hosting-Architektur](../architektur.md#hosting).

| Ticket | Typ | Vorgänger |
| --- | --- | --- |
| [CC-601 · Arena-Grafik und Gefechtsrückmeldung ausarbeiten](CC-601.md) | Gestaltung | [CC-206](CC-206.md), [CC-701](CC-701.md), [CC-712](CC-712.md) |
| [CC-602 · Spielbaren Stand privat auf ChatGPT Sites bereitstellen](CC-602.md) | Veröffentlichung | — |
| [CC-603 · Wortduelle mit Kapitänsporträts und Reaktionen inszenieren](CC-603.md) | Gestaltung und Entwicklung | [CC-403](CC-403.md), [CC-701](CC-701.md) |
| [CC-604 · Schiffe und Wasser mit Wellen, Segeln und Kielwasser animieren](CC-604.md) | Gestaltung und Entwicklung | [CC-601](CC-601.md) |
| [CC-605 · 2,5D und schräge Seekamera mit der Vogelperspektive vergleichen](CC-605.md) | Gestaltungsprototyp | [CC-601](CC-601.md), [CC-505](CC-505.md) |

## Abgleich mit dem Spielkonzept

| Abschnitte im Spielkonzept | Abdeckung im Backlog |
| --- | --- |
| 1–3 · Grundidee und Kampfphasen | Das Rastermatch ist umgesetzt. [CC-206](CC-206.md) prüft den Arena-Kreislauf. [CC-204](CC-204.md), [CC-403](CC-403.md) und [CC-405](CC-405.md) verbinden bestätigten Arena-Ausgang, Duellauslöser und Matchbelohnung. [CC-210](CC-210.md) integriert freigegebene Wind- und Manöverregeln. [CC-209](CC-209.md) erprobt Entern als optionalen Übergang. |
| 4 und 7 · Kapitäne und Karten | Charaktergestaltung, Besetzung und Duellkarten folgen auf den festen Duellsatz. [CC-411](CC-411.md) macht alle vier Spezialisierungen spielbar. Kampagnenrekrutierung und Kartenerwerb liegen in [CC-509](CC-509.md), Kapitänslevel und Skilltrees in [CC-511](CC-511.md). |
| 5–6 · Schiffe, Flotten und Kanonen | [CC-305](CC-305.md), [CC-306](CC-306.md) und [CC-311](CC-311.md) decken Katalog, Formen und Deckbau ab. [CC-301](CC-301.md) bis [CC-310](CC-310.md) führen vom ersten Spezialschuss zum späteren Flottenvergleich. [CC-208](CC-208.md) ergänzt Arenaprofile. |
| 8 und 10 · Spielmodi und Fortschritt | [CC-716](CC-716.md) verbindet die verfügbaren Modi mit dem Hauptmenü. [CC-508](CC-508.md) baut die Solo-KI aus. [CC-514](CC-514.md) setzt Entdeckungen und ihre bestätigten Folgen um. [CC-510](CC-510.md) prüft die Kampagne bis zum Boss und den Einsatz verdienter Belohnungen. [CC-512](CC-512.md) und [CC-513](CC-513.md) decken vollständiges PvP und kompetitive Freischaltungen ab. |
| 9 · Präsentation | Die UI-Tickets decken Seekartenoptik, Lesbarkeit und Rasterrückmeldung ab. [CC-601](CC-601.md) und [CC-603](CC-603.md) bis [CC-605](CC-605.md) decken Arena, Duell, Wasser und Kamera ab. |
| 11–13 · Balancing, Phasen und offene Entscheidungen | Maßgeblich ist die aktuelle [Roadmap](../roadmap.md). [CC-310](CC-310.md) bewertet Raster und Flottengröße, [CC-401](CC-401.md) und [CC-404](CC-404.md) Auslöser und Belohnungen. [CC-409](CC-409.md) klärt Kapitänswechsel, [CC-410](CC-410.md) den Kartenumfang und [CC-207](CC-207.md) Energiekosten. Gefechtsdauer, PvP-Fairness und Darstellung werden in [CC-206](CC-206.md), [CC-513](CC-513.md) und [CC-605](CC-605.md) geprüft. Die Rolle des Verteidigers steht bereits in den [Spielregeln](../spielregeln.md#schiffskampf). |
| 14 · Zusammenhängender Spielablauf | [CC-412](CC-412.md) prüft alle Kampfphasen und deren Dauer im freien Solo-Match. [CC-510](CC-510.md) und [CC-512](CC-512.md) prüfen den Zusammenhang in Kampagne und PvP. |
