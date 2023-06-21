use rand::Rng;
use std::{io, fmt::{Display, Formatter}};

const BOARD_SIZE: usize = 3;
const MAX_DICE_IN_COLUMN: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct Die {
    value: u8,
}

impl Die {
    pub fn new(value: u8) -> Self {
        Die { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub dice: Vec<Die>,
}

impl Column {
    pub fn new() -> Self {
        Column { dice: Vec::new() }
    }

    pub fn is_full(&self) -> bool {
        self.dice.len() >= MAX_DICE_IN_COLUMN
    }

    pub fn add_die(&mut self, die: Die) {
        if !self.is_full() {
            self.dice.push(die);
        }
    }

    pub fn remove_dice(&mut self, value: u8) {
        self.dice.retain(|d| d.value() != value);
    }

    pub fn score(&self) -> u16 {
        let mut value_counts = [0u8; 6];
        for die in &self.dice {
            value_counts[(die.value() - 1) as usize] += 1;
        }

        let mut score = 0;
        for (value, count) in value_counts.iter().enumerate() {
            score += (value as u16 + 1) * *count as u16 * *count as u16;
        }
        score
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub columns: Vec<Column>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            columns: vec![Column::new(); BOARD_SIZE],
        }
    }

    pub fn is_full(&self) -> bool {
        self.columns.iter().all(|col| col.is_full())
    }

    pub fn place_die(&mut self, column_index: usize, die: Die) {
        if let Some(column) = self.columns.get_mut(column_index) {
            column.add_die(die);
        }
    }

    pub fn remove_dice(&mut self, column_index: usize, value: u8) {
        if let Some(column) = self.columns.get_mut(column_index) {
            column.remove_dice(value);
        }
    }

    pub fn score(&self) -> u16 {
        self.columns.iter().map(|col| col.score()).sum()
    }
}

pub struct Game {
    pub player1_board: Board,
    pub player2_board: Board,
    pub current_player: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player1_board: Board::new(),
            player2_board: Board::new(),
            current_player: 1,
        }
    }

    pub fn play(&mut self) {
        while !self.player1_board.is_full() && !self.player2_board.is_full() {
            self.display_boards();

            println!("Player {}'s turn", self.current_player);
            let die_value = self.roll_die();
            println!("Player {} rolled a {}", self.current_player, die_value);

            if self.current_player == 1 {
                let column_index = self.get_column_input(&self.player1_board);
                self.player1_board.place_die(column_index, Die::new(die_value));
                self.player2_board.remove_dice(column_index, die_value);
            } else {
                let column_index = self.get_column_input(&self.player2_board);
                self.player2_board.place_die(column_index, Die::new(die_value));
                self.player1_board.remove_dice(column_index, die_value);
            }

            self.current_player = 3 - self.current_player;
        }

        self.display_final_boards();
        self.display_scores_and_winner();
    }

    pub fn roll_die(&self) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=6)
    }

    pub fn place_die(&mut self, column_index: usize, die: u8) {
        if self.current_player == 1 {
            self.player1_board.place_die(column_index, Die::new(die));
            self.player2_board.remove_dice(column_index, die);
        } else {
            self.player2_board.place_die(column_index, Die::new(die));
            self.player1_board.remove_dice(column_index, die);
        }
    }

    pub fn get_column_input(&self, board: &Board) -> usize {
        loop {
            println!("Enter the column index (0-2) where you want to place the die:");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");

            match input.trim().parse::<usize>() {
                Ok(index) if index < 3 && !board.columns[index].is_full() => return index,
                _ => println!("Invalid or full column index, please try again."),
            }
        }
    }

    fn display_boards(&self) {
        println!("Player 1's board:");
        self.print_board(&self.player1_board);
        println!("Player 2's board:");
        self.print_board(&self.player2_board);
    }

    pub fn display_final_boards(&self) {
        println!("Final boards:");
        self.display_boards();
    }

    pub fn display_scores_and_winner(&self) {
        let player1_score = self.player1_board.score();
        let player2_score = self.player2_board.score();

        println!("Player 1's score: {}", player1_score);
        println!("Player 2's score: {}", player2_score);

        if player1_score > player2_score {
            println!("Player 1 wins!");
        } else if player2_score > player1_score {
            println!("Player 2 wins!");
        } else {
            println!("It's a tie!");
        }
    }

    fn print_board(&self, board: &Board) {
        for row in 0..3 {
            for col in 0..3 {
                let column = &board.columns[col];
                let die_value = if row < column.dice.len() {
                    column.dice[row].value()
                } else {
                    0
                };
                print!("{} ", die_value);
            }
            println!();
        }
        println!("Scores: {:?}", board.columns.iter().map(|col| col.score()).collect::<Vec<u16>>());
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            for col in 0..3 {
                let column = &self.columns[col];
                let die_value = if row < column.dice.len() {
                    column.dice[row].value()
                } else {
                    0
                };
                write!(f, "{} ", die_value)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Current player: {}\n", self.current_player)?;
        write!(f, "Player 1's board:\n{}", self.player1_board)?;
        write!(f, "Player 1's Score: {}\n", self.player1_board.score())?;
        write!(f, "Player 2's board:\n{}", self.player2_board)?;
        write!(f, "Player 2's Score: {}\n", self.player2_board.score())?;
        Ok(())
    }
}