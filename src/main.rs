use rand::prelude::*;

mod game_runner;
use game_runner::*;

fn main() {
    let upper_bound = 99;
    let max_turns = upper_bound * 3;

    let mut wins = 0;
    let mut results: Vec<i32> = vec!();

    let num_games = 1000;

    for _ in 0..num_games {
        let result = play_game::<DoubleCheckMax>(upper_bound, max_turns);
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

struct DoubleCheckZero {
    current_position: i32,
    has_repeated: bool,
}

impl HuntingAlgorithm for DoubleCheckZero {
    fn check(&mut self, upper_bound: i32, _rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;
        if self.current_position >= upper_bound {
            self.current_position = 0;
            self.has_repeated = false;
        }
        else if self.current_position == 0 && !self.has_repeated {
            self.current_position = 0;
            self.has_repeated = true;
        }
        else {
            self.current_position += 1;
        }
        return check;
    }

    fn new() -> Box<dyn HuntingAlgorithm> where Self: Sized {
        Box::new(DoubleCheckZero { current_position: 0, has_repeated: true})
    }
}

struct DoubleCheckMax {
    current_position: i32,
    has_repeated: bool,
}

impl HuntingAlgorithm for DoubleCheckMax {
    fn check(&mut self, upper_bound: i32, _rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;

        if self.current_position >= upper_bound && self.has_repeated {
            self.current_position = 0;
            self.has_repeated = false;
        } else if self.current_position >= upper_bound {
            self.has_repeated = true;
        } else {
            self.current_position = self.current_position + 1;
        }

        return check;
    }

    fn new() -> Box<dyn HuntingAlgorithm> where Self: Sized {
        Box::new(DoubleCheckMax { current_position: 0, has_repeated: false})
    }
}
