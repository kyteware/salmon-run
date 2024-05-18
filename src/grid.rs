use iced::{mouse, widget::{canvas::{self, Frame, Path}, Canvas}, Color, Element, Length, Point, Renderer, Size, Theme};
use rand::{Rng, thread_rng};

use crate::{app::Message, instruction::Instruction, level::Level};

#[derive(Debug, Default)]
pub struct Grid {
    tiles: [[Tile; 10]; 20],
    salmon: Vec<Salmon>,
    pub instructions: Vec<Instruction>
}

impl Grid {
    pub fn load_level(level: &Level, instructions: Vec<Instruction>) -> Self {
        Self {
            tiles: level.tiles.clone(),
            salmon: level.salmon_starts.iter().map(|coords| Salmon { coords: coords.clone(), next_instruction: 0 }).collect(),
            instructions
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn tick(&mut self) -> bool {
        let mut remove_list = vec![];
        for i in 0..self.salmon.len() {
            let pos = self.salmon[i].coords;
            let mut next_instruction = self.salmon[i].next_instruction;
            let instruction = self.instructions[next_instruction].clone();
            let mut step = true;

            match instruction {
                Instruction::Move(dir) => {
                    let new_pos = pos.in_direction(dir);
                    if self.tiles[new_pos.y][new_pos.x] != Tile::Rock {
                        self.salmon[i].coords = new_pos;
                    }
                }
                Instruction::Goto(i) => {
                    next_instruction = i;
                    step = false;
                }
            }

            let pos = self.salmon[i].coords;
            if self.tiles[pos.y][pos.x] == Tile::Finish {
                remove_list.push(i);
            }
            
            if step { 
                next_instruction += 1;
            }
            self.salmon[i].next_instruction = if next_instruction < self.instructions.len() {
                next_instruction
            } else {
                0
            }
        }

        for i in remove_list.iter().rev() {
            self.salmon.remove(*i);
        }

        self.salmon.len() == 0
    }
}

impl canvas::Program<Message> for Grid {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let tile_size = (bounds.size().height / 20.).min(bounds.size().width / 10.);

        let mut frame = Frame::new(renderer, bounds.size());

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                frame.fill(&Path::circle(Point::new(x as f32 * tile_size + tile_size / 2., y as f32 * tile_size + tile_size / 2.), tile_size * 0.4), tile.color(theme));
            }
        }

        for salmon in &self.salmon {
            frame.fill(&Path::rectangle(Point::new(salmon.coords.x as f32 * tile_size + tile_size * 0.3, salmon.coords.y as f32 * tile_size + tile_size * 0.3), Size::new(tile_size * 0.4, tile_size * 0.4)), Color::from_rgb(0.8, 0.2, 0.2));
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Empty,
    Rock,
    Finish
}

impl Tile {
    fn color(&self, _theme: &Theme) -> Color {
        match self {
            Tile::Empty => Color::from_rgb(0.25, 0.25, 0.75),
            Tile::Rock => Color::from_rgb(0.5, 0.5, 0.5),
            Tile::Finish => Color::from_rgb(0.7, 0.7, 0.2)
        }
    }

    pub fn from_level_char(c: char) -> Option<Self> {
        match c {
            'e' => Some(Tile::Empty),
            'r' => Some(Tile::Rock),
            'f' => Some(Tile::Finish),
            _ => None
        }
    }
}

#[derive(Debug, Default)]
pub struct Salmon {
    coords: Coords,
    next_instruction: usize
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coords {
    pub x: usize,
    pub y: usize
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn in_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Coords::new(self.x, self.y.saturating_sub(1)),
            Direction::Down => Coords::new(self.x, (self.y + 1).min(19)),
            Direction::Left => Coords::new(self.x.saturating_sub(1), self.y),
            Direction::Right => Coords::new((self.x + 1).min(9), self.y),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
