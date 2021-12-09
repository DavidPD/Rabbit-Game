use rand::prelude::*;

fn main() {
    let max_turns = 1000;
    let mut game = RabbitGame {
        upper_bound: 10,
        rabbit_position: 8,
    };
    let mut won = false;
    let mut turn = 0;
    let mut rng = thread_rng();

    let mut algo = RandoAlgo {};

    while !won && turn < max_turns
    {
        turn += 1;
        game.step(&mut rng);
        let check = algo.check(game.upper_bound, &mut rng);
        won = check == game.rabbit_position;
        println!("Turn {}, the rabbit moved to {}, and you checked {}", turn, game.rabbit_position, check);
    }
}

struct RandoAlgo {

}

impl Algo for RandoAlgo {
    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32 {
        rng.gen_range(0..upper_bound)
    }
}

struct RabbitGame {
    upper_bound: i32,
    rabbit_position: i32,
}

impl RabbitGame {
    pub fn step(&mut self, rng: &mut ThreadRng)
    {
        self.rabbit_position = if self.rabbit_position == self.upper_bound {
            self.rabbit_position - 1
        } else if self.rabbit_position == 0 {
            1
        } else {
            self.rabbit_position + [1, -1].iter().choose(rng).unwrap()
        }
    }
}

trait Algo {
    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32;
}
