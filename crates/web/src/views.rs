use game_core::{STANDARD_BOARD_SIZE, STANDARD_FLEET_LENGTHS, STANDARD_SHOT_COST};
use maud::{DOCTYPE, Markup, html};

pub fn home() -> Markup {
    html! {
        (DOCTYPE)
        html lang="de" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Cannon & Cutlass" }
                link rel="stylesheet" href="./assets/style.css";
                script type="module" src="./assets/app.js" {}
            }
            body {
                main {
                    header class="masthead" {
                        h1 { "Cannon & Cutlass" }
                        p { "Schiffe versenken gegen die KI" }
                    }
                    section class="commands" aria-label="Matchstatus" {
                        h2 id="phase" { "Match wird geladen" }
                        p id="message" role="status" { "Das Spiel wird geladen …" }
                        p id="reply" aria-live="polite" {}
                        noscript { p { "Aktiviere JavaScript, um die Partie im Browser zu spielen." } }
                    }
                    div id="match" hidden {
                        section id="placement" aria-labelledby="placement-title" {
                            h2 id="placement-title" { "Flotte aufstellen" }
                            p { "Wähle ein Schiff und setze es auf ein Startfeld. Es erstreckt sich nach rechts oder unten." }
                            div class="fleet" role="group" aria-label="Schiff auswählen" {
                                @for (index, length) in STANDARD_FLEET_LENGTHS.iter().enumerate() {
                                    button type="button" data-ship=(index) data-length=(length) aria-pressed="false" {
                                        "Schiff " (index + 1)
                                        span { (length) " Felder" }
                                        span class="placed" hidden { "Platziert" }
                                    }
                                }
                            }
                            div class="actions" {
                                button type="button" id="orientation" { "Ausrichtung: waagerecht" }
                                button type="button" id="auto" { "Flotte automatisch verteilen" }
                                button type="button" class="primary" id="start" disabled { "Match starten" }
                            }
                        }
                        section id="battle" aria-label="Spielstand" hidden {
                            dl class="score" {
                                div { dt { "Deine Totenschädel" } dd id="skulls" { "0" } }
                                div { dt { "Totenschädel der KI" } dd id="enemy-skulls" { "0" } }
                                div { dt { "Standardschuss" } dd { (STANDARD_SHOT_COST) " Totenschädel" } }
                            }
                            button type="button" id="restart" hidden { "Neues Match" }
                        }
                        div class="boards" {
                            (board("own", "Deine Flotte"))
                            (board("enemy", "Gegnerisches Gewässer"))
                        }
                        ul class="legend" aria-label="Zeichenerklärung" {
                            li { span aria-hidden="true" data-cell="ship" { "■" } "Schiff" }
                            li { span aria-hidden="true" data-cell="miss" { "·" } "Fehlschuss" }
                            li { span aria-hidden="true" data-cell="hit" { "×" } "Treffer" }
                            li { span aria-hidden="true" data-cell="sunk" { "⊗" } "Versenkt" }
                        }
                    }
                }
            }
        }
    }
}

fn board(id: &str, title: &str) -> Markup {
    html! {
        section class="board-panel" id=(format!("{id}-panel")) {
            div class="board-heading" {
                h2 id=(format!("{id}-title")) { (title) }
                p id=(format!("{id}-remaining")) {}
            }
            table id=(id) aria-labelledby=(format!("{id}-title")) data-size=(STANDARD_BOARD_SIZE) {
                thead {
                    tr {
                        td {}
                        @for x in 0..STANDARD_BOARD_SIZE {
                            th scope="col" { (column(x)) }
                        }
                    }
                }
                tbody {
                    @for y in 0..STANDARD_BOARD_SIZE {
                        tr {
                            th scope="row" { (y + 1) }
                            @for x in 0..STANDARD_BOARD_SIZE {
                                td {
                                    button type="button" data-x=(x) data-y=(y)
                                        aria-label=(format!("{}{}", column(x), y + 1))
                                        tabindex="-1" {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn column(index: usize) -> char {
    (b'A' + index as u8) as char
}
