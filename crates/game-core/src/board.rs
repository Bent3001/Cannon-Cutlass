use serde::Serialize;

use crate::{CELL_COUNT, FLEET_SIZE, RuleError, STANDARD_BOARD_SIZE, STANDARD_FLEET_LENGTHS};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Result<Self, RuleError> {
        if x >= STANDARD_BOARD_SIZE || y >= STANDARD_BOARD_SIZE {
            return Err(RuleError::OutsideBoard);
        }
        Ok(Self { x, y })
    }

    pub(super) fn all() -> impl Iterator<Item = Self> {
        (0..CELL_COUNT).map(Self::from_index)
    }

    pub(super) fn from_index(index: usize) -> Self {
        Self {
            x: index % STANDARD_BOARD_SIZE,
            y: index / STANDARD_BOARD_SIZE,
        }
    }

    fn index(self) -> usize {
        self.y * STANDARD_BOARD_SIZE + self.x
    }
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Cell {
    Water,
    Ship,
    Miss,
    Hit,
    Sunk,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ShotOutcome {
    Miss,
    Hit,
    Sunk { ship: usize },
}

pub(super) struct Board {
    ships: [Option<Vec<Coord>>; FLEET_SIZE],
    shots: [bool; CELL_COUNT],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            ships: Default::default(),
            shots: [false; CELL_COUNT],
        }
    }
}

impl Board {
    pub fn place(
        &mut self,
        ship: usize,
        start: Coord,
        orientation: Orientation,
    ) -> Result<(), RuleError> {
        let fields = self.footprint(ship, start, orientation)?;
        self.ships[ship] = Some(fields);
        Ok(())
    }

    pub(super) fn footprint(
        &self,
        ship: usize,
        start: Coord,
        orientation: Orientation,
    ) -> Result<Vec<Coord>, RuleError> {
        let length = *STANDARD_FLEET_LENGTHS
            .get(ship)
            .ok_or(RuleError::UnknownShip)?;
        let fields = (0..length)
            .map(|offset| match orientation {
                Orientation::Horizontal => Coord::new(start.x + offset, start.y),
                Orientation::Vertical => Coord::new(start.x, start.y + offset),
            })
            .collect::<Result<Vec<_>, _>>()?;

        if fields
            .iter()
            .any(|&field| self.ship_at(field).is_some_and(|occupied| occupied != ship))
        {
            return Err(RuleError::Overlap);
        }
        Ok(fields)
    }

    pub fn ready(&self) -> bool {
        self.ships.iter().all(Option::is_some)
    }

    pub fn placed(&self) -> [bool; FLEET_SIZE] {
        std::array::from_fn(|ship| self.ships[ship].is_some())
    }

    pub fn remaining(&self) -> usize {
        self.intact_ships().count()
    }

    pub(super) fn intact_ships(&self) -> impl Iterator<Item = usize> + '_ {
        self.ships
            .iter()
            .enumerate()
            .filter_map(|(ship, fields)| (fields.is_some() && !self.sunk(ship)).then_some(ship))
    }

    pub fn fire(&mut self, target: Coord) -> Result<ShotOutcome, RuleError> {
        if self.shots[target.index()] {
            return Err(RuleError::AlreadyTargeted);
        }
        self.shots[target.index()] = true;

        Ok(match self.ship_at(target) {
            None => ShotOutcome::Miss,
            Some(ship) if self.sunk(ship) => ShotOutcome::Sunk { ship },
            Some(_) => ShotOutcome::Hit,
        })
    }

    pub fn cells(&self, show_fleet: bool) -> Vec<Cell> {
        Coord::all()
            .map(|coord| {
                let ship = self.ship_at(coord);
                match (self.shots[coord.index()], ship) {
                    (true, Some(ship)) if self.sunk(ship) => Cell::Sunk,
                    (true, Some(_)) => Cell::Hit,
                    (true, None) => Cell::Miss,
                    (false, Some(_)) if show_fleet => Cell::Ship,
                    (false, _) => Cell::Water,
                }
            })
            .collect()
    }

    fn ship_at(&self, coord: Coord) -> Option<usize> {
        self.ships
            .iter()
            .position(|ship| ship.as_ref().is_some_and(|fields| fields.contains(&coord)))
    }

    fn sunk(&self, ship: usize) -> bool {
        self.ships[ship]
            .as_ref()
            .is_some_and(|fields| fields.iter().all(|field| self.shots[field.index()]))
    }
}
