use rand::prelude::*;

mod rabbit_game;
use rabbit_game::*;
mod rabbit_catching_algorithms;
use rabbit_catching_algorithms::*;

fn main() {
    let mut wins = 0;
    let mut results: Vec<i32> = vec![];

    let num_games = 1000;

    let mut rng = thread_rng();

    for _ in 0..num_games {
        let upper_bound = rng.gen_range(2..=100);
        let max_turns = upper_bound * 2;

        let result = play_game::<ShortStop>(upper_bound, max_turns);
        let win = result.did_win;
        results.push(result.num_turns);

        if win {
            wins += 1;
        }
    }

    println!("Wins: {}", wins);
    let average = results.iter().sum::<i32>() / results.len() as i32;
    println!("Average Turns: {}", average)
}

fn play_game<T: HuntingAlgorithm>(upper_bound: i32, max_turns: i32) -> GameResult {
    let hunter = T::new();

    let mut game = RabbitGame::new(hunter, upper_bound, max_turns);
    let result = game.run();

    if result.did_win {
        println!("You won in {} turns!!!", result.num_turns);
    } else {
        println!("You lost despite being given {} turns.", result.num_turns);
    }

    result
}
