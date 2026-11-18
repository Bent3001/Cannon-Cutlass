/// Persistent deck identifiers in the same order as the raster slots.
/// These literals keep decks saved by the original prototype readable.
pub const STANDARD_FLEET_IDS: [&str; 5] = [
    "standard-0",
    "standard-1",
    "standard-2",
    "standard-3",
    "standard-4",
];

/// Test boats 6 through 2, longest first for automatic placement.
pub const STANDARD_FLEET_LENGTHS: [usize; STANDARD_FLEET_IDS.len()] = [6, 5, 4, 3, 2];

#[derive(Clone, Copy, Debug)]
pub struct TestBoatArenaProfile {
    pub acceleration: f32,
    pub braking: f32,
    pub max_speed: f32,
    pub turn_rate: f32,
}

/// Shared movement preset for all five test boats, in arena units and seconds.
/// Turn rate is in radians per second.
pub const TEST_BOAT_ARENA_PROFILE: TestBoatArenaProfile = TestBoatArenaProfile {
    acceleration: 70.0,
    braking: 120.0,
    max_speed: 120.0,
    turn_rate: 2.1,
};
