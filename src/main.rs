use rand::prelude::*;

mod game_runner;
use game_runner::*;

fn main() {
    let max_turns = 100000;
    let upper_bound = 10000;

    let mut wins = 0;
    let mut results: Vec<i32> = vec!();

    let num_games = 10;

    for _ in 0..num_games {
        let result = play_game::<OffsetSweep>(upper_bound, max_turns);
        let win = result.did_win;
        results.push(result.num_turns);

        if win {
            wins += 1;
        }
    }

    println!("Wins: {}", wins);
    println!("Average Turns: {}", results.iter().sum::<i32>() / num_games)
}

fn play_game<T: HuntingAlgorithm>(upper_bound: i32, max_turns: i32) -> GameResult
{
    let hunter = T::new();

    let mut game = GameRunner::new(hunter, upper_bound, max_turns);
    let result = game.run();

    if result.did_win {
        println!("You won in {} turns!!!", result.num_turns);
    } else {
        println!("You lost despite being given {} turns.", result.num_turns);
    }

    result
}

struct RandoAlgo;

impl HuntingAlgorithm for RandoAlgo {

    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32 {
        rng.gen_range(0..upper_bound)
    }

    fn new() -> Box<dyn HuntingAlgorithm> where Self: Sized {
        Box::new(RandoAlgo {})
    }
}

struct SweepAlgo {
    current_position: i32,
}

impl HuntingAlgorithm for SweepAlgo {
    fn check(&mut self, upper_bound: i32, _rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;
        self.current_position += 1;
        if self.current_position >= upper_bound {
            self.current_position = 0;
        }
        return check;
    }

    fn new() -> Box<dyn HuntingAlgorithm> where Self: Sized {
        Box::new(SweepAlgo { current_position: 0 })
    }
}

struct OffsetSweep {
    current_position: i32,
    start_even: bool,
}

impl HuntingAlgorithm for OffsetSweep {
    fn check(&mut self, upper_bound: i32, _rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;
        self.current_position += 1;
        if self.current_position >= upper_bound {
            if self.start_even {
                self.current_position = 0;
            } else {
                self.current_position = 1;
            }
            self.start_even = !self.start_even;
        }
        return check;
    }

    fn new() -> Box<dyn HuntingAlgorithm> where Self: Sized {
        Box::new(OffsetSweep { current_position: 0, start_even: true})
    }
}
