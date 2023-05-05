//
// Daniel Bauer (bauerda@pm.me)
//

use bevy::prelude::*;
use super::maze::*;

const MAZE_START_X: u32 = 100;
const MAZE_END_X: u32 = MAZE_WIDTH - 100;
const MAZE_START_Y: u32 = 100;
const MAZE_END_Y: u32 = MAZE_HEIGHT - 100;

const OBSTACLES: [Rect; 28] = [
    
    Rect { x: 1133, y: 100, w: 100, h: 133 }, // Spike 1
    Rect { x: 1533, y: 100, w: 100, h: 133 }, // Spike 2
    Rect { x: 2666, y: 100, w: 100, h: 200 }, // Spike 3

    Rect { x: 200, y: 200, w: 233, h: 100 }, // Bottom 1
    Rect { x: 533, y: 200, w: 500, h: 100 }, // Bottom 2
    Rect { x: 1733, y: 200, w: 833, h: 100 }, // Bottom 3
    Rect { x: 2866, y: 200, w: 300, h: 100 }, // Bottom 4
    Rect { x: 3266, y: 200, w: 234, h: 100 }, // Bottom 5

    Rect { x: 100, y: 400, w: 266, h: 166 }, // Left Gate Bottom
    Rect { x: 100, y: 666, w: 266, h: 167 }, // Left Gate Top
    //
    Rect { x: 3333, y: 400, w: 266, h: 166 }, // Right Gate Bottom
    Rect { x: 3333, y: 666, w: 266, h: 167 }, // Right Gate Top

    Rect { x: 466, y: 400, w: 567, h: 633 }, // D

    Rect { x: 1733, y: 400, w: 167, h: 433 }, // n 1
    Rect { x: 1900, y: 733, w: 100, h: 100 }, // n 2
    Rect { x: 2000, y: 400, w: 166, h: 433 }, // n 3

    Rect { x: 2266, y: 400, w: 167, h: 433 }, // i

    Rect { x: 2533, y: 400, w: 167, h: 433 }, // e 1
    Rect { x: 2533, y: 400, w: 433, h: 100 }, // e 2
    Rect { x: 2533, y: 600, w: 433, h: 233 }, // e 3

    Rect { x: 3066, y: 400, w: 167, h: 633 }, // l

    Rect { x: 1133, y: 533, w: 500, h: 300 }, // Box

    Rect { x: 1133, y: 333, w: 500, h: 100 }, // Start 1
    Rect { x: 1333, y: 200, w: 100, h: 233 }, // Start 2

    Rect { x: 200, y: 933, w: 166, h: 100 }, // Top 1
    Rect { x: 1133, y: 933, w: 500, h: 100 }, // Top 2
    Rect { x: 1733, y: 933, w: 1233, h: 100 }, // Top 3
    Rect { x: 3333, y: 933, w: 167, h: 100 }, // Top 4
];

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    fn collide(&self, x: i32, y: i32, size: u32) -> bool {
        let offset = (size / 2) as i32;
        self.x < x + offset && self.x + self.w > x - offset
            && self.y < y + offset && self.y + self.h > y - offset
    }
}

pub fn check_in_map(x: i32, y: i32, size: u32) -> bool {
    let offset = (size / 2) as i32;
    x - offset >= MAZE_START_X as i32 && x + offset <= MAZE_END_X as i32
        && y - offset >= MAZE_START_Y as i32 && y + offset <= MAZE_END_Y as i32
}

pub fn check_for_collisions(x: i32, y: i32, size: u32) -> bool {
    for obstacle in OBSTACLES.iter() {
        if obstacle.collide(x, y, size) {
            return true;
        }
    }
    false
}