pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone)]
pub struct Grid {
    pub buffer: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Grid {
    pub fn from_input(input: &str) -> Self {
        let buffer: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self {
            height: buffer.len(),
            width: buffer[0].len(),
            buffer,
        }
    }

    pub fn char_at(&self, position: &Vec2) -> char {
        self.buffer[position.y as usize][position.x as usize]
    }

    pub fn get_word(&self, start_from: &Vec2, direction: &Direction, length: usize) -> Vec<char> {
        let offset = Vec2::from(direction);
        let mut current_position = start_from.to_owned();
        let mut chars = vec![];
        for _ in 0..length {
            if self.is_valid(&current_position) {
                chars.push(self.char_at(&current_position));
            }
            current_position = current_position.add(&offset);
        }
        chars
    }

    pub fn find_word(&self, word: &Vec<char>) -> usize {
        let mut count: usize = 0;
        let directions = [
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];
        let word_length = word.len();
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                let point = Vec2 { x, y };
                if self.char_at(&point) == word[0] {
                    for direction in directions {
                        if self.get_word(&point, &direction, word_length) == *word {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn is_valid(&self, point: &Vec2) -> bool {
        point.y >= 0 && point.x >= 0 && point.y < self.height as i32 && point.x < self.width as i32
    }

    pub fn is_valid_subgrid(&self, point: &Vec2, radius: &usize) -> bool {
        point.y >= *radius as i32
            && point.x >= *radius as i32
            && point.y + (*radius as i32) < self.height as i32
            && point.x + (*radius as i32) < self.width as i32
    }

    pub fn get_subgrid(&self, point: &Vec2, radius: usize) -> Option<Self> {
        if self.is_valid_subgrid(point, &radius) {
            let mut subgrid = vec![];
            for y in (point.y - radius as i32)..=(point.y + radius as i32) {
                subgrid.push(vec![]);
                for x in (point.x - radius as i32)..=(point.x + radius as i32) {
                    subgrid
                        .last_mut()?
                        .push(self.buffer[y as usize][x as usize])
                }
            }
            return Some(Self {
                height: subgrid.len(),
                width: subgrid[0].len(),
                buffer: subgrid,
            });
        }
        None
    }

    pub fn find_cross_word(&self, word: &Vec<char>) -> usize {
        let radius = (word.len() - 1) / 2;
        let length = word.len() as i32 - 1;
        let middle_letter = word[radius];
        let mut count = 0;
        for x in radius as i32..(self.width - radius) as i32 {
            for y in radius as i32..(self.height - radius) as i32 {
                let point = Vec2 { x, y };
                if self.char_at(&point) != middle_letter {
                    continue;
                }
                let Some(subgrid) = self.get_subgrid(&point, radius) else {
                    continue;
                };
                if (subgrid.get_word(&Vec2::new(0, 0), &Direction::DownRight, word.len()) == *word
                    || subgrid.get_word(&Vec2::new(length, length), &Direction::UpLeft, word.len())
                        == *word)
                    && (subgrid.get_word(&Vec2::new(length, 0), &Direction::DownLeft, word.len())
                        == *word
                        || subgrid.get_word(&Vec2::new(0, length), &Direction::UpRight, word.len())
                            == *word)
                {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Vec2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<&Direction> for Vec2 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Right => Self { x: 1, y: 0 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Up => Self { x: 0, y: -1 },
            Direction::Down => Self { x: 0, y: 1 },
            Direction::UpRight => Self { x: 1, y: -1 },
            Direction::UpLeft => Self { x: -1, y: -1 },
            Direction::DownRight => Self { x: 1, y: 1 },
            Direction::DownLeft => Self { x: -1, y: 1 },
        }
    }
}
