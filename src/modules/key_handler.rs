use std::io::Stdout;

use ratatui::{crossterm::event::{KeyCode, KeyEvent, KeyEventKind}, prelude::CrosstermBackend, Terminal};

use super::game::{Game, Screen};

pub fn menu_key_handler(key : KeyEvent, game : &mut Game, selected : &mut usize, terminal : &Terminal<CrosstermBackend<Stdout>>) {
    if key.kind == KeyEventKind::Press {
        if key.code == KeyCode::Esc {
            game.exit = true;
        }
        if key.code == KeyCode::Up {
            if *selected < 2 {*selected += 1;}
        }
        if key.code == KeyCode::Down {
            if *selected > 0 {
                *selected -= 1;
            }
        }
        if key.code == KeyCode::Enter {
            match *selected {
                0 => game.exit = true,
                1 => game.start(Screen::Snake(super::game::Rule::Bordered), terminal.size().unwrap()),
                2 => game.start(Screen::Snake(super::game::Rule::NoBorder), terminal.size().unwrap()),
                _ => panic!("Option not implemented.")
            }
        }
    }
}

pub fn snake_key_handler(key : KeyEvent, game : &mut Game) {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Esc => game.screen = Screen::MainMenu,
            KeyCode::Up => game.snake.as_mut().unwrap().move_up(),
            KeyCode::Down => game.snake.as_mut().unwrap().move_down(),
            KeyCode::Left => game.snake.as_mut().unwrap().move_left(),
            KeyCode::Right => game.snake.as_mut().unwrap().move_right(),
            _ => {}
        }
    }
}