# Mit Tickets arbeiten

Die einzelnen Aufgaben stehen im [Ticket-Backlog](docs/tickets/README.md). Die jeweilige Ticketdatei enthält den verbindlichen Status, die Abnahmekriterien und die Abhängigkeiten.

## Ein Ticket auswählen

1. Prüfe die [Roadmap](docs/roadmap.md), den aktuellen Projektstand und die bereits abgeschlossenen Tickets. Wenn der nächste Schritt ein Review ist, kann dieses Review selbst die passende nächste Aufgabe sein.
2. Wähle für die Umsetzung ein offenes Ticket, das zum aktuellen Meilenstein passt und dessen Voraussetzungen erfüllt sind. Ein Vorgängerticket zählt erst nach bestandenem Review als abgeschlossen. Achte bei bedingten Abhängigkeiten darauf, welche Variante tatsächlich gewählt wurde.
3. Bevorzuge bei mehreren passenden Tickets eine Aufgabe, die den aktuellen Meilenstein voranbringt oder andere Tickets freigibt. Die Ticketnummer dient der Zuordnung. Wenn eine Voraussetzung oder eine Designentscheidung fehlt, halte den Blocker im Ticket fest und wähle eine andere passende Aufgabe.
4. Setze das gewählte Ticket auf **In Arbeit**. Prüfe vor Beginn, ob jemand bereits daran arbeitet.

## Umsetzung zur Prüfung vorlegen

Arbeite die Abnahmekriterien ab und halte im Ticket fest, was umgesetzt und wie es geprüft wurde. Nenne relevante Dateien, Testbefehle oder Schritte zum Nachspielen. Bei Design- und Inhaltstickets verlinke die Entscheidung oder das Ergebnis. Offene Punkte gehören ebenfalls ins Ticket.

Setze den Status anschließend auf **Im Review**. Abgehakte Kriterien und erfolgreiche Tests sind Teil des Prüfnachweises. Der Abschluss folgt nach der Freigabe.

## Review und Abschluss

Der Projektinhaber oder eine KI übernimmt das Review getrennt von der Umsetzung. Wer das Ticket umgesetzt hat, erteilt sich keine eigene Freigabe. Die prüfende Person oder KI vergleicht das tatsächliche Ergebnis mit dem Ticketumfang und den Abnahmekriterien, prüft die relevanten Nachweise und wiederholt bei Bedarf einzelne Tests oder Spielschritte.

- **Freigegeben.** Trage im Ticket unter `## Review` Datum, prüfende Person oder KI und das Ergebnis ein. Setze den Status erst nach der ausdrücklichen Freigabe auf **Abgeschlossen**.
- **Änderungen erforderlich.** Halte die konkreten Befunde im Ticket fest und setze den Status zurück auf **In Arbeit**. Nach der Korrektur folgt ein erneutes Review.
- **Review ausstehend.** Das Ticket bleibt **Im Review**. Andere passende Tickets können in der Zwischenzeit bearbeitet werden.

## Statuswerte

| Status | Bedeutung |
| --- | --- |
| Offen | Die Aufgabe wartet auf Auswahl. |
| In Arbeit | Die Umsetzung oder eine Korrektur läuft. |
| Blockiert | Eine benannte Voraussetzung oder Entscheidung fehlt. |
| Im Review | Die Umsetzung liegt zur Prüfung vor. |
| Abgeschlossen | Der Projektinhaber oder eine KI hat das Ticket ausdrücklich freigegeben. |
