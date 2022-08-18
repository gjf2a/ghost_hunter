#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, plot_str, plot_num, clear_row, ColorCode, Color};
use ghost_hunter_core::{Status, Position, Cell};

pub use ghost_hunter_core::GhostHunterGame;

const GAME_HEIGHT: usize = BUFFER_HEIGHT - 2;
const HEADER_SPACE: usize = BUFFER_HEIGHT - GAME_HEIGHT;

pub type MainGame = GhostHunterGame<BUFFER_WIDTH,GAME_HEIGHT>;

const GHOST_COLORS: [Color; 4] = [Color::Red, Color::Pink, Color::LightGreen, Color::Cyan];

pub fn tick(game: &mut MainGame) {
    if game.countdown_complete() {
        game.update();
        draw(game);
    }
}

fn draw(game: &MainGame) {
    draw_header(game);
    draw_board(game);
}

fn draw_header(game: &MainGame) {
    match game.status() {
        Status::Normal => draw_normal_header(game),
        Status::Over => draw_game_over_header(game),
        Status::Empowered => draw_empowered_header(game)
    }
}

fn draw_normal_header(game: &MainGame) {
    let header_color = ColorCode::new(Color::White, Color::Black);
    let score_text = "Score:";
    clear_row(0, Color::Black);
    clear_row(1, Color::Black);
    plot_str(score_text, 0, 0, header_color);
    plot_num(game.score() as isize, score_text.len() + 1, 0, header_color);
}

fn draw_subheader(subheader: &str) {
    plot_str(subheader, 0, 1, ColorCode::new(Color::LightRed, Color::Black));
}

fn draw_game_over_header(game: &MainGame) {
    draw_normal_header(game);
    draw_subheader("Game over. Press S to restart.");
}

fn draw_empowered_header(game: &MainGame) {
    draw_normal_header(game);
    const POWER_UP_MSG: &str = "Powered up!";
    draw_subheader(POWER_UP_MSG);
    for i in 0..game.empowered_ticks_left() {
        plot('.', i + POWER_UP_MSG.len(), 1, ColorCode::new(Color::Cyan, Color::Black));
    }
}

fn draw_board(game: &MainGame) {
    for p in game.cell_pos_iter() {
        let (row, col) = p.row_col();
        let (c, color) = get_icon_color(game, p, &game.cell(p));
        plot(c, col, row + HEADER_SPACE, color);
    }
}

fn get_icon_color(game: &MainGame, p: Position<BUFFER_WIDTH,GAME_HEIGHT>, cell: &Cell) -> (char, ColorCode) {
    let (icon, foreground) =
        if p == game.ghost_hunter_at() {
            (match game.status() {
                Status::Over => '*',
                _ => game.ghost_hunter_icon()
            }, Color::Yellow)
        } else {
            if let Some((g, ghost)) = game.ghost_at(p) {
                (ghost.icon(), GHOST_COLORS[g])
            } else {
                match cell {
                    Cell::Dot => ('.', Color::White),
                    Cell::Empty => (' ', Color::Black),
                    Cell::Wall => ('#', Color::Blue),
                    Cell::PowerDot => ('O', Color::Green)
                }
            }
        };
    (icon, ColorCode::new(foreground, Color::Black))
}
