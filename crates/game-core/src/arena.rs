//! Fixed-step movement for the two ships in an encounter.

use std::f32::consts::{PI, TAU};

use serde::Serialize;

use crate::Player;

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 450.0;
pub const STEPS_PER_SECOND: u32 = 60;
const STEP_SECONDS: f32 = 1.0 / STEPS_PER_SECOND as f32;

/// Linear values use arena units and seconds. Turn rate is in radians per second.
#[derive(Clone, Copy, Debug)]
pub struct MovementProfile {
    pub acceleration: f32,
    pub braking: f32,
    pub max_speed: f32,
    pub turn_rate: f32,
}

pub const SMALL_SHIP: MovementProfile = MovementProfile {
    acceleration: 90.0,
    braking: 140.0,
    max_speed: 140.0,
    turn_rate: 2.8,
};

pub const LARGE_SHIP: MovementProfile = MovementProfile {
    acceleration: 55.0,
    braking: 100.0,
    max_speed: 100.0,
    turn_rate: 1.4,
};

#[derive(Clone, Copy, Debug, Default)]
pub enum Throttle {
    #[default]
    Coast,
    Accelerate,
    Brake,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Steering {
    #[default]
    Straight,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShipInput {
    pub throttle: Throttle,
    pub steering: Steering,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct ShipState {
    pub x: f32,
    pub y: f32,
    /// Radians in [0, 2π), with zero pointing right and positive angles turning down.
    pub heading: f32,
    pub speed: f32,
}

struct Ship {
    profile: MovementProfile,
    state: ShipState,
}

impl Ship {
    fn step(&mut self, input: ShipInput) {
        let speed_change = match input.throttle {
            Throttle::Coast => 0.0,
            Throttle::Accelerate => self.profile.acceleration,
            Throttle::Brake => -self.profile.braking,
        };
        let turn_rate = match input.steering {
            Steering::Straight => 0.0,
            Steering::Left => -self.profile.turn_rate,
            Steering::Right => self.profile.turn_rate,
        };
        let state = &mut self.state;
        state.speed =
            (state.speed + speed_change * STEP_SECONDS).clamp(0.0, self.profile.max_speed);
        state.heading = (state.heading + turn_rate * STEP_SECONDS).rem_euclid(TAU);
        // Rounding in rem_euclid can produce TAU for a tiny negative angle.
        if state.heading == TAU {
            state.heading = 0.0;
        }

        let distance = state.speed * STEP_SECONDS;
        let (dy, dx) = state.heading.sin_cos();
        state.x = (state.x + dx * distance).clamp(0.0, WIDTH);
        state.y = (state.y + dy * distance).clamp(0.0, HEIGHT);
    }
}

pub struct Arena {
    ships: [Ship; 2],
}

impl Arena {
    /// Profiles are in player order, First then Second.
    ///
    /// Panics if any profile value is negative or non-finite.
    pub fn new(profiles: [MovementProfile; 2]) -> Self {
        let ships = std::array::from_fn(|index| {
            let profile = profiles[index];
            assert!(
                [
                    profile.acceleration,
                    profile.braking,
                    profile.max_speed,
                    profile.turn_rate
                ]
                .into_iter()
                .all(|value| value.is_finite() && value >= 0.0),
                "arena movement values must be finite and non-negative"
            );
            let (x, heading) = [(WIDTH * 0.25, 0.0), (WIDTH * 0.75, PI)][index];
            Ship {
                profile,
                state: ShipState {
                    x,
                    y: HEIGHT * 0.5,
                    heading,
                    speed: 0.0,
                },
            }
        });
        Self { ships }
    }

    /// Advance both ships by one fixed step. Inputs are in player order.
    pub fn step(&mut self, inputs: [ShipInput; 2]) {
        for (ship, input) in self.ships.iter_mut().zip(inputs) {
            ship.step(input);
        }
    }

    pub fn ship(&self, player: Player) -> ShipState {
        self.ships[player as usize].state
    }
}
