use std::{fs::{self, File}, io::Read, path::PathBuf};

use crate::grid::{Coords, Tile};

#[derive(Debug, Default)]
pub struct Level {
    pub number: u32,
    pub tiles: [[Tile; 10]; 20],
    pub salmon_starts: Vec<Coords>
}

impl Level {
    fn load(path: PathBuf) -> Option<Level> {
        let mut file = File::open(&path).ok()?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).ok()?;
        Level::interpret(&buf)
    }

    fn interpret(i: &str) -> Option<Level> {
        let chars = i.chars().collect::<Vec<char>>();
        let mut level = Level::default();
        for line in 0..20 {
            for col in 0..10 {
                let c = chars[line * 11 + col];
                if c == 's' {
                    level.salmon_starts.push(Coords::new(col, line));
                    level.tiles[line][col] = Tile::Empty;
                } else if let Some(tile) = Tile::from_level_char(c) {
                    level.tiles[line][col] = tile;
                } else {
                    return None
                }
            }
            if chars[(line + 1) * 11 - 1] != '\n' {
                return None;
            }
        }

        level.number = (&chars[220..]).iter().collect::<String>().parse::<u32>().ok()?;

        Some(level)
    }

    pub fn load_all() -> Vec<Level> {
        let path_list = fs::read_dir("./levels").unwrap();
        let mut levels = vec![];
        for path in path_list {
            if let Ok(path) = path.map(|x| x.path()) {
                if let Some(Some(filename)) = path.file_name().map(|x| x.to_str()) {
                    if filename.starts_with("level") && filename.ends_with(".txt") {
                        if let Some(level) = Level::load(path) {
                            levels.push(level);
                        }
                    }
                }
            }
        }

        levels
    }
}