use rand::prelude::*;

mod game_runner;
use game_runner::*;

fn main() {
    let max_turns = 1000;
    let upper_bound = 10;

    let algo = SweepAlgo { current_position: 0 };

    let mut game = GameRunner::new(Box::new(algo), upper_bound, max_turns);
    let result = game.run();

    if result.did_win {
        println!("You won in {} turns!!!", result.num_turns);
    } else {
        println!("You lost despite being given {} turns.", result.num_turns);
    }
}

struct RandoAlgo;

impl HuntingAlgorithm for RandoAlgo {
    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32 {
        rng.gen_range(0..upper_bound)
    }
}

struct SweepAlgo {
    current_position: i32,
}

impl HuntingAlgorithm for SweepAlgo {
    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;
        self.current_position += 1;
        if self.current_position >= upper_bound {
            self.current_position = 0;
        }
        return check;
    }
}
