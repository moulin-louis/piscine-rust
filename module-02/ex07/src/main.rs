use ftkit::{random_number, ARGS};
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq)]
enum ParseError {
    InvalidWidth { arg: &'static str },
    InvalidHeight { arg: &'static str },
    InvalidPercentage { arg: &'static str },
    TooManyArguments,
    NotEnoughArguments,
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn is_alive(self) -> bool {
        match self {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
    fn is_dead(self) -> bool {
        match self {
            Cell::Dead => true,
            Cell::Alive => false,
        }
    }
}

#[derive(Clone, PartialEq)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize, percentage: u32) -> Self {
        let mut cells_vec: Vec<Cell> = Vec::new();
        cells_vec.reserve(width * height);
        let needed_alive_cell: f32 = (percentage as f32 / 100.0) * (width * height) as f32;
        for _idx in 0..(width * height) {
            cells_vec.push(Cell::Dead);
        }
        for _idx in 0..(needed_alive_cell as usize) {
            let x: usize = random_number(0..width as i32) as usize;
            let y: usize = random_number(0..height as i32) as usize;
            cells_vec[y * width + x] = Cell::Alive;
        }
        Board {
            width,
            height,
            cells: cells_vec,
        }
    }

    fn from_args() -> Result<Self, ParseError> {
        if ARGS.len() < 4 {
            return Err(ParseError::NotEnoughArguments);
        }
        if ARGS.len() > 4 {
            return Err(ParseError::TooManyArguments);
        }
        let width: usize = match ARGS[1].parse() {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {e}");
                return Err(ParseError::InvalidWidth { arg: &ARGS[1] });
            }
        };
        let height: usize = match ARGS[2].parse() {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {e}");
                return Err(ParseError::InvalidHeight { arg: &ARGS[2] });
            }
        };
        let percentage: u32 = match ARGS[3].parse() {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {e}");
                return Err(ParseError::InvalidPercentage { arg: &ARGS[3] });
            }
        };
        Ok(Board::new(width, height, percentage))
    }

    fn get_wrapped_index(row: isize, column: isize, width: usize, height: usize) -> usize {
        let wrapped_row = (row + height as isize) as usize % height;
        let wrapped_column = (column + width as isize) as usize % width;
        wrapped_row * width + wrapped_column
    }

    fn get_nbr_neighbours(&self, _cell: &Cell, idx: usize) -> usize {
        let row = idx / self.width;
        let col = idx % self.width;
        let mut result = 0;
        for &r in [-1, 0, 1].iter() {
            for &c in [-1, 0, 1].iter() {
                if r == 0 && c == 0 {
                    continue;
                }
                let neighbor_idx = Board::get_wrapped_index(
                    row as isize + r,
                    col as isize + c,
                    self.width,
                    self.height,
                );
                if self.cells[neighbor_idx].is_alive() {
                    result += 1;
                }
            }
        }
        result
    }

    fn step(&mut self) {
        let mut to_toogle: Vec<usize> = Vec::new();
        for (idx, cell) in self.cells.iter().enumerate() {
            let nbr_neighbours: usize = self.get_nbr_neighbours(cell, idx);
            if (cell.is_alive() && !(2..=3).contains(&nbr_neighbours))
                || (cell.is_dead() && nbr_neighbours == 3)
            {
                to_toogle.push(idx);
            }
        }
        for idx in to_toogle {
            self.cells[idx] = match self.cells[idx] {
                Cell::Dead => Cell::Alive,
                Cell::Alive => Cell::Dead,
            }
        }
    }

    fn print(&self, clear: bool) {
        if clear {
            println!("\x1B[{}A", self.height + 1);
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells[y * self.width + x];
                if cell.is_alive() {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut board = match Board::from_args() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };
    let mut clean: bool = false;
    loop {
        board.print(clean);
        clean = true;
        board.step();
        let mut all_dead = true;
        for cell in &board.cells {
            if cell.is_alive() {
                thread::sleep(Duration::from_secs(1));
                all_dead = false;
                break;
            }
        }
        if all_dead {
            break;
        }
    }
}
