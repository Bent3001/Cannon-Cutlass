use game_core::{
    Player,
    arena::{
        Arena, HEIGHT, LARGE_SHIP, SMALL_SHIP, STEPS_PER_SECOND, ShipInput, Steering, Throttle,
        WIDTH,
    },
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum ArenaThrottle {
    Coast,
    Accelerate,
    Brake,
}

impl From<ArenaThrottle> for Throttle {
    fn from(input: ArenaThrottle) -> Self {
        match input {
            ArenaThrottle::Coast => Self::Coast,
            ArenaThrottle::Accelerate => Self::Accelerate,
            ArenaThrottle::Brake => Self::Brake,
        }
    }
}

#[wasm_bindgen]
pub enum ArenaSteering {
    Straight,
    Left,
    Right,
}

impl From<ArenaSteering> for Steering {
    fn from(input: ArenaSteering) -> Self {
        match input {
            ArenaSteering::Straight => Self::Straight,
            ArenaSteering::Left => Self::Left,
            ArenaSteering::Right => Self::Right,
        }
    }
}

/// Independent movement prototype for the browser, with small and large test profiles.
#[wasm_bindgen]
pub struct BrowserArena(Arena);

#[wasm_bindgen]
impl BrowserArena {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Arena::new([SMALL_SHIP, LARGE_SHIP]))
    }

    /// Returns current ship positions, headings and speeds in player order.
    pub fn view(&self) -> String {
        serde_json::json!({
            "width": WIDTH,
            "height": HEIGHT,
            "steps_per_second": STEPS_PER_SECOND,
            "ships": [self.0.ship(Player::First), self.0.ship(Player::Second)],
        })
        .to_string()
    }

    /// Advances both ships by exactly one simulation step.
    pub fn step(
        &mut self,
        first_throttle: ArenaThrottle,
        first_steering: ArenaSteering,
        second_throttle: ArenaThrottle,
        second_steering: ArenaSteering,
    ) {
        self.0.step([
            ShipInput {
                throttle: first_throttle.into(),
                steering: first_steering.into(),
            },
            ShipInput {
                throttle: second_throttle.into(),
                steering: second_steering.into(),
            },
        ]);
    }
}
