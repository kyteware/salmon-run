use iced::{widget::{column, row, text}, Element};

#[derive(Debug, Default)]
pub struct SalmonRun {
    grid: [[Tile; 10]; 10]
}

impl SalmonRun {
    pub fn update(&mut self, message: Message) {

    }

    pub fn view(&self) -> Element<Message> {
        let mut rows = column!().spacing(10);
        for tile_row in &self.grid {
            let mut row = row!().spacing(10);
            for tile in tile_row {
                row = row.push(text(tile.text()))
            }
            rows = rows.push(row)
        }
        rows.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

#[derive(Debug, Default)]
pub enum Tile {
    #[default]
    Water,
    Rock,
    Salmon
}

impl Tile {
    fn text(&self) -> &'static str {
        match self {
            Tile::Water => "-water-",
            Tile::Rock => "--rock-",
            Tile::Salmon => "-salmon",
        }
    }
}