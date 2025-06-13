use std::{fmt::{Display, Formatter}, io, ops::{Index, IndexMut}};
use colored::{Colorize, ColoredString};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const WIN_NUM: usize = 4;
const P: char = 'O';

type Grid = [[Option<bool>; HEIGHT]; WIDTH];

#[derive(Default, Debug)]
struct Board(Grid);

fn main() {
    let mut board = Board::default();

    let mut player = true;

    println!("Connect 4!");
    println!("Player 1 is playing as {}", p_name(true));
    println!("Player 2 is playing as {}", p_name(false));
    println!("To place your piece, write the number you see in the column where you want to place it");


    loop {
        println!("It's `{}`'s turn.", p_name(player));
        println!("{}", board);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: usize = match input.trim().parse::<usize>() {
            // Works because if the cond is false && _, the second is omitted (so doesn't panic)
            Ok(num) if num > 0 && num <= WIDTH => num - 1,
            _ => {
                eprintln!("That's not a valid number.");
                continue;
            }
        };
        if !board.in_bounds(input) {
            eprintln!("Column is full");
            continue;
        }
        match board[input] {
            Some(_) => {
                eprintln!("Column is full");
                continue;
            }
            None => {
                board[input] = Some(player);
            }
        }

        if board.has_won(input % WIDTH) {
            println!("{}", board);
            println!("`{}` player won!", p_name(player));
            break;
        }
        if board.is_full() {
            println!("It's a tie!");
            break;
        }

        player = !player;
    }
}

#[inline]
fn p_colored(turn: bool) -> ColoredString {
    if turn {
        P.to_string().bright_yellow().bold()
    } else {
        P.to_string().bright_red().bold()
    }
}
#[inline]
fn p_name(turn: bool) -> ColoredString {
    if turn { "yellow".bright_yellow().bold() } else { "red".bright_red().bold() }
}

impl Board {
    #[inline]
    fn iter(&self) -> impl Iterator<Item= &[Option<bool>; HEIGHT]> {
        self.0.iter()
    }
    #[inline]
    fn is_full(&self) -> bool {
        self.iter().all(|col| col.iter().all(Option::is_some))
    }

    fn has_won(&self, x: usize) -> bool {
        let (y, turn) = self.0[x]
            .iter()
            .enumerate()
            .find(|cell| cell.1.is_some())
            .unwrap(); // if `has_won` is called this is surely Some
        let turn = turn.unwrap(); // same reason
        
        let all_same = |cell: &Option<bool>| matches!(cell, Some(v) if *v == turn);

        // cols
        for offset in 0..WIN_NUM {
            if y + offset + 1 < WIN_NUM {
                continue;
            }
            let vec: Vec<Option<bool>> =
                (0..WIN_NUM).filter_map(|i| self.0[x].get(y + offset - i))
                    .copied()
                    .collect();
            let Ok(arr): Result<[Option<bool>; 4], _> = vec.try_into() else {
                continue;
            };

            if arr.iter().all(all_same) {
                return true;
            }
        }
        // rows
        for offset in 0..WIN_NUM {
            if x + offset + 1 < WIN_NUM {
                continue;
            }
            let vec: Vec<Option<bool>> =
                (0..WIN_NUM).map(|i| self.0.get(x + offset - i))
                    .filter(|cell| cell.is_some())
                    .map(|cell| cell.unwrap()[y])
                    .collect();

            let Ok(arr): Result<[Option<bool>; 4], _> = vec.try_into() else {
                continue;
            };
            if arr.iter().all(all_same) {
                return true;
            }
        }
        // down diagonals
        for offset in 0..WIN_NUM {
            if x + offset + 1 < WIN_NUM || y + offset + 1 < WIN_NUM {
                continue;
            }
            let vec: Vec<Option<bool>> =
                (0..WIN_NUM).map(|i| self.0.get(x + offset - i))
                    .filter(|cell| cell.is_some())
                    .enumerate()
                    .filter_map(|(i, cell)| cell.unwrap().get(y + offset - i))
                    .copied()
                    .collect();

            let Ok(arr): Result<[Option<bool>; 4], _> = vec.try_into() else {
                continue;
            };
            if arr.iter().all(all_same) {
                return true;
            }
        }
        // up-diagonals
        for offset in 0..WIN_NUM {
            if x + offset + 1 < WIN_NUM || y < offset {
                continue;
            }
            let vec: Vec<Option<bool>> =
                (0..WIN_NUM).map(|i| self.0.get(x + offset - i))
                    .filter(|cell| cell.is_some())
                    .enumerate()
                    .filter_map(|(i, cell)| cell.unwrap().get(y - offset + i))
                    .copied()
                    .collect();

            let Ok(arr): Result<[Option<bool>; 4], _> = vec.try_into() else {
                continue;
            };
            if arr.iter().all(all_same) {
                return true;
            }
        }
        false
    }
    #[inline]
    const fn in_bounds(&self, col: usize) -> bool {
        self.0[col][0].is_none()
    }

    fn get_y(&self, x: usize) -> usize {
        self
            .iter()
            .nth(x)
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.is_none())
            .next_back()
            .unwrap().0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // ---------------------------------------
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.0[x][y] {
                    Some(player) => write!(f, "{}", p_colored(player))?,
                    None => write!(f, "{}", x + 1)?,
                }
                if x + 1 != WIDTH {
                    write!(f, " | ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<usize> for Board {
    type Output = Option<bool>;
    /// before indexing the board check that that column has at least 1 space empty.
    fn index(&self, x: usize) -> &Self::Output {
        let y = self.get_y(x);
        &self.0[x][y]
    }
}
impl IndexMut<usize> for Board {
    /// before indexing the board check that that column has at least 1 space empty.
    fn index_mut(&mut self, x: usize) -> &mut Self::Output {
        let y = self.get_y(x);
        &mut self.0[x][y]
    }
}
