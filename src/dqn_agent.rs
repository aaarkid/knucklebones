use ndarray::Array1;

use crate::dice_game::{Board, Die};

const LEARNING_RATE: f32 = 0.001;
const EPSILON: f32 = 0.1;
const GAMMA: f32 = 0.9;
const CAPACITY: usize = 1000;
const BATCH_SIZE: usize = 128;

pub fn initialize_state(own_board: &Board, opponent_board: &Board, die: u8) -> Array1<f32> {
    let mut state = vec![0.0; 2*3*3*6 + 6]; // create a zero-filled vector
    for (board_idx, board) in [own_board, opponent_board].iter().enumerate() {
        let base_board_idx = board_idx * 3 * 3 * 6;
        for (column_idx, column) in board.columns.iter().enumerate() {
            let base_idx = base_board_idx + column_idx * 18;
            for die in &column.dice {
                let value_idx = (die.value() as usize - 1) * 3;
                let mut slots_filled = 0;
                for i in 0..3 {
                    if state[base_idx + value_idx + i] == 0.0 {
                        state[base_idx + value_idx + i] = 1.0;
                        slots_filled += 1;
                        break;
                    }
                }
                assert!(slots_filled > 0, "Too many dice of the same value in a column");
            }
        }
    }
    // add the current die roll
    state[2*3*3*6 + (die as usize - 1)] = 1.0;
    Array1::from(state)
}

use athena::{optimizer::OptimizerWrapper, optimizer::SGD, agent::DqnAgent, replay_buffer::ReplayBuffer, replay_buffer::Experience};
use rand::rngs::ThreadRng;
use ndarray::ArrayView1;

pub struct KnucklebonesDqnAgent {
    agent: DqnAgent,
    replay_buffer: ReplayBuffer,
    #[allow(dead_code)]
    rng: ThreadRng,
}

impl KnucklebonesDqnAgent {
    pub fn new() -> Self {
        let layer_sizes = &[2*3*3*6 + 6, 128, 3];
        let optimizer = OptimizerWrapper::SGD(SGD::new());
        let agent = DqnAgent::new(layer_sizes, EPSILON, optimizer);

        let replay_buffer = ReplayBuffer::new(CAPACITY);
        let rng = ThreadRng::default();

        KnucklebonesDqnAgent { agent, replay_buffer, rng }
    }

    pub fn choose_column(&mut self, state: ArrayView1<f32>) -> usize {
        self.agent.act(state)
    }

    pub fn calculate_reward(&self, own_board: &Board, opponent_board: &Board, column_index: usize, die: u8) -> (f32, bool) {
        let own_score = own_board.score();
        let opponent_score = opponent_board.score();
        let mut own_board = own_board.clone();
        let mut opponent_board = opponent_board.clone();
        own_board.place_die(column_index, Die::new(die));
        opponent_board.remove_dice(column_index, die);
        let own_score_after = own_board.score();
        let opponent_score_after = opponent_board.score();
        let reward;
        if own_score == own_score_after {
            return (-2., false);
        } else {
            reward = (own_score_after - own_score + (opponent_score - opponent_score_after) * 2) as f32 / 100.;
        }
        let done = own_board.is_full() || opponent_board.is_full();
        if done {
            if own_score_after > opponent_score_after {
                return (5., done);
            } else if own_score_after < opponent_score_after {
                return (-5., done);
            } else {
                return (0., done);
            }
        }

        (reward, done)
    }

    pub fn remember(&mut self, state: Array1<f32>, action: usize, reward: f32, next_state: Array1<f32>, done: bool) {
        self.replay_buffer.add(Experience {
            state,
            action,
            reward,
            next_state,
            done,
        });
    }

    pub fn replay(&mut self) {
        let batch = self.replay_buffer.sample(BATCH_SIZE);
        self.agent.train_on_batch(&batch, GAMMA, LEARNING_RATE);
    }
}
