use iced::{mouse, widget::{canvas::{self, Frame, Path}, Canvas}, Color, Element, Length, Point, Renderer, Size, Theme};
use rand::{Rng, thread_rng};

use crate::{app::Message, instruction::Instruction};

#[derive(Debug, Default)]
pub struct Grid {
    tiles: [[Tile; 10]; 20],
    salmon: Vec<Salmon>,
    pub instructions: Vec<Instruction>
}

impl Grid {
    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn tick(&mut self) {
        for i in 0..self.salmon.len() {
            let pos = (self.salmon[i].x, self.salmon[i].y);
            let mut next_instruction = self.salmon[i].next_instruction;
            let instruction = self.instructions[next_instruction].clone();

            match instruction {
                Instruction::Forwards => {
                    let new_pos = (pos.0, pos.1.saturating_sub(1));
                    if self.tiles[new_pos.1 as usize][new_pos.0 as usize] != Tile::Rock {
                        self.salmon[i].x = new_pos.0;
                        self.salmon[i].y = new_pos.1;
                    }
                },
                Instruction::Left => {
                    let new_pos = (pos.0.saturating_sub(1), pos.1);
                    if self.tiles[new_pos.1 as usize][new_pos.0 as usize] != Tile::Rock {
                        self.salmon[i].x = new_pos.0;
                        self.salmon[i].y = new_pos.1;
                    }
                },
                Instruction::Right => {
                    let new_pos = ((pos.0 + 1).min(9), pos.1);
                    if self.tiles[new_pos.1 as usize][new_pos.0 as usize] != Tile::Rock {
                        self.salmon[i].x = new_pos.0;
                        self.salmon[i].y = new_pos.1;
                    }
                },
                Instruction::Back => {
                    let new_pos = (pos.0, (pos.1 + 1).min(9));
                    if self.tiles[new_pos.1 as usize][new_pos.0 as usize] != Tile::Rock {
                        self.salmon[i].x = new_pos.0;
                        self.salmon[i].y = new_pos.1;
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
                self.salmon.push(Salmon { x: i as u32, y: 19, next_instruction: 0 })
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
            frame.fill(&Path::rectangle(Point::new(salmon.x as f32 * tile_size + tile_size * 0.3, salmon.y as f32 * tile_size + tile_size * 0.3), Size::new(tile_size * 0.4, tile_size * 0.4)), Color::from_rgb(0.8, 0.2, 0.2));
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Tile {
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
}

#[derive(Debug, Default)]
pub struct Salmon {
    x: u32,
    y: u32,
    next_instruction: usize
}