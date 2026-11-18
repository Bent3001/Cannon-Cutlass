//! Zugbasierte Regeln und Spielersichten für Cannon & Cutlass.

mod board;
mod fleet;
mod solo;

use board::Board;
use serde::Serialize;

pub use board::{Cell, Coord, Orientation, ShotOutcome};
pub use fleet::{
    STANDARD_FLEET_IDS, STANDARD_FLEET_LENGTHS, TEST_BOAT_ARENA_PROFILE, TestBoatArenaProfile,
};
pub use solo::SoloMatch;

pub const STANDARD_BOARD_SIZE: usize = 10;
pub const SKULLS_PER_TURN: u8 = 1;
pub const MAX_SKULLS: u8 = 8;
pub const STANDARD_SHOT_COST: u8 = 0;
const CELL_COUNT: usize = STANDARD_BOARD_SIZE * STANDARD_BOARD_SIZE;
const FLEET_SIZE: usize = STANDARD_FLEET_LENGTHS.len();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleError {
    OutsideBoard,
    UnknownShip,
    Overlap,
    FleetIncomplete,
    AlreadyTargeted,
    WrongPhase,
    WrongTurn,
    WrongEncounter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Player {
    First,
    Second,
}

impl Player {
    fn opponent(self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "phase", content = "player", rename_all = "snake_case")]
pub enum Phase {
    Placement,
    Turn(Player),
    Encounter,
    Finished(Player),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Encounter {
    pub id: u32,
    pub attacker: Player,
    pub defender: Player,
    pub attacker_ship: usize,
    pub defender_ship: usize,
}

struct PendingEncounter {
    encounter: Encounter,
    next: Phase,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Shot {
    pub player: Player,
    pub target: Coord,
    #[serde(flatten)]
    pub outcome: ShotOutcome,
}

#[derive(Serialize)]
pub struct PlayerView {
    pub state: Phase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Encounter>,
    pub own: Vec<Cell>,
    pub enemy: Vec<Cell>,
    pub placed: [bool; FLEET_SIZE],
    pub skulls: u8,
    pub enemy_skulls: u8,
    pub max_skulls: u8,
    pub own_remaining: usize,
    pub enemy_remaining: usize,
}

pub struct Match {
    phase: Phase,
    pending_encounter: Option<PendingEncounter>,
    next_encounter_id: Option<u32>,
    boards: [Board; 2],
    skulls: [u8; 2],
}

impl Default for Match {
    fn default() -> Self {
        Self {
            phase: Phase::Placement,
            pending_encounter: None,
            next_encounter_id: None,
            boards: [Board::default(), Board::default()],
            skulls: [0; 2],
        }
    }
}

impl Match {
    pub fn with_encounters() -> Self {
        Self {
            next_encounter_id: Some(1),
            ..Self::default()
        }
    }

    fn phase(&self) -> Phase {
        if self.pending_encounter.is_some() {
            Phase::Encounter
        } else {
            self.phase
        }
    }

    pub fn place_ship(
        &mut self,
        player: Player,
        ship: usize,
        start: Coord,
        orientation: Orientation,
    ) -> Result<(), RuleError> {
        if self.phase() != Phase::Placement {
            return Err(RuleError::WrongPhase);
        }
        self.boards[player as usize].place(ship, start, orientation)
    }

    pub fn start(&mut self) -> Result<(), RuleError> {
        if self.phase() != Phase::Placement {
            return Err(RuleError::WrongPhase);
        }
        if self.boards.iter().any(|board| !board.ready()) {
            return Err(RuleError::FleetIncomplete);
        }
        self.begin_turn(Player::First);
        Ok(())
    }

    pub fn fire(&mut self, player: Player, target: Coord) -> Result<Shot, RuleError> {
        match self.phase() {
            Phase::Turn(active) if active == player => {}
            Phase::Turn(_) => return Err(RuleError::WrongTurn),
            _ => return Err(RuleError::WrongPhase),
        }

        let opponent = player.opponent();
        let board = &mut self.boards[opponent as usize];
        let outcome = board.fire(target)?;
        self.skulls[player as usize] -= STANDARD_SHOT_COST;

        let next = if board.remaining() == 0 {
            Phase::Finished(player)
        } else {
            Phase::Turn(opponent)
        };
        if let (Some(id), ShotOutcome::Sunk { ship }) = (self.next_encounter_id, &outcome) {
            let attacker_ship = self.boards[player as usize]
                .intact_ships()
                .next()
                .expect("a player taking a raster turn has an intact ship");
            self.pending_encounter = Some(PendingEncounter {
                encounter: Encounter {
                    id,
                    attacker: player,
                    defender: opponent,
                    attacker_ship,
                    defender_ship: *ship,
                },
                next,
            });
            self.next_encounter_id = Some(id + 1);
        } else {
            self.advance_to(next);
        }
        Ok(Shot {
            player,
            target,
            outcome,
        })
    }

    /// Called by the arena controller when the identified encounter has ended.
    pub fn finish_encounter(&mut self, id: u32) -> Result<(), RuleError> {
        let pending = self
            .pending_encounter
            .as_ref()
            .filter(|pending| pending.encounter.id == id)
            .ok_or(RuleError::WrongEncounter)?;
        let next = pending.next;
        self.pending_encounter = None;
        self.advance_to(next);
        Ok(())
    }

    pub fn view(&self, player: Player) -> PlayerView {
        let own = &self.boards[player as usize];
        let enemy = &self.boards[player.opponent() as usize];
        PlayerView {
            state: self.phase(),
            encounter: self
                .pending_encounter
                .as_ref()
                .map(|pending| pending.encounter),
            own: own.cells(true),
            enemy: enemy.cells(false),
            placed: own.placed(),
            skulls: self.skulls[player as usize],
            enemy_skulls: self.skulls[player.opponent() as usize],
            max_skulls: MAX_SKULLS,
            own_remaining: own.remaining(),
            enemy_remaining: enemy.remaining(),
        }
    }

    fn advance_to(&mut self, next: Phase) {
        if let Phase::Turn(player) = next {
            self.begin_turn(player);
        } else {
            self.phase = next;
        }
    }

    fn begin_turn(&mut self, player: Player) {
        self.credit_skulls(player, SKULLS_PER_TURN);
        self.phase = Phase::Turn(player);
    }

    // All skull grants, including future rewards, share the same cap.
    fn credit_skulls(&mut self, player: Player, amount: u8) {
        let balance = &mut self.skulls[player as usize];
        *balance = balance.saturating_add(amount).min(MAX_SKULLS);
    }
}
