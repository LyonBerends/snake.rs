use std::io::Stdout;

use ratatui::{crossterm::event::{KeyCode, KeyEvent, KeyEventKind}, prelude::CrosstermBackend, Terminal};

use super::game::{Game, Screen};

pub fn menu_key_handler(key : KeyEvent, game : &mut Game, selected : &mut usize, terminal : &Terminal<CrosstermBackend<Stdout>>) {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Esc => game.exit = true,
            KeyCode::Up | KeyCode::Char('w') => {if *selected < 2 {*selected += 1;}},
            KeyCode::Down | KeyCode::Char('s') => {if *selected > 0 {*selected -= 1;}}
            KeyCode::Enter => {
                match *selected {
                    0 => game.exit = true,
                    1 => game.start(Screen::Snake(super::game::Rule::Bordered), terminal.size().unwrap()),
                    2 => game.start(Screen::Snake(super::game::Rule::NoBorder), terminal.size().unwrap()),
                    _ => panic!("Option not implemented.")
                }
            },
            _ => {}
        }
    }
}

pub fn snake_key_handler(key : KeyEvent, game : &mut Game) {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Esc => game.screen = Screen::MainMenu,
            KeyCode::Up | KeyCode::Char('w') => game.snake.as_mut().unwrap().move_up(),
            KeyCode::Down | KeyCode::Char('s') => game.snake.as_mut().unwrap().move_down(),
            KeyCode::Left | KeyCode::Char('a') => game.snake.as_mut().unwrap().move_left(),
            KeyCode::Right | KeyCode::Char('d') => game.snake.as_mut().unwrap().move_right(),
            _ => {}
        }
    }
}

// pub fn press_any_key_to_continue() -> std::io::Result<()> {
//     loop {
//         if event::poll(Duration::from_millis(5))? {
//             if let event::Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Press {
//                     break;
//                 }
//             }
//         }
//     }

//     Ok(())
// }