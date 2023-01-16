extern crate rand;

use macroquad::prelude::*;
use rand::Rng;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

pub type Tile = Option<Color>;
const TILE_SIZE: f32 = 30f32;
const TILE_SPACING: f32 = TILE_SIZE / 6.5f32;

const TICK_LEN: f32 = 0.5f32;

const STATES: &[[&[&[Tile]]; 4]] = &[
    [
        &[
            &[None,          None,          None,          None],
            &[Some(SKYBLUE), Some(SKYBLUE), Some(SKYBLUE), Some(SKYBLUE)],
            &[None,          None,          None,          None],
            &[None,          None,          None,          None],
        ],
        &[
            &[None, None, Some(SKYBLUE), None],
            &[None, None, Some(SKYBLUE), None],
            &[None, None, Some(SKYBLUE), None],
            &[None, None, Some(SKYBLUE), None],
        ],
        &[
            &[None,          None,          None,          None],
            &[None,          None,          None,          None],
            &[Some(SKYBLUE), Some(SKYBLUE), Some(SKYBLUE), Some(SKYBLUE)],
            &[None,          None,          None,          None],
        ],
        &[
            &[None, Some(SKYBLUE), None, None],
            &[None, Some(SKYBLUE), None, None],
            &[None, Some(SKYBLUE), None, None],
            &[None, Some(SKYBLUE), None, None],
        ],
    ],
    [
        &[
            &[Some(DARKBLUE), None,           None],
            &[Some(DARKBLUE), Some(DARKBLUE), Some(DARKBLUE)],
            &[None,           None,           None],
        ],
        &[
            &[None, Some(DARKBLUE), Some(DARKBLUE)],
            &[None, Some(DARKBLUE), None],
            &[None, Some(DARKBLUE), None],
        ],
        &[
            &[None,           None,           None],
            &[Some(DARKBLUE), Some(DARKBLUE), Some(DARKBLUE)],
            &[None,           None,           Some(DARKBLUE)],
        ],
        &[
            &[None,           Some(DARKBLUE), None],
            &[None,           Some(DARKBLUE), None],
            &[Some(DARKBLUE), Some(DARKBLUE), None],
        ],
    ],
    [
        &[
            &[None,         None,         Some(ORANGE)],
            &[Some(ORANGE), Some(ORANGE), Some(ORANGE)],
            &[None,         None,         None],
        ],
        &[
            &[None, Some(ORANGE), None],
            &[None, Some(ORANGE), None],
            &[None, Some(ORANGE), Some(ORANGE)],
        ],
        &[
            &[None,         None,         None],
            &[Some(ORANGE), Some(ORANGE), Some(ORANGE)],
            &[Some(ORANGE), None,         None],
        ],
        &[
            &[Some(ORANGE), Some(ORANGE), None],
            &[None,         Some(ORANGE), None],
            &[None,         Some(ORANGE), None],
        ],
    ],
    [
        &[
            &[None, Some(YELLOW), Some(YELLOW), None],
            &[None, Some(YELLOW), Some(YELLOW), None],
        ],
        &[
            &[None, Some(YELLOW), Some(YELLOW), None],
            &[None, Some(YELLOW), Some(YELLOW), None],
        ],
        &[
            &[None, Some(YELLOW), Some(YELLOW), None],
            &[None, Some(YELLOW), Some(YELLOW), None],
        ],
        &[
            &[None, Some(YELLOW), Some(YELLOW), None],
            &[None, Some(YELLOW), Some(YELLOW), None],
        ],
    ],
    [
        &[
            &[None,        Some(GREEN), Some(GREEN)],
            &[Some(GREEN), Some(GREEN), None],
            &[None,        None,        None],
        ],
        &[
            &[None, Some(GREEN), None],
            &[None, Some(GREEN), Some(GREEN)],
            &[None, None,        Some(GREEN)],
        ],
        &[
            &[None,        None,        None],
            &[None,        Some(GREEN), Some(GREEN)],
            &[Some(GREEN), Some(GREEN), None],
        ],
        &[
            &[Some(GREEN), None,        None],
            &[Some(GREEN), Some(GREEN), None],
            &[None,        Some(GREEN), None],
        ],
    ],
    [
        &[
            &[None,         Some(PURPLE), None],
            &[Some(PURPLE), Some(PURPLE), Some(PURPLE)],
            &[None,         None,         None],
        ],
        &[
            &[None, Some(PURPLE), None],
            &[None, Some(PURPLE), Some(PURPLE)],
            &[None, Some(PURPLE), None],
        ],
        &[
            &[None,         None,         None],
            &[Some(PURPLE), Some(PURPLE), Some(PURPLE)],
            &[None,         Some(PURPLE), None],
        ],
        &[
            &[None,         Some(PURPLE), None],
            &[Some(PURPLE), Some(PURPLE), None],
            &[None,         Some(PURPLE), None],
        ],
    ],
    [
        &[
            &[Some(RED), Some(RED), None],
            &[None,      Some(RED), Some(RED)],
            &[None,      None,      None],
        ],
        &[
            &[None, None,      Some(RED)],
            &[None, Some(RED), Some(RED)],
            &[None, Some(RED), None],
        ],
        &[
            &[None,      None,      None],
            &[Some(RED), Some(RED), None],
            &[None,      Some(RED), Some(RED)],
        ],
        &[
            &[None,      Some(RED), None],
            &[Some(RED), Some(RED), None],
            &[Some(RED), None,      None],
        ],
    ],
];

pub struct Player<'a> {
    x: isize,
    y: isize,
    dir: usize,
    sprite: [&'a [&'a [Tile]]; 4]
}

