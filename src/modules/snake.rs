use std::io::Stdout;

use ratatui::{layout::Rect, prelude::CrosstermBackend, style::Stylize, widgets::{Block, Paragraph}, Terminal};

use super::game::{Game, Rule};

pub fn render(terminal : &mut Terminal<CrosstermBackend<Stdout>>, game : &mut Game, rule : Rule) -> std::io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.size();

        let b = match rule {Rule::Bordered => {Block::bordered()}, Rule::NoBorder => {Block::default()}};

        let mut new_pos;
        
        {
            let snake = game.snake.as_ref().unwrap();

            for pos in snake.body.iter() {
                frame.render_widget(Paragraph::new("█").green(), Rect::new(pos.0 as u16, pos.1 as u16, 1, 1));
            }

            let last = *snake.body.back().unwrap();
            new_pos = (last.0 + snake.velocity.0, last.1 + snake.velocity.1);
        }

        let fruit_clone = game.fruit.clone();

        if fruit_clone.is_some() {
            let fruit = fruit_clone.unwrap();
            if (new_pos.0 as usize, new_pos.1 as usize) == fruit {
                game.eat_fruit(area);
            } else {
                frame.render_widget(Paragraph::new("█").red(), Rect::new(fruit.0 as u16, fruit.1 as u16, 1, 1));
            }
        }
        
        match rule {
            Rule::Bordered => {
                if new_pos.0 >= area.width as i32 || new_pos.0 <= 0 || new_pos.1 >= area.height as i32 || new_pos.1 <= 1 {
                    game.die();
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
            game.die();
            return ();
        }

        snake.body.push_back(new_pos);
        if snake.body.len() > snake.length {
            snake.body.pop_front();
        }

        frame.render_widget(b, area);
    })?;

    Ok(())
}