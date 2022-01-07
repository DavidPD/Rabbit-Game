use rand::prelude::*;

pub const LOWER_BOUND: i32 = 1;

pub trait HuntingAlgorithm {
    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32;

    fn new() -> Box<dyn HuntingAlgorithm>
    where
        Self: Sized;
}

pub struct RabbitGame {
    pub max_turns: i32,
    pub upper_bound: i32,
    hunter: Box<dyn HuntingAlgorithm>,

    rabbit_position: i32,
    rng: ThreadRng,
}

pub struct GameResult {
    pub did_win: bool,
    pub num_turns: i32,
}

impl RabbitGame {
    pub fn new(hunter: Box<dyn HuntingAlgorithm>, upper_bound: i32, max_turns: i32) -> RabbitGame {
        let mut rng = thread_rng();

        RabbitGame {
            max_turns,
            upper_bound,
            hunter,
            rabbit_position: rng.gen_range(LOWER_BOUND..=upper_bound),
            // rabbit_position: (0..upper_bound).filter(|i| i%2 != 0).into_iter().choose(&mut rng).unwrap(), // pick an even number
            rng: rng,
        }
    }

    pub fn step(&mut self) {
        self.rabbit_position = if self.rabbit_position == self.upper_bound {
            self.rabbit_position - 1
        } else if self.rabbit_position == LOWER_BOUND {
            self.rabbit_position + 1
        } else {
            let direction = [1, -1].iter().choose(&mut self.rng).unwrap();
            self.rabbit_position + direction
        }
    }

    pub fn run(&mut self) -> GameResult {
        let mut won = false;
        let mut turn = 0;

        println!(
            "Starting game from {} in range 0..{}",
            self.rabbit_position, self.upper_bound
        );

        while !won && turn < self.max_turns {
            turn += 1;
            self.step();
            let check = self.hunter.check(self.upper_bound, &mut self.rng);

            assert!(check >= LOWER_BOUND && check <= self.upper_bound);

            won = check == self.rabbit_position;

            // println!("Turn {}, the rabbit moved to {}, and you checked {}", turn, self.rabbit_position, check);
        }

        GameResult {
            did_win: won,
            num_turns: turn,
        }
    }
}
