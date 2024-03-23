use std::cmp::min;

use bitvec::{bitvec, vec::BitVec};

#[derive(Clone, Debug)]
pub struct GameOfLife {
    height: usize,
    width: usize,
    curr: BitVec,
    next: BitVec,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let len = height * width;
        Self {
            height,
            width,
            curr: bitvec![0; len],
            next: bitvec![0; len],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn toggle(&mut self, x: usize, y: usize) -> Option<bool> {
        if !self.in_bounds(x, y) {
            return None;
        }

        let index = y * self.width + x;
        let new = !*self.curr.get(index)?;
        self.curr.set(index, new);

        Some(new)
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) -> Option<bool> {
        if !self.in_bounds(x, y) {
            return None;
        }

        let index = y * self.width + x;
        let old = *self.curr.get(index)?;
        self.curr.set(index, alive);

        Some(old)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if !self.in_bounds(x, y) {
            return None;
        }

        self.curr.get(y * self.width + x).map(|b| *b)
    }

    pub fn step(&mut self) {
        self.step_iter().for_each(|_update| {});
    }

    pub fn step_iter(&mut self) -> Updates {
        Updates {
            game: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Update {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
}

pub struct Updates<'a> {
    game: &'a mut GameOfLife,
    x: usize,
    y: usize,
}

impl Iterator for Updates<'_> {
    type Item = Update;

    fn next(&mut self) -> Option<Self::Item> {
        while self.x < self.game.width() {
            while self.y < self.game.height() {
                let xr = self.x.saturating_sub(1)..=min(self.x + 1, self.game.width - 1);
                let yr = self.y.saturating_sub(1)..=min(self.y + 1, self.game.height - 1);

                let alive = self.game.curr[self.y * self.game.width + self.x];

                let mut neighs = 0;
                for nx in xr {
                    for ny in yr.clone() {
                        if self.x == nx && self.y == ny {
                            continue;
                        }

                        neighs += self.game.curr[ny * self.game.width + nx] as u8;
                    }
                }

                let lives = match (alive, neighs) {
                    (true, 0..=1) => false,
                    (true, 2..=3) => true,
                    (true, 4..) => false,
                    (false, 3) => true,
                    _ => alive,
                };

                self.game.next.set(self.y * self.game.width + self.x, lives);

                let update = (alive != lives).then_some(Update {
                    x: self.x,
                    y: self.y,
                    alive: lives,
                });

                self.y += 1;

                if update.is_some() {
                    return update;
                }
            }

            self.x += 1;
            self.y = 0;
        }

        std::mem::swap(&mut self.game.curr, &mut self.game.next);

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set_toggle() {
        let mut game = GameOfLife::new(10, 10);

        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(game.get(x, y), Some(false));
            }
        }

        assert_eq!(game.get(10, 1), None);
        assert_eq!(game.get(1, 10), None);
        assert_eq!(game.get(100, 1), None);
        assert_eq!(game.get(1, 100), None);

        assert_eq!(game.set(5, 5, true), Some(false));
        assert_eq!(game.get(5, 5), Some(true));

        assert_eq!(game.set(5, 5, false), Some(true));
        assert_eq!(game.get(5, 5), Some(false));

        assert_eq!(game.toggle(3, 3), Some(true));
        assert_eq!(game.toggle(3, 3), Some(false));
    }

    #[test]
    fn block() {
        let mut game = GameOfLife::new(4, 4);

        for start in [(1, 1), (1, 2), (2, 1), (2, 2)] {
            game.set(start.0, start.1, true);
        }

        let updates = game.step_iter().collect::<Vec<_>>();
        assert_eq!(updates.len(), 0);

        let updates = game.step_iter().collect::<Vec<_>>();
        assert_eq!(updates.len(), 0);
    }

    #[test]
    fn spinner() {
        let mut game = GameOfLife::new(5, 5);

        for start in [(2, 1), (2, 2), (2, 3)] {
            game.set(start.0, start.1, true);
        }

        let updates = game.step_iter().collect::<Vec<_>>();

        assert_eq!(updates.len(), 4);
        assert!(updates.contains(&Update {
            x: 2,
            y: 1,
            alive: false
        }));
        assert!(updates.contains(&Update {
            x: 2,
            y: 3,
            alive: false
        }));
        assert!(updates.contains(&Update {
            x: 1,
            y: 2,
            alive: true
        }));
        assert!(updates.contains(&Update {
            x: 3,
            y: 2,
            alive: true
        }));
    }
}
