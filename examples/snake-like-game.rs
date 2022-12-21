extern crate rand;
extern crate sdl2;

use rand::{thread_rng, Rng};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};
use std::{collections::VecDeque, result, time::Duration};

// Our exceptionally basic snake style video game core.
struct Game {
    // Game grid size.
    grid_size: Point,
    // Food position.
    food: Point,
    // Snake body positions.
    snake: VecDeque<Point>,
    // Snake travel direction.
    snake_direction: Direction,
    // Snake alive/ dead status.
    snake_alive: bool,
    // Whether we can accept a command/ user input this game cycle.
    // We use this to throttle keyboard spamming and prevent the snake turning multiple times
    // within a single game cycle.
    has_command: bool,
}

impl Game {
    fn new(grid_size: Point) -> Self {
        Self {
            grid_size,
            food: Point::new(0, 0),
            snake: Default::default(),
            snake_direction: Direction::Right,
            snake_alive: false,
            has_command: false,
        }
    }

    fn turn_snake(&mut self, direction: Direction) {
        // Check we can accept a command and that snake is not turning in opposite direction.
        if !self.has_command && self.snake_direction != direction.opposite() {
            // Update direction.
            self.snake_direction = direction;
            // Stop accepting additional commands for this game cycle.
            self.has_command = true;
        }
    }

    fn update(&mut self) {
        // Exit if snake is dead.
        if !self.snake_alive {
            return;
        }
        // Calculate our new snake head position.
        let old_head = self.snake.back().expect("internal error");
        let (mut x, mut y) = (old_head.x(), old_head.y());
        match self.snake_direction {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        let new_head = Point::new(x, y);
        // Check if snake is ok, i.e. no snake to snake or snake to boundary collision.
        if new_head.x() < 0
            || new_head.y() < 0
            || new_head.x() >= self.grid_size.x()
            || new_head.y() >= self.grid_size.y()
            || self.snake.contains(&new_head)
        {
            // Snake not ok. Actually it's now quite dead.
            self.snake_alive = false;
        } else {
            // Snake is ok. Push snake head.
            self.snake.push_back(new_head);
            // Check if snake has taken food.
            if new_head == self.food {
                // Snake has taken food.
                // Do not pop tail to increase length.
                // Put more food.
                self.put_food();
            } else {
                // Snake has not taken food.
                // Pop tail to maintain length.
                self.snake.pop_front();
            }
            // Now accepting commands.
            self.has_command = false;
        }
    }

    fn reset_snake(&mut self) {
        // Clear old snake body.
        self.snake.clear();
        // Spawn a new snake.
        self.snake
            .push_back(Point::new(self.grid_size.x / 2, self.grid_size.y / 2));
        self.snake_alive = true;
        // Now accepting commands.
        self.has_command = false;
    }

    fn put_food(&mut self) {
        for _ in 0..1024 {
            // Put out food in a random location.
            self.food = Point::new(
                thread_rng().gen_range(0, self.grid_size.x),
                thread_rng().gen_range(0, self.grid_size.y),
            );
            // Check we haven't spawned food within snake body, if so we try again.
            if !self.snake.contains(&self.food) {
                break;
            }
        }
        // Unable to put food. The grid is likely very full and our simple demo algorithm
        // has failed us.
    }

    fn draw<E, R>(&self, mut render: R) -> result::Result<(), E>
    where
        R: FnMut(Point, Color) -> Result<(), E>,
    {
        // Render food tile.
        render(self.food, Color::RED)?;
        // Select snake alive/ dead color.
        let color = match self.snake_alive {
            true => Color::YELLOW,
            false => Color::GREEN,
        };
        // Render snake body tiles.
        for &snake_pos in &self.snake {
            render(snake_pos, color)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn main() -> Result<(), String> {
    // Initialize SDL2.
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("SDL2 SNAKE STYLE GAME", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // Print basic game instructions.
    println!("Welcome to Snake Style Game!");
    println!("Use arrow keys to control the snake.");
    println!("Close window/ press escape to quit.");
    println!("Restart game if snake dies to play again.");

    // Initialize our game.
    let mut game = Game::new(Point::new(40, 30));
    game.reset_snake();
    game.put_food();

    let mut running = true;
    while running {
        // Handle events.
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => running = false,
                    Some(Keycode::Left) => game.turn_snake(Direction::Left),
                    Some(Keycode::Right) => game.turn_snake(Direction::Right),
                    Some(Keycode::Up) => game.turn_snake(Direction::Up),
                    Some(Keycode::Down) => game.turn_snake(Direction::Down),
                    _ => {}
                },
                Event::Quit { .. } => running = false,
                _ => {}
            }
        }
        // Clear canvas.
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        canvas.clear();
        // Update game state and render game tiles as filled rectangles.
        game.update();
        game.draw::<String, _>(|tile, color| {
            let rect = Rect::new(tile.x() * 16, tile.y() * 16, 16, 16);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect)
        })?;
        // Flip canvas.
        canvas.present();
        // Sleep/ limit frame rate.
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
