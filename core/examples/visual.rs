use std::path::Path;

use minesweeper_core::*;
use piston_window::types::Color;
use piston_window::{rectangle::square, *};

const SIZE_FACTOR: f64 = 40.0;

fn draw_cell(cell: &Cell, coordinates: Point, graphics: &mut G2d, context: Context, glyphs: &mut Glyphs) {
    let x = coordinates.y as f64;
    let y = coordinates.x as f64;

    let border = square(x * SIZE_FACTOR, y * SIZE_FACTOR, SIZE_FACTOR);
    let border_rect = Rectangle::new(color::BLACK);
    border_rect.draw(border, &context.draw_state, context.transform, graphics);

    match cell.state {
        CellState::Hidden | CellState::Flagged => draw_cell_info(None, color::GRAY, x, y, graphics, context, glyphs),
        CellState::Cleared => match cell.kind {
            CellKind::Empty => draw_cell_info(None, color::WHITE, x, y, graphics, context, glyphs),
            CellKind::Number(number) => draw_cell_info(
                Some(&format!("{}", number)),
                color::WHITE,
                x,
                y,
                graphics,
                context,
                glyphs,
            ),
            CellKind::Mine => draw_cell_info(None, color::RED, x, y, graphics, context, glyphs),
        },
    }
}

fn draw_cell_info(
    text: Option<&str>,
    color: Color,
    x: f64,
    y: f64,
    graphics: &mut G2d,
    context: Context,
    glyphs: &mut Glyphs,
) {
    let inner_square = square(x * SIZE_FACTOR + 1.0, y * SIZE_FACTOR + 1.0, SIZE_FACTOR - 2.0);
    let rectangle = Rectangle::new(color);
    rectangle.draw(inner_square, &context.draw_state, context.transform, graphics);

    if let Some(text) = text {
        text::Text::new_color(color::BLACK, 20)
            .draw(
                text,
                glyphs,
                &context.draw_state,
                context.transform.trans(x * SIZE_FACTOR + 14.0, y * SIZE_FACTOR + 27.0),
                graphics,
            )
            .unwrap();
    }
}

fn main() {
    let mut game = Game::new(Difficulty::Easy);
    let size = game.board.get_size();
    let mut window: PistonWindow = WindowSettings::new(
        "MinesBooMer",
        [size.width as f64 * SIZE_FACTOR, size.height as f64 * SIZE_FACTOR],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut last_cursor_position: [f64; 2] = [0.0, 0.0];

    let font_path = "examples/assets/FiraSans-Medium.ttf";
    let mut glyphs = window.load_font(Path::new(&font_path)).unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let mine_path = "examples/assets/mine.png";
    let mine_texture = Texture::from_path(
        &mut texture_context,
        Path::new(&mine_path),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    let flag_path = "examples/assets/flag.png";
    let flag_texture = Texture::from_path(
        &mut texture_context,
        Path::new(&flag_path),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, device| {
            clear([1.0; 4], graphics);

            game.board.iter().for_each(|(coordinates, cell)| {
                draw_cell(cell, coordinates, graphics, context, &mut glyphs);
            });

            let scale = SIZE_FACTOR / mine_texture.get_size().0 as f64;
            let flag_scale = (SIZE_FACTOR - 10.0) / flag_texture.get_size().1 as f64;

            game.board.iter().for_each(|(coordinates, cell)| {
                if cell.is_flagged() {
                    image(
                        &flag_texture,
                        context
                            .transform
                            .trans_pos([
                                coordinates.y as f64 * SIZE_FACTOR + 5.0,
                                coordinates.x as f64 * SIZE_FACTOR + 5.0,
                            ])
                            .scale(flag_scale, flag_scale),
                        graphics,
                    );
                } else if cell.is_mine() && cell.is_cleared() {
                    image(
                        &mine_texture,
                        context
                            .transform
                            .trans_pos([coordinates.y as f64 * SIZE_FACTOR, coordinates.x as f64 * SIZE_FACTOR])
                            .scale(scale, scale),
                        graphics,
                    );
                }
            });

            glyphs.factory.encoder.flush(device);
        });

        event.mouse_cursor(|position| {
            last_cursor_position = position;
        });

        event.button(|args| {
            if game.is_game_over() || game.is_win() {
                return;
            }

            match args.button {
                Button::Mouse(button) => {
                    if args.state == ButtonState::Release {
                        let coordinates = Point {
                            x: (last_cursor_position[1] / SIZE_FACTOR) as usize,
                            y: (last_cursor_position[0] / SIZE_FACTOR) as usize,
                        };
                        match button {
                            MouseButton::Left => _ = game.selected_at(coordinates),
                            MouseButton::Right => game.toggle_flagged(coordinates),
                            _ => {}
                        }
                    }
                    if game.is_win() {
                        game.clear_all()
                    }
                }
                Button::Keyboard(_) => {}
                Button::Controller(_) => {}
                Button::Hat(_) => {}
            }
        });
    }
}
