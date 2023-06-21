use knucklebones::{dqn_agent::*, dice_game::Game};

const NUM_EPISODES: usize = 1800;
const TRAIN_EVERY_N_TURNS: usize = 10;


fn main() {
    let mut player1 = KnucklebonesDqnAgent::new();
    let mut player2 = KnucklebonesDqnAgent::new();

    for i in 0..NUM_EPISODES {
        if i % 100 == 0 {
            println!("Episode {}", i);
        }
        let mut game = Game::new();
        game.current_player = rand::random::<usize>() % 2 + 1;
        let mut finished = false;
        let mut turns = 0;

        while !finished {
            turns += 1;
            match game.current_player {
                1 => {
                    let die = game.roll_die();
                    let state = initialize_state(&game.player1_board, &game.player2_board, die);
                    let action = player1.choose_column(state.view());
                    let (reward, done) = player1.calculate_reward(&game.player1_board, &game.player2_board, action, die);
                    game.place_die(action, die);
                    let next_state = initialize_state(&game.player1_board, &game.player2_board, 1);

                    player1.remember(state, action, reward, next_state, done);

                    if turns % TRAIN_EVERY_N_TURNS == 0 {
                        player1.replay();
                    }

                    finished = done;
                },
                2 => {
                    let die = game.roll_die();
                    let state = initialize_state(&game.player2_board, &game.player1_board, die);
                    let action = player2.choose_column(state.view());
                    let (reward, done) = player2.calculate_reward(&game.player2_board, &game.player1_board, action, die);
                    game.place_die(action, die);
                    let next_state = initialize_state(&game.player2_board, &game.player1_board, 1);

                    player2.remember(state, action, reward, next_state, done);

                    if turns % TRAIN_EVERY_N_TURNS == 0 {
                        player2.replay();
                    }

                    finished = done;
                },
                _ => {}
            }

            game.current_player = 3 - game.current_player;
        }

        // println!("Game finished after {} turns with scores: {} - {}", turns, game.player1_board.score(), game.player2_board.score());
    }

    // let mut game = Game::new();
    // game.current_player = rand::random::<usize>() % 2 + 1;
    // let mut finished = false;
    
    // //play one game and show every turn to see how much the agents learnt

    // while !finished {
    //     match game.current_player {
    //         1 => {
    //             let die = game.roll_die();
    //             let state = initialize_state(&game.player1_board, &game.player2_board, die);
    //             let action = player1.choose_column(state.view());
    //             let (reward, done) = player1.calculate_reward(&game.player1_board, &game.player2_board, action, die);
    //             game.place_die(action, die);
    //             let next_state = initialize_state(&game.player1_board, &game.player2_board, 1);

    //             player1.remember(state, action, reward, next_state, done);

    //             finished = done;
    //         },
    //         2 => {
    //             let die = game.roll_die();
    //             let state = initialize_state(&game.player2_board, &game.player1_board, die);
    //             let action = player2.choose_column(state.view());
    //             let (reward, done) = player2.calculate_reward(&game.player2_board, &game.player1_board, action, die);
    //             game.place_die(action, die);
    //             let next_state = initialize_state(&game.player2_board, &game.player1_board, 1);

    //             player2.remember(state, action, reward, next_state, done);

    //             finished = done;
    //         },
    //         _ => {}
    //     }

    //     game.current_player = 3 - game.current_player;
    //     println!("{}", game);
    // }

    //play a game: Human vs first agent

    let mut game = Game::new();
    game.current_player = rand::random::<usize>() % 2 + 1;
    let mut finished = false;

    while !finished {
        match game.current_player {
            1 => {
                let die = game.roll_die();
                let state = initialize_state(&game.player1_board, &game.player2_board, die);
                let action = player1.choose_column(state.view());
                let (reward, done) = player1.calculate_reward(&game.player1_board, &game.player2_board, action, die);
                game.place_die(action, die);
                let next_state = initialize_state(&game.player1_board, &game.player2_board, 1);

                player1.remember(state, action, reward, next_state, done);

                finished = done;
            },
            2 => {
                let die = game.roll_die();
                println!("Die: {}", die);
                let column_index = game.get_column_input(&game.player2_board);
                game.player2_board.place_die(column_index, knucklebones::dice_game::Die::new(die));
                game.player1_board.remove_dice(column_index, die);
                let done = game.player1_board.is_full() || game.player2_board.is_full();

                finished = done;
            },
            _ => {}
        }

        game.current_player = 3 - game.current_player;
        println!("{}", game);
    }

    game.display_final_boards();
    game.display_scores_and_winner();
}