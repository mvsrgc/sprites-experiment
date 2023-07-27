use bracket_lib::prelude::*;

pub mod player;

const WIDTH: i32 = 65;
const HEIGHT: i32 = 52;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub struct Player {
    x: i32,
    y: i32,
}

struct State {
    map: Vec<TileType>,
    player_position: usize,
    visible: Vec<bool>,
    floor_glyph: i32,
    floor_glyphs: Vec<i32>,
    wall_glyphs: Vec<i32>,
    wall_glyph: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();

        for v in &mut self.visible {
            *v = false;
        }

        let player_position = self.index_to_point2d(self.player_position);
        let fov = field_of_view_set(player_position, 8, self);

        for idx in &fov {
            self.visible[xy_idx(idx.x, idx.y)];
        }

        draw_batch.target(0);
        draw_batch.cls();

        let mut x = 0;
        let mut y = 0;

        let mut rng = RandomNumberGenerator::new();

        for (i, tile) in self.map.iter().enumerate() {
            let mut fg = RGB::from_f32(1.0, 1.0, 1.0);
            let glyph;

            match tile {
                TileType::Floor => {
                    // 519 to 522
                    glyph = self.floor_glyphs[i]
                }
                TileType::Wall => {
                    // 206 to 207
                    glyph = self.wall_glyphs[i]
                }
            }

            if !self.visible[i] {
                fg = fg * 1.0;
            } else {
                let distance = 1.0
                    - (DistanceAlg::Pythagoras.distance2d(Point::new(x, y), player_position)
                        as f32
                        / 10.0);
                fg = RGB::from_f32(distance, distance, distance);
            }

            draw_batch.set(
                Point::new(x, y),
                ColorPair::new(fg, RGB::from_f32(0., 0., 0.)),
                glyph,
            );

            x += 1;
            if x > WIDTH - 1 {
                x = 0;
                y += 1;
            }

            draw_batch.submit(0).expect("Batch error");
            render_draw_buffer(ctx).expect("Render error");
        }
    }
}

impl State {
    pub fn new() -> State {
        let mut rng = RandomNumberGenerator::new();
        let floor_glyph = (rng.roll_dice(1, 5) - 1) + 519;
        let wall_glyph = (rng.roll_dice(1, 2) - 1) + 206;
        let floor_glyphs: Vec<i32> = (0..WIDTH * HEIGHT)
            .map(|_| (rng.roll_dice(1, 4) - 1) + 519)
            .collect();
        let wall_glyphs: Vec<i32> = (0..WIDTH * HEIGHT)
            .map(|_| (rng.roll_dice(1, 2) - 1) + 206)
            .collect();
        let mut state = State {
            map: vec![TileType::Floor; (WIDTH * HEIGHT) as usize],
            player_position: xy_idx(WIDTH / 2, HEIGHT / 2),
            visible: vec![false; (WIDTH * HEIGHT) as usize],
            floor_glyph,
            wall_glyph,
            floor_glyphs,
            wall_glyphs,
        };

        for x in 0..WIDTH {
            state.map[xy_idx(x, 0)] = TileType::Wall;
            state.map[xy_idx(x, HEIGHT - 1)] = TileType::Wall;
        }

        for y in 0..HEIGHT {
            state.map[xy_idx(0, y)] = TileType::Wall;
            state.map[xy_idx(WIDTH - 1, y)] = TileType::Wall;
        }

        for _ in 0..400 {
            let x = rng.range(1, WIDTH - 1);
            let y = rng.range(1, HEIGHT - 1);
            let idx = xy_idx(x, y);
            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        }

        state
    }

    pub fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let current_position = idx_xy(self.player_position);
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        let new_idx = xy_idx(new_position.0, new_position.1);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
    }
}

impl BaseMap for State {
    fn is_opaque(&self, idx: usize) -> bool {
        return self.map[idx] == TileType::Wall;
    }
}

impl Algorithm2D for State {
    fn dimensions(&self) -> Point {
        Point::new(WIDTH, HEIGHT)
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % WIDTH, idx as i32 / WIDTH)
}

bracket_lib::terminal::embedded_resource!(TILE_FONT, "../resources/tileset.png");

fn main() -> BError {
    bracket_lib::terminal::link_resource!(TILE_FONT, "../resources/tileset.png");
    println!("Hello, world!");

    let context = BTermBuilder::new()
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .with_tile_dimensions(13u32, 13u32)
        .with_font("tileset.png", 13u32, 13u32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(WIDTH as u32, HEIGHT as u32, "tileset.png")
        .with_sparse_console_no_bg(WIDTH as u32, HEIGHT as u32, "tileset.png")
        .with_title("Bracket Terminal - Sprite Console")
        .with_vsync(true)
        .build()?;

    let gs = State::new();

    main_loop(context, gs)
}
