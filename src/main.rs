extern crate macroquad;

use macroquad::prelude::*;

mod game;
use game::*;

#[macroquad::main("tetris")]
async fn main() {
    let mut game = Game::<10, 20>::new();
    loop {
        clear_background(BLACK);
        game.update();
        game.draw(0f32, 0f32);
        next_frame().await
    }
}
