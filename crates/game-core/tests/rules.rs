use game_core::{
    Cell, Coord, MAX_SKULLS, Match, Orientation, Phase, Player, RuleError, STANDARD_FLEET_LENGTHS,
    ShotOutcome, SoloMatch,
};

fn coord(x: usize, y: usize) -> Coord {
    Coord::new(x, y).unwrap()
}

fn ready_match() -> Match {
    place_standard_fleets(Match::default())
}

fn place_standard_fleets(mut game: Match) -> Match {
    for player in [Player::First, Player::Second] {
        for ship in 0..STANDARD_FLEET_LENGTHS.len() {
            game.place_ship(player, ship, coord(0, ship), Orientation::Horizontal)
                .unwrap();
        }
    }
    game.start().unwrap();
    game
}

#[test]
fn encounter_mode_defers_the_final_win_and_does_not_credit_an_extra_turn() {
    for winner in [Player::First, Player::Second] {
        let mut game = place_standard_fleets(Match::with_encounters());
        let loser = if winner == Player::First {
            Player::Second
        } else {
            Player::First
        };
        let mut misses = 0;
        for (y, length) in STANDARD_FLEET_LENGTHS.into_iter().enumerate() {
            for x in 0..length {
                if game.view(winner).state == Phase::Turn(loser) {
                    game.fire(loser, coord(misses % 10, 5 + misses / 10))
                        .unwrap();
                    misses += 1;
                }
                let before = game.view(winner);
                game.fire(winner, coord(x, y)).unwrap();
                if x + 1 < length {
                    assert_eq!(game.view(winner).state, Phase::Turn(loser));
                    continue;
                }

                let paused = game.view(winner);
                let encounter = paused.encounter.unwrap();
                assert_eq!(paused.state, Phase::Encounter);
                assert_eq!((encounter.attacker, encounter.defender), (winner, loser));
                assert_eq!((encounter.attacker_ship, encounter.defender_ship), (0, y));
                assert!(paused.enemy[y * 10 + x] == Cell::Sunk);
                assert_eq!(paused.enemy_skulls, before.enemy_skulls);
                assert_eq!(game.fire(loser, coord(9, 8)), Err(RuleError::WrongPhase));
                assert_eq!(game.view(winner).enemy_skulls, paused.enemy_skulls);
                game.finish_encounter(encounter.id).unwrap();
                let resumed = game.view(winner);
                let final_ship = y + 1 == STANDARD_FLEET_LENGTHS.len();
                assert_eq!(
                    resumed.state,
                    if final_ship {
                        Phase::Finished(winner)
                    } else {
                        Phase::Turn(loser)
                    }
                );
                assert_eq!(
                    resumed.enemy_skulls,
                    (before.enemy_skulls + u8::from(!final_ship)).min(MAX_SKULLS)
                );
                assert_eq!(resumed.skulls, before.skulls);
                assert!(resumed.enemy[y * 10 + x] == Cell::Sunk);
                assert_eq!(
                    game.finish_encounter(encounter.id),
                    Err(RuleError::WrongEncounter)
                );
            }
        }
    }
}

#[test]
fn invalid_placement_preserves_the_fleet_and_start_requires_both_fleets() {
    let mut game = Match::default();
    game.place_ship(Player::First, 0, coord(0, 0), Orientation::Horizontal)
        .unwrap();
    assert_eq!(
        game.place_ship(Player::First, 0, coord(0, 5), Orientation::Vertical),
        Err(RuleError::OutsideBoard)
    );
    assert_eq!(
        game.place_ship(Player::First, 1, coord(4, 0), Orientation::Vertical),
        Err(RuleError::Overlap)
    );
    assert_eq!(
        game.place_ship(Player::First, 5, coord(0, 0), Orientation::Horizontal),
        Err(RuleError::UnknownShip)
    );
    assert_eq!(game.start(), Err(RuleError::FleetIncomplete));
    let view = game.view(Player::First);
    assert_eq!(view.placed, [true, false, false, false, false]);
    assert_eq!(
        view.own.iter().filter(|&&cell| cell == Cell::Ship).count(),
        6
    );
}

#[test]
fn every_test_boat_fits_exactly_at_both_board_edges() {
    assert_eq!(STANDARD_FLEET_LENGTHS, [6, 5, 4, 3, 2]);
    for (ship, length) in STANDARD_FLEET_LENGTHS.into_iter().enumerate() {
        let edge = 10 - length;
        for (orientation, valid, outside) in [
            (Orientation::Horizontal, coord(edge, 9), coord(edge + 1, 9)),
            (Orientation::Vertical, coord(9, edge), coord(9, edge + 1)),
        ] {
            let mut game = Match::default();
            game.place_ship(Player::First, ship, valid, orientation)
                .unwrap();
            assert_eq!(
                game.place_ship(Player::First, ship, outside, orientation),
                Err(RuleError::OutsideBoard)
            );
            assert_eq!(
                game.view(Player::First)
                    .own
                    .iter()
                    .filter(|&&cell| cell == Cell::Ship)
                    .count(),
                length
            );
        }
    }
}

#[test]
fn shots_keep_the_enemy_hidden_and_invalid_shots_preserve_the_turn() {
    let mut game = ready_match();
    let view = game.view(Player::First);
    assert_eq!((view.skulls, view.enemy_skulls), (1, 0));
    assert!(view.enemy.iter().all(|&cell| cell == Cell::Water));
    assert_eq!(
        game.fire(Player::Second, coord(0, 4)),
        Err(RuleError::WrongTurn)
    );
    assert_eq!(
        game.fire(Player::First, coord(0, 4)).unwrap().outcome,
        ShotOutcome::Hit
    );
    assert_eq!(game.view(Player::First).state, Phase::Turn(Player::Second));
    game.fire(Player::Second, coord(9, 9)).unwrap();
    assert_eq!(
        game.fire(Player::First, coord(0, 4)),
        Err(RuleError::AlreadyTargeted)
    );
    let view = game.view(Player::First);
    assert_eq!(view.state, Phase::Turn(Player::First));
    assert_eq!((view.skulls, view.enemy_skulls), (2, 1));
    assert!(view.enemy[40] == Cell::Hit && view.enemy[41] == Cell::Water);
    assert_eq!(
        game.fire(Player::First, coord(1, 4)).unwrap().outcome,
        ShotOutcome::Sunk { ship: 4 }
    );
    let view = game.view(Player::First);
    assert!(view.enemy[40] == Cell::Sunk && view.enemy[41] == Cell::Sunk);
    assert_eq!(view.enemy_remaining, 4);
}

#[test]
fn the_final_sinking_finishes_the_match_without_an_extra_turn() {
    let mut game = ready_match();
    let mut replies = 0;
    for (y, length) in STANDARD_FLEET_LENGTHS.into_iter().enumerate() {
        for x in 0..length {
            game.fire(Player::First, coord(x, y)).unwrap();
            if game.view(Player::First).state == Phase::Turn(Player::Second) {
                game.fire(Player::Second, coord(replies % 10, 5 + replies / 10))
                    .unwrap();
                replies += 1;
            }
        }
    }
    let view = game.view(Player::First);
    assert_eq!(view.state, Phase::Finished(Player::First));
    assert_eq!(view.enemy_remaining, 0);
    assert_eq!((view.skulls, view.enemy_skulls), (8, 8));
    assert_eq!(
        game.fire(Player::Second, coord(9, 8)),
        Err(RuleError::WrongPhase)
    );
    let after = game.view(Player::First);
    assert_eq!((after.skulls, after.enemy_skulls), (8, 8));
}

#[test]
fn skull_credit_caps_each_player_and_standard_shots_are_free() {
    let mut game = ready_match();
    for index in 0..10 {
        let first = game.view(Player::First);
        assert_eq!(first.max_skulls, MAX_SKULLS);
        assert_eq!(first.skulls, (index as u8 + 1).min(8));
        assert_eq!(first.enemy_skulls, (index as u8).min(8));

        if index == 6 {
            assert_eq!(
                game.fire(Player::First, coord(0, 9)),
                Err(RuleError::AlreadyTargeted)
            );
            let after = game.view(Player::First);
            assert_eq!(after.state, first.state);
            assert_eq!((after.skulls, after.enemy_skulls), (7, 6));
        }

        game.fire(Player::First, coord(index, 9)).unwrap();
        let second = game.view(Player::Second);
        assert_eq!(second.max_skulls, MAX_SKULLS);
        assert_eq!(
            (second.skulls, second.enemy_skulls),
            (first.skulls, first.skulls)
        );
        game.fire(Player::Second, coord(index, 9)).unwrap();
        assert_eq!(game.view(Player::Second).skulls, second.skulls);
    }
}

#[test]
fn solo_match_auto_places_five_ships_and_the_ai_replies_at_most_once_per_shot() {
    for seed in [0, 1, 42, u32::MAX] {
        let mut game = SoloMatch::new(seed);
        game.auto_place().unwrap();
        let mut replay = SoloMatch::new(seed);
        replay.auto_place().unwrap();
        assert!(game.view().own == replay.view().own);
        assert_eq!(game.view().placed, [true; 5]);
        assert_eq!(
            game.view()
                .own
                .iter()
                .filter(|&&cell| cell == Cell::Ship)
                .count(),
            20
        );
        game.start().unwrap();
        let mut ai_targets = Vec::new();
        for index in 0..100 {
            let shots = game.fire(coord(index % 10, index / 10)).unwrap();
            assert_eq!(shots[0].player, Player::First);
            if let [_, response] = shots.as_slice() {
                assert_eq!(response.player, Player::Second);
                assert!(!ai_targets.contains(&response.target));
                ai_targets.push(response.target);
            } else {
                assert_eq!(game.view().state, Phase::Finished(Player::First));
            }
            if matches!(game.view().state, Phase::Finished(_)) {
                break;
            }
        }
        assert!(matches!(game.view().state, Phase::Finished(_)));
    }
}
