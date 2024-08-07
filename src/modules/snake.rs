use std::{io::Stdout, thread::sleep, time::Duration};

use ratatui::{layout::Rect, prelude::CrosstermBackend, style::{Style, Stylize}, widgets::{Block, Paragraph}, Terminal};

use super::game::{Fruit, Game, Rule};

pub fn render(terminal : &mut Terminal<CrosstermBackend<Stdout>>, game : &mut Game, rule : Rule) -> std::io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.size();

        let b = match rule {Rule::Bordered => {Block::bordered()}, Rule::NoBorder => {Block::default()}};
        frame.render_widget(b, area);

        let mut new_pos;
        
        let snake = game.snake.as_ref().unwrap();

        for pos in snake.body.iter() {
            frame.render_widget(Paragraph::new("█").green(), Rect::new(pos.0 as u16, pos.1 as u16, 1, 1));
        }

        let last = *snake.body.back().unwrap();
        new_pos = (last.0 + snake.velocity.0, last.1 + snake.velocity.1);

        let fruit_loc : (usize, usize);
        let fruit_style;

        match game.fruit.as_mut() {
            Some(fruit) => {
                match fruit {
                    Fruit::Common(loc) => {
                        fruit_loc = *loc;
                        fruit_style = Style::new().red();
                    },
                    Fruit::Uncommon(loc) => {
                        fruit_loc = *loc;
                        fruit_style = Style::new().blue();
                    },
                    Fruit::Rare(loc) => {
                        fruit_loc = *loc;
                        fruit_style = Style::new().yellow();
                    }
                }
            },
            None => {
                fruit_loc = (0xffffffff, 0xffffffff);
                fruit_style = Style::new();
            }
        }

        if (new_pos.0 as usize, new_pos.1 as usize) == fruit_loc {
            game.eat_fruit(area);
        } else {
            frame.render_widget(Paragraph::new("█").style(fruit_style), Rect::new(fruit_loc.0 as u16, fruit_loc.1 as u16, 1, 1));
        }
        
        match rule {
            Rule::Bordered => {
                if new_pos.0 >= area.width as i32 - 1 || new_pos.0 <= 0 || new_pos.1 >= area.height as i32 - 1 || new_pos.1 <= 0 {
                    game.game_over();
                    return ();
                }
            },
            Rule::NoBorder => {
                if new_pos.0 < 0 {new_pos.0 = area.width as i32 - 1;}
                if new_pos.0 == area.width as i32 {new_pos.0 = 0;}
                if new_pos.1 < 0 {new_pos.1 = area.height as i32 - 1;}
                if new_pos.1 == area.height as i32 {new_pos.1 = 0;}
            }
        }

        let snake = game.snake.as_mut().unwrap();

        if snake.body.contains(&new_pos) {
            game.game_over();
            return ();
        }

        snake.body.push_back(new_pos);
        if snake.body.len() > snake.length {
            snake.body.pop_front();
        }
    })?;

    sleep(Duration::from_millis(33));

    Ok(())
}