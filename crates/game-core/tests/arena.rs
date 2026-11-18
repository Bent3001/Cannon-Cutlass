use game_core::Player::{First, Second};
use game_core::arena::{
    Arena, HEIGHT, LARGE_SHIP, SMALL_SHIP, STEPS_PER_SECOND, ShipInput, Steering, Throttle, WIDTH,
};

#[test]
fn ships_spawn_apart_and_can_be_controlled_independently() {
    let mut arena = Arena::new([SMALL_SHIP, LARGE_SHIP]);
    let first = arena.ship(First);
    let second = arena.ship(Second);
    assert_ne!((first.x, first.y), (second.x, second.y));
    for ship in [first, second] {
        assert!(ship.x > 0.0 && ship.x < WIDTH);
        assert!(ship.y > 0.0 && ship.y < HEIGHT);
        assert_eq!(ship.speed, 0.0);
    }
    assert_ne!(first.heading, second.heading);

    arena.step([
        ShipInput {
            throttle: Throttle::Accelerate,
            steering: Steering::Right,
        },
        ShipInput::default(),
    ]);
    let moved = arena.ship(First);
    let expected_speed = SMALL_SHIP.acceleration / STEPS_PER_SECOND as f32;
    assert!((moved.speed - expected_speed).abs() < 0.0001);
    assert!(moved.x > first.x && moved.y > first.y);
    assert!(moved.heading > first.heading);
    assert_eq!(arena.ship(Second), second);
}

#[test]
fn identical_fixed_steps_replay_both_trajectories() {
    let profiles = [SMALL_SHIP, LARGE_SHIP];
    let mut original = Arena::new(profiles);
    let mut replay = Arena::new(profiles);
    let mut inputs = [ShipInput::default(); 2];
    for (throttle, steering) in [
        (Throttle::Accelerate, Steering::Left),
        (Throttle::Coast, Steering::Straight),
        (Throttle::Brake, Steering::Right),
    ] {
        inputs.rotate_left(1);
        inputs[0] = ShipInput { throttle, steering };
        for _ in 0..STEPS_PER_SECOND {
            original.step(inputs);
            replay.step(inputs);
            assert_eq!(original.ship(First), replay.ship(First));
            assert_eq!(original.ship(Second), replay.ship(Second));
        }
    }
}

#[test]
fn speed_and_arena_bounds_hold_while_accelerating_and_braking() {
    let mut arena = Arena::new([SMALL_SHIP, LARGE_SHIP]);
    let accelerate = ShipInput {
        throttle: Throttle::Accelerate,
        ..ShipInput::default()
    };
    for _ in 0..10 * STEPS_PER_SECOND {
        arena.step([accelerate; 2]);
        assert!((0.0..=SMALL_SHIP.max_speed).contains(&arena.ship(First).speed));
        assert!((0.0..=LARGE_SHIP.max_speed).contains(&arena.ship(Second).speed));
    }
    assert_eq!(arena.ship(First).x, WIDTH);
    assert_eq!(arena.ship(Second).x, 0.0);
    assert_eq!(arena.ship(First).speed, SMALL_SHIP.max_speed);
    assert_eq!(arena.ship(Second).speed, LARGE_SHIP.max_speed);

    let turn_right = ShipInput {
        steering: Steering::Right,
        ..ShipInput::default()
    };
    for _ in 0..STEPS_PER_SECOND {
        arena.step([turn_right; 2]);
    }
    for _ in 0..10 * STEPS_PER_SECOND {
        arena.step([ShipInput::default(); 2]);
    }
    assert_eq!(arena.ship(First).y, HEIGHT);
    assert_eq!(arena.ship(Second).y, 0.0);

    let brake = ShipInput {
        throttle: Throttle::Brake,
        ..ShipInput::default()
    };
    arena.step([brake; 2]);
    let expected_speed = SMALL_SHIP.max_speed - SMALL_SHIP.braking / STEPS_PER_SECOND as f32;
    assert!((arena.ship(First).speed - expected_speed).abs() < 0.0001);
    for _ in 0..2 * STEPS_PER_SECOND {
        arena.step([brake; 2]);
    }
    assert_eq!(arena.ship(First).speed, 0.0);
    assert_eq!(arena.ship(Second).speed, 0.0);
}

#[test]
fn small_ship_turns_faster_than_large_ship() {
    let mut arena = Arena::new([SMALL_SHIP, LARGE_SHIP]);
    let first_heading = arena.ship(First).heading;
    let second_heading = arena.ship(Second).heading;
    let steer = ShipInput {
        steering: Steering::Right,
        ..ShipInput::default()
    };
    for _ in 0..STEPS_PER_SECOND {
        arena.step([steer; 2]);
    }
    let first_turn = arena.ship(First).heading - first_heading;
    let second_turn = arena.ship(Second).heading - second_heading;
    assert!((first_turn - SMALL_SHIP.turn_rate).abs() < 0.0001);
    assert!((second_turn - LARGE_SHIP.turn_rate).abs() < 0.0001);
    assert!(first_turn > second_turn);
}
