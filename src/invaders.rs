use std::{cmp::max, time::Duration};

use rusty_time::Timer;

use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

pub struct Invader {
    x: usize,
    y: usize,
}

pub struct Invaders {
    army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in (1..(NUM_COLS - 1)).step_by(2) {
            for y in (1..(NUM_ROWS / 2)).step_by(2) {
                army.push(Invader { x, y })
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;

            // Move downwards if we reach left / right sides.
            if self.direction == -1
                && self.army.iter().map(|invader| invader.x).min().unwrap_or(0) == 0
            {
                self.direction = 1;
                downwards = true;
            } else if self.direction == 1
                && self.army.iter().map(|invader| invader.x).max().unwrap_or(0) == NUM_COLS - 1
            {
                self.direction = -1;
                downwards = true;
            }

            if downwards {
                self.move_timer =
                    Timer::from_millis(max(self.move_timer.duration.as_millis() - 250, 250) as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = (invader.x as i32 + self.direction) as usize;
                }
            }

            return true;
        }
        false
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| invader.x == x && invader.y == y)
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            };
        }
    }
}
