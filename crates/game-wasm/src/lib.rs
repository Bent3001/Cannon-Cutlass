mod arena;

use game_core::{Coord, Orientation, RuleError, SoloMatch};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BrowserMatch(SoloMatch);

#[wasm_bindgen]
impl BrowserMatch {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Self {
        Self(SoloMatch::new(seed))
    }

    pub fn view(&self) -> String {
        serde_json::to_string(&self.0.view()).expect("match view is serializable")
    }

    pub fn place(
        &mut self,
        ship: usize,
        x: usize,
        y: usize,
        vertical: bool,
    ) -> Result<(), JsError> {
        let orientation = if vertical {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        };
        let start = Coord::new(x, y).map_err(rule_error)?;
        self.0
            .place_ship(ship, start, orientation)
            .map_err(rule_error)
    }

    pub fn auto_place(&mut self) -> Result<(), JsError> {
        self.0.auto_place().map_err(rule_error)
    }

    pub fn start(&mut self) -> Result<(), JsError> {
        self.0.start().map_err(rule_error)
    }

    pub fn fire(&mut self, x: usize, y: usize) -> Result<String, JsError> {
        let target = Coord::new(x, y).map_err(rule_error)?;
        let shots = self.0.fire(target).map_err(rule_error)?;
        Ok(serde_json::to_string(&shots).expect("shots are serializable"))
    }
}

fn rule_error(error: RuleError) -> JsError {
    JsError::new(match error {
        RuleError::OutsideBoard => "Das Schiff passt an dieser Stelle nicht ins Raster.",
        RuleError::UnknownShip => "Dieses Schiff gehört nicht zur Flotte.",
        RuleError::Overlap => "An dieser Stelle liegt bereits ein anderes Schiff.",
        RuleError::FleetIncomplete => "Platziere alle fünf Schiffe, um das Match zu starten.",
        RuleError::AlreadyTargeted => "Auf dieses Feld wurde bereits geschossen.",
        RuleError::WrongPhase => "Diese Aktion ist in der aktuellen Spielphase nicht verfügbar.",
        RuleError::WrongTurn => "Der andere Spieler ist am Zug.",
        RuleError::WrongEncounter => "Dieses Seegefecht ist nicht aktiv.",
    })
}
