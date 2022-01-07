use rand::prelude::*;

use crate::rabbit_game::*;

pub struct RandoAlgo;

impl HuntingAlgorithm for RandoAlgo {
    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32 {
        rng.gen_range(0..upper_bound)
    }

    fn new() -> Box<dyn HuntingAlgorithm>
    where
        Self: Sized,
    {
        Box::new(RandoAlgo {})
    }
}

pub struct SweepAlgo {
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

    fn new() -> Box<dyn HuntingAlgorithm>
    where
        Self: Sized,
    {
        Box::new(SweepAlgo {
            current_position: 0,
        })
    }
}

pub struct OffsetSweep {
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

    fn new() -> Box<dyn HuntingAlgorithm>
    where
        Self: Sized,
    {
        Box::new(OffsetSweep {
            current_position: 0,
            start_even: true,
        })
    }
}

pub struct DoubleCheckZero {
    current_position: i32,
    has_repeated: bool,
}

impl HuntingAlgorithm for DoubleCheckZero {
    fn check(&mut self, upper_bound: i32, _rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;
        if self.current_position >= upper_bound {
            self.current_position = 0;
            self.has_repeated = false;
        } else if self.current_position == 0 && !self.has_repeated {
            self.current_position = 0;
            self.has_repeated = true;
        } else {
            self.current_position += 1;
        }
        return check;
    }

    fn new() -> Box<dyn HuntingAlgorithm>
    where
        Self: Sized,
    {
        Box::new(DoubleCheckZero {
            current_position: 0,
            has_repeated: true,
        })
    }
}

pub struct DoubleCheckMax {
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

    fn new() -> Box<dyn HuntingAlgorithm>
    where
        Self: Sized,
    {
        Box::new(DoubleCheckMax {
            current_position: 0,
            has_repeated: false,
        })
    }
}

pub struct ConditionalDoubleCheckMax {
    current_position: i32,
    has_repeated: bool,
}

impl HuntingAlgorithm for ConditionalDoubleCheckMax {
    fn check(&mut self, upper_bound: i32, _rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;
        let upper_is_even = upper_bound % 2 == 0;

        if upper_is_even {
            if self.current_position >= upper_bound {
                self.current_position = 0;
            } else {
                self.current_position = self.current_position + 1;
            }
        } else {
            if self.current_position >= upper_bound && self.has_repeated {
                self.current_position = 0;
                self.has_repeated = false;
            } else if self.current_position >= upper_bound {
                self.has_repeated = true;
            } else {
                self.current_position = self.current_position + 1;
            }
        }

        return check;
    }

    fn new() -> Box<dyn HuntingAlgorithm>
    where
        Self: Sized,
    {
        Box::new(ConditionalDoubleCheckMax {
            current_position: 0,
            has_repeated: false,
        })
    }
}
