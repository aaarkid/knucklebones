use crate::dice_game::{Board};
use ndarray::Array2;
use rand::{Rng, seq::SliceRandom};
use std::cmp::min;

pub struct QLearningAgent {
    q_table: Array2<f64>,
    learning_rate: f64,
    discount_factor: f64,
    exploration_rate: f64,
}

impl QLearningAgent {
    pub fn new(learning_rate: f64, discount_factor: f64, exploration_rate: f64) -> Self {
        let q_table = Array2::zeros((6 * 6 * 6 * 6, 3));
        QLearningAgent {
            q_table,
            learning_rate,
            discount_factor,
            exploration_rate,
        }
    }

    pub fn choose_action(&self, board: &Board, die_value: u8) -> usize {
        let state_index = Self::board_state_to_index(board, die_value);
        if rand::thread_rng().gen::<f64>() < self.exploration_rate {
            rand::thread_rng().gen_range(0..3)
        } else {
            let actions = self.q_table.row(state_index);
            actions
                .iter()
                .enumerate()
                .filter(|(_, &q_value)| q_value == actions.iter().cloned().fold(f64::MIN, f64::max))
                .map(|(index, _)| index)
                .collect::<Vec<_>>()
                .choose(&mut rand::thread_rng())
                .copied()
                .unwrap()
        }
    }

    pub fn update_q_value(
        &mut self,
        board: &Board,
        die_value: u8,
        action: usize,
        reward: f64,
        next_board: &Board,
        next_die_value: u8,
    ) {
        let state_index = Self::board_state_to_index(board, die_value);
        let next_state_index = Self::board_state_to_index(next_board, next_die_value);

        let current_q_value = self.q_table[[state_index, action]];
        let max_next_q_value = self.q_table.row(next_state_index).iter().cloned().fold(f64::MIN, f64::max);

        let new_q_value = current_q_value
            + self.learning_rate * (reward + self.discount_factor * max_next_q_value - current_q_value);

        self.q_table[[state_index, action]] = new_q_value;
    }

    fn board_state_to_index(board: &Board, die_value: u8) -> usize {
        let mut index = 0;
        let base = 6;
        for i in 0..3 {
            let column_size = min(board.columns[i].dice.len(), 3);
            index = index * base + column_size;
        }
        index = index * base + (die_value as usize - 1);
        index
    }
}