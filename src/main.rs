use std::{io::stdout, time::Duration};

use modules::{game::{Game, Screen}, game_over, key_handler, main_menu, snake};
use ratatui::{crossterm::{event::{self, KeyEventKind}, terminal::{enable_raw_mode, EnterAlternateScreen}, ExecutableCommand}, prelude::CrosstermBackend, Terminal};

pub mod modules;

fn main() -> std::io::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut game : Game = Game::new();
 
    let main_menu_items = ["Exit", "Bordered Mode", "No Border Mode"];
    let mut selected = 0;
    loop {
        match game.screen {
            Screen::MainMenu => main_menu::render(&mut terminal, main_menu_items, &mut selected)?,
            Screen::Snake(rule) => {
                snake::render(&mut terminal, &mut game, rule)?;
            },
            Screen::GameOver(points) => {game_over::render(&mut terminal, points)?;}
        }

        if event::poll(Duration::from_millis(1))? {
            if let event::Event::Key(key) = event::read()? {
                match game.screen {
                    Screen::MainMenu => key_handler::menu_key_handler(key, &mut game, &mut selected, &terminal),
                    Screen::Snake(_) => key_handler::snake_key_handler(key, &mut game),
                    Screen::GameOver(_) => {
                        if key.kind == KeyEventKind::Press {
                            game.screen = Screen::MainMenu;
                        }
                    }
                }
            }
        }

        if game.exit == true {
            terminal.clear()?;
            break;
        }
    }

    Ok(())
}
