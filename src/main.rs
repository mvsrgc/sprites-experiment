use std::collections::HashMap;

use bracket_lib::prelude::*;

pub mod player;
use player::*;

pub struct Player {
    x: i32,
    y: i32,
}

struct State {
    frame: usize,
    timer: f32,
    players: Vec<Player>,
    key_state: HashMap<VirtualKeyCode, bool>
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let player = &mut self.players[0];

        player_input(ctx, player);

        ctx.cls();

        ctx.print(1, 1, "Watch them go!");
        ctx.printer(
            1,
            2,
            format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Left,
            None,
        );

        ctx.add_sprite(
            Rect::with_size(player.x, player.y, 32, 32),
            400 - player.y,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            self.frame % 4,
        );

        self.timer += ctx.frame_time_ms;
    }
}

fn main() -> BError {
    println!("Hello, world!");

    let context = BTermBuilder::new()
        .with_sprite_console(640, 400, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_title("Bracket Terminal - Sprite Console")
        .with_sprite_sheet(
            SpriteSheet::new("resources/sprite_dood.png")
                .add_sprite(Rect::with_size(0, 0, 85, 132)),
        )
        .with_vsync(false)
        .build()?;

    let mut players = Vec::new();
    players.push(Player { x: 120, y: 120 });

    let gs = State {
        frame: 0,
        timer: 0.0,
        players,
        key_state: HashMap::default(),
    };

    main_loop(context, gs)
}