impl<'a> Player<'a> {
    pub fn new() -> Self {
        let sprite = STATES[rand::thread_rng().gen_range(0..STATES.len())];
        Self {
            x: (WIDTH / 2 - (sprite[0][0].len() + 1) / 2) as isize,
            y: -1,
            dir: 0,
            sprite
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn key_detection<const W: usize, const H: usize>(&mut self, tiles: &Tiles<W, H>) {
        let prev_x = self.x;
        self.x += if is_key_pressed(KeyCode::Right) {
            1
        } else if is_key_pressed(KeyCode::Left) {
            -1
        } else {
            0
        };
        if self.collides(tiles) {
            self.x = prev_x;
        }
        let dir = self.dir;
        if is_key_pressed(KeyCode::A) {
            if self.dir == 0 {
                self.dir = 4;
            }
            self.dir -= 1;
        }
        if is_key_pressed(KeyCode::D) {
            self.dir += 1;
            self.dir %= 4;
        }
        if self.collides(tiles) {
            self.x += 1;
            if self.collides(tiles) {
                self.x -= 2;
                if self.collides(tiles) {
                    self.x += 1;
                    self.dir = dir;
                }
            }
        }
    }

    pub fn collides<const W: usize, const H: usize>(&self, tiles: &Tiles<W, H>) -> bool {
        let mut cond = false;
        for (tile_y, line) in self.sprite[self.dir].iter().enumerate() {
            for (tile_x, &tile) in line.iter().enumerate() {
                cond = cond || (tiles.tile_at(self.x + tile_x as isize, self.y + tile_y as isize).is_some() && tile.is_some());
            }
        }
        cond
    }

    pub fn place<const W: usize, const H: usize>(&self, tiles: &mut Tiles<W, H>) {
        for (tile_y, line) in self.sprite[self.dir].iter().enumerate() {
            for (tile_x, &tile) in line.iter().enumerate() {
                if tile.is_some() {
                    tiles.set_tile(self.x + tile_x as isize, self.y + tile_y as isize - 1, tile)
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.y += 1;
    }

    pub fn draw(&self, x: f32, y: f32) {
        for (tile_y, line) in self.sprite[self.dir].iter().enumerate() {
            for (tile_x, tile) in line.iter().enumerate() {
                if let &Some(color) = tile {
                    draw_rectangle(x + ((self.x + tile_x as isize) as f32 * TILE_SIZE) + TILE_SPACING / 2.,
                                   y + ((self.y + tile_y as isize) as f32 * TILE_SIZE) + TILE_SPACING / 2.,
                                   TILE_SIZE - TILE_SPACING,
                                   TILE_SIZE - TILE_SPACING,
                                   color);
                }
            }
        }
    }
}

pub struct Tiles<const W: usize, const H: usize> {
    tiles: Vec<[Tile; W]>
}

impl<const W: usize, const H: usize> Tiles<W, H> {
    pub fn new() -> Self {
        Self {
            tiles: vec![[None; W]; H]
        }
    }

    pub fn reset(&mut self) {
        self.tiles = vec![[None; W]; H];
    }

    pub fn update(&mut self) -> usize {
        let mut lines: usize = 0;
        self.tiles.retain(|&x| x.contains(&None));
        while self.tiles.len() < H {
            lines += 1;
            self.tiles.push([None; W]);
        }
        lines
    }

    pub fn tile_at(&self, x: isize, y: isize) -> Option<Color> {
        if y < 0 {
            None
        } else if x as usize >= W || x < 0 || y as usize >= H {
            Some(BLACK)
        } else {
            self.tiles[H - y as usize - 1][W - x as usize - 1]
        }
    }

    pub fn set_tile(&mut self, x: isize, y: isize, t: Tile) {
        if (x as usize) < W && x >= 0 && (y as usize) < H && y >= 0 {
            self.tiles[H - y as usize - 1][W - x as usize - 1] = t;
        }
    }
}

pub struct Game<'a, const W: usize, const H: usize> {
    score: usize,
    time: f32,
    pub tiles: Tiles<W, H>,
    player: Player<'a>
}

impl<'a, const W: usize, const H: usize> Game<'a, W, H> {
    pub fn new() -> Self {
        Self {
            score: 0,
            time: 0f32,
            tiles: Tiles::new(),
            player: Player::new()
        }
    }

    pub fn update(&mut self) {
        let y = self.player.y;
        self.player.key_detection(&self.tiles);
        if self.time >= if is_key_down(KeyCode::Down) { TICK_LEN / 16f32 } else { TICK_LEN } {
            self.player.update();
            if self.player.collides(&self.tiles) {
                self.player.place(&mut self.tiles);
                self.score += self.tiles.update();
                self.player.reset();
                while self.player.collides(&self.tiles) {
                    self.player.y -= 1;
                }
            }
            self.time = 0f32;
            if self.player.y == y {
                self.tiles.reset();
                self.player.reset();
                self.score = 0;
            }
        }
        self.time += get_frame_time();
    }

    pub fn draw(&mut self, x: f32, y: f32) {
        draw_rectangle(x, y, W as f32 * TILE_SIZE, H as f32 * TILE_SIZE, DARKGRAY);
        for bx in 0..W {
            for by in 0..H {
                if let Some(color) = self.tiles.tile_at(bx as isize, by as isize) {
                    draw_rectangle(x + (bx as f32 * TILE_SIZE) + TILE_SPACING / 2.,
                                   y + (by as f32 * TILE_SIZE) + TILE_SPACING / 2.,
                                   TILE_SIZE - TILE_SPACING,
                                   TILE_SIZE - TILE_SPACING,
                                   color);
                }
            }
        }
        self.player.draw(x, y);
        draw_text(&self.score.to_string(), x + TILE_SIZE / 10f32, y + TILE_SIZE * 0.9f32, TILE_SIZE * 1.5, WHITE);
    }
}
