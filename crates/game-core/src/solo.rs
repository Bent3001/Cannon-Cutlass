use crate::{
    Cell, Coord, FLEET_SIZE, Match, Orientation, Phase, Player, PlayerView, RuleError, Shot,
    board::Board,
};

pub struct SoloMatch {
    game: Match,
    seed: u32,
}

impl SoloMatch {
    pub fn new(seed: u32) -> Self {
        Self::from_match(seed, Match::default())
    }

    pub fn with_encounters(seed: u32) -> Self {
        Self::from_match(seed, Match::with_encounters())
    }

    fn from_match(seed: u32, game: Match) -> Self {
        let mut solo = Self { game, seed };
        solo.game.boards[Player::Second as usize] = solo.random_fleet();
        solo
    }

    pub fn view(&self) -> PlayerView {
        self.game.view(Player::First)
    }

    pub fn place_ship(
        &mut self,
        ship: usize,
        start: Coord,
        orientation: Orientation,
    ) -> Result<(), RuleError> {
        self.game
            .place_ship(Player::First, ship, start, orientation)
    }

    pub fn auto_place(&mut self) -> Result<(), RuleError> {
        if self.game.phase() != Phase::Placement {
            return Err(RuleError::WrongPhase);
        }
        self.game.boards[Player::First as usize] = self.random_fleet();
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), RuleError> {
        self.game.start()
    }

    pub fn fire(&mut self, target: Coord) -> Result<Vec<Shot>, RuleError> {
        let player_shot = self.game.fire(Player::First, target)?;
        let mut shots = vec![player_shot];
        shots.extend(self.reply_if_due());
        Ok(shots)
    }

    pub fn finish_encounter(&mut self, id: u32) -> Result<Option<Shot>, RuleError> {
        self.game.finish_encounter(id)?;
        Ok(self.reply_if_due())
    }

    fn reply_if_due(&mut self) -> Option<Shot> {
        if self.game.phase() != Phase::Turn(Player::Second) {
            return None;
        }

        let options: Vec<_> = self.game.boards[Player::First as usize]
            .cells(false)
            .into_iter()
            .enumerate()
            .filter_map(|(index, cell)| (cell == Cell::Water).then_some(Coord::from_index(index)))
            .collect();
        let target = options[self.draw(options.len())];
        let opponent_shot = self
            .game
            .fire(Player::Second, target)
            .expect("the computer selects an untargeted cell on its own turn");
        Some(opponent_shot)
    }

    fn random_fleet(&mut self) -> Board {
        let mut board = Board::default();
        for ship in 0..FLEET_SIZE {
            let placements: Vec<_> = Coord::all()
                .flat_map(|coord| {
                    [Orientation::Horizontal, Orientation::Vertical]
                        .map(|orientation| (coord, orientation))
                })
                .filter(|&(coord, orientation)| board.footprint(ship, coord, orientation).is_ok())
                .collect();
            // The five standard ships, placed longest first, always leave room on a 10×10 board.
            let (coord, orientation) = placements[self.draw(placements.len())];
            board
                .place(ship, coord, orientation)
                .expect("validated fleet placement");
        }
        board
    }

    fn draw(&mut self, choices: usize) -> usize {
        // A seeded generator keeps runs reproducible and avoids browser APIs in the core.
        self.seed = self
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        ((u64::from(self.seed) * choices as u64) >> 32) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ShotOutcome;

    #[test]
    fn a_deferred_ai_reply_can_open_another_encounter() {
        let mut solo = SoloMatch::with_encounters(0);
        solo.game.boards[Player::Second as usize] = Board::default();
        // With this seed the AI draws index 23 from the 99 untargeted cells.
        solo.seed = 0;
        let ai_target = Coord::new(3, 2).unwrap();
        for (ship, (x, y)) in [(0, 0), (0, 1), (0, 3), (0, 4), (3, 2)]
            .into_iter()
            .enumerate()
        {
            solo.place_ship(ship, Coord::new(x, y).unwrap(), Orientation::Horizontal)
                .unwrap();
            solo.game
                .place_ship(
                    Player::Second,
                    ship,
                    Coord::new(0, ship).unwrap(),
                    Orientation::Horizontal,
                )
                .unwrap();
        }
        solo.start().unwrap();
        for x in 0..5 {
            solo.game.boards[Player::Second as usize]
                .fire(Coord::new(x, 0).unwrap())
                .unwrap();
        }
        for (player, x, y) in [(Player::First, 4, 2), (Player::Second, 0, 4)] {
            assert_eq!(
                solo.game.boards[player as usize]
                    .fire(Coord::new(x, y).unwrap())
                    .unwrap(),
                ShotOutcome::Hit
            );
        }

        let shots = solo.fire(Coord::new(1, 4).unwrap()).unwrap();
        assert_eq!(shots.len(), 1);
        assert_eq!(shots[0].outcome, ShotOutcome::Sunk { ship: 4 });
        let first = solo.view().encounter.unwrap();
        assert_eq!(first.attacker, Player::First);
        assert_eq!((solo.view().skulls, solo.view().enemy_skulls), (1, 0));

        let reply = solo.finish_encounter(first.id).unwrap().unwrap();
        assert_eq!(reply.player, Player::Second);
        assert_eq!(reply.target, ai_target);
        assert_eq!(reply.outcome, ShotOutcome::Sunk { ship: 4 });
        let second = solo.view().encounter.unwrap();
        assert_ne!(first.id, second.id);
        assert_eq!(second.attacker, Player::Second);
        assert_eq!(second.attacker_ship, 1);
        assert_eq!((solo.view().skulls, solo.view().enemy_skulls), (1, 1));

        assert_eq!(
            solo.finish_encounter(first.id),
            Err(RuleError::WrongEncounter)
        );
        assert_eq!(solo.view().encounter, Some(second));
        assert_eq!((solo.view().skulls, solo.view().enemy_skulls), (1, 1));

        assert_eq!(solo.finish_encounter(second.id), Ok(None));
        let resumed = solo.view();
        assert_eq!(resumed.state, Phase::Turn(Player::First));
        assert_eq!((resumed.skulls, resumed.enemy_skulls), (2, 1));
        assert_eq!((resumed.own_remaining, resumed.enemy_remaining), (4, 3));
        assert_eq!(
            solo.finish_encounter(second.id),
            Err(RuleError::WrongEncounter)
        );
    }
}
