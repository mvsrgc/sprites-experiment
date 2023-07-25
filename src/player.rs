use bracket_lib::terminal::{VirtualKeyCode, BTerm, INPUT};

use crate::State;

use super::Player;

const PLAYER_SPEED: f32 = 100.0;

pub fn try_move_player(mut dx: f32, mut dy: f32, player: &mut Player, ctx: &mut BTerm) {
    let len = (dx * dx + dy * dy).sqrt();
    if len > 0.0 {
        dx /= len;
        dy /= len;
    }

    player.x = (player.x as f32 + dx * ctx.frame_time_ms / 1000.0 * PLAYER_SPEED) as i32;
    player.y = (player.y as f32 + dy * ctx.frame_time_ms / 1000.0 * PLAYER_SPEED) as i32;
}

pub fn player_input(ctx: &mut BTerm, player: &mut Player) {
    let mut dx: f32 = 0.0;
    let mut dy: f32 = 0.0;

    let mut input = INPUT.lock();
    if input.is_key_pressed(VirtualKeyCode::Up) {
        dy = -1.0;
    }
    if input.is_key_pressed(VirtualKeyCode::Down) {
        dy = 1.0;
    }
    if input.is_key_pressed(VirtualKeyCode::Left) {
        dx = -1.0;
    }
    if input.is_key_pressed(VirtualKeyCode::Right) {
        dx = 1.0;
    }

    try_move_player(dx, dy, player, ctx);
}
