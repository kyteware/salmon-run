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

    pub fn tick(&mut self) {
        for i in 0..self.salmon.len() {
            let pos = self.salmon[i].coords;
            let mut next_instruction = self.salmon[i].next_instruction;
            let instruction = self.instructions[next_instruction].clone();

            match instruction {
                Instruction::Forwards => {
                    let new_pos = pos.forwards();
                    if self.tiles[new_pos.y][new_pos.x] != Tile::Rock {
                        self.salmon[i].coords = new_pos;
                    }
                },
                Instruction::Left => {
                    let new_pos = pos.left();
                    if self.tiles[new_pos.y][new_pos.x] != Tile::Rock {
                        self.salmon[i].coords = new_pos;
                    }
                },
                Instruction::Right => {
                    let new_pos = pos.right();
                    if self.tiles[new_pos.y][new_pos.x] != Tile::Rock {
                        self.salmon[i].coords = new_pos;
                    }
                },
                Instruction::Backwards => {
                    let new_pos = pos.backwards();
                    if self.tiles[new_pos.y][new_pos.x] != Tile::Rock {
                        self.salmon[i].coords = new_pos;
                    }
                },
            }
            next_instruction += 1;
            self.salmon[i].next_instruction = if next_instruction < self.instructions.len() {
                next_instruction
            } else {
                0
            }
        }
    }

    pub fn shuffle(&mut self) {
        let mut tiles = <[[Tile; 10]; 20]>::default();
        for i in 0..20 {
            for j in 0..10 {
                if thread_rng().gen_bool(0.3) {
                    tiles[i][j] = Tile::Rock
                }
            }
        }
        self.tiles = tiles;
    }

    pub fn reset_salmon(&mut self) {
        self.salmon.clear();
        for i in 0..10 {
            if self.tiles[19][i] == Tile::Empty {
                self.salmon.push(Salmon { coords: Coords::new(i, 19), next_instruction: 0 })
            }
        }
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
    Rock
}

impl Tile {
    fn color(&self, _theme: &Theme) -> Color {
        match self {
            Tile::Empty => Color::from_rgb(0.25, 0.25, 0.75),
            Tile::Rock => Color::from_rgb(0.5, 0.5, 0.5),
        }
    }

    pub fn from_level_char(c: char) -> Option<Self> {
        match c {
            'e' => Some(Tile::Empty),
            'r' => Some(Tile::Rock),
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

    pub fn forwards(self) -> Self {
        Coords::new(self.x, self.y.saturating_sub(1))
    }

    pub fn backwards(self) -> Self {
        Coords::new(self.x, (self.y + 1).min(19))
    }

    pub fn left(self) -> Self {
        Coords::new(self.x.saturating_sub(1), self.y)
    }

    pub fn right(self) -> Self {
        Coords::new((self.x + 1).min(9), self.y)
    }
}
