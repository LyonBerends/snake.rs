use std::collections::VecDeque;

use rand::{rngs::ThreadRng, Rng};
use ratatui::layout::Rect;


#[derive(Clone, Copy)]
pub enum Rule {
    Bordered,
    NoBorder
}
pub enum Screen {
    MainMenu,
    Snake (Rule)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Left,
    Right, 
    Up, 
    Down
}

pub struct Game {
    pub exit : bool,
    pub screen : Screen,
    pub snake : Option<Snake>,
    pub fruit : Option<(usize, usize)>,
    pub rng : ThreadRng
}

#[derive(Clone)]
pub struct Snake {
    pub body : VecDeque<(i32, i32)>,
    pub velocity : (i32, i32),
    pub length : usize,
    pub moving : Move
}

impl Snake {
    pub fn move_left(&mut self) {
        if self.moving != Move::Right {
            self.velocity = (-1, 0);
            self.moving = Move::Left;
        }
    }
    pub fn move_right(&mut self) {
        if self.moving != Move::Left {
            self.velocity = (1, 0);
            self.moving = Move::Right;
        }
    }
    pub fn move_up(&mut self) {
        if self.moving != Move::Down {
            self.velocity = (0, -1);
            self.moving = Move::Up;
        }
    }
    pub fn move_down(&mut self) {
        if self.moving != Move::Up {
            self.velocity = (0, 1);
            self.moving = Move::Down;
        }
    }

    pub fn eat(&mut self) {
        self.length += 1;
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            exit : false,
            screen : Screen::MainMenu,
            snake : None,
            fruit : None,
            rng : rand::thread_rng()
        }
    }

    pub fn die(&mut self) {
        self.snake = None;
        self.screen = Screen::MainMenu;
    }

    pub fn spawn_snake(&mut self, x : u16, y : u16) {
        let mut body : VecDeque<(i32, i32)> = VecDeque::new();
        body.push_back((x as i32, y as i32));

        self.snake = Some(Snake {body, velocity : (0, 1), length : 3, moving : Move::Down});
    }

    pub fn start(&mut self, screen : Screen, size : Rect) {
        self.spawn_snake(size.as_size().width / 2, size.as_size().height / 2);

        self.spawn_fruit(size);

        self.screen = screen;
    }

    pub fn spawn_fruit(&mut self, size : Rect) {
        while self.fruit == None {
            let fruit_coords = (self.rng.gen_range(2..size.as_size().width) as usize, self.rng.gen_range(2..size.as_size().height) as usize);
            if !self.snake.as_mut().unwrap().body.contains(&(fruit_coords.0 as i32, fruit_coords.1 as i32)) {
                self.fruit = Some(fruit_coords);
            }
        }
    }

    pub fn eat_fruit(&mut self, size : Rect) {
        self.fruit = None;

        self.snake.as_mut().unwrap().length += 1;
        self.spawn_fruit(size);
    }
}