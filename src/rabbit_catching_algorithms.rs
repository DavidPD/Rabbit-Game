use rand::prelude::*;

use crate::rabbit_game::*;

pub struct RandoAlgo;

impl HuntingAlgorithm for RandoAlgo {
    fn check(&mut self, upper_bound: i32, rng: &mut ThreadRng) -> i32 {
        rng.gen_range(LOWER_BOUND..=upper_bound)
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        RandoAlgo {}
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
            self.current_position = LOWER_BOUND;
        }

        return check;
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        SweepAlgo {
            current_position: LOWER_BOUND,
        }
    }
}

pub struct OffsetSweep {
    current_position: i32,
    start_low: bool,
}

impl HuntingAlgorithm for OffsetSweep {
    fn check(&mut self, upper_bound: i32, _rng: &mut ThreadRng) -> i32 {
        let check = self.current_position;
        self.current_position += 1;
        if self.current_position >= upper_bound {
            if self.start_low {
                self.current_position = LOWER_BOUND;
            } else {
                self.current_position = LOWER_BOUND + 1;
            }
            self.start_low = !self.start_low;
        }
        return check;
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        OffsetSweep {
            current_position: LOWER_BOUND,
            start_low: true,
        }
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
            self.current_position = LOWER_BOUND;
            self.has_repeated = false;
        } else if self.current_position == LOWER_BOUND && !self.has_repeated {
            self.current_position = LOWER_BOUND;
            self.has_repeated = true;
        } else {
            self.current_position += 1;
        }
        return check;
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        DoubleCheckZero {
            current_position: LOWER_BOUND,
            has_repeated: true,
        }
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
            self.current_position = LOWER_BOUND;
            self.has_repeated = false;
        } else if self.current_position >= upper_bound {
            self.has_repeated = true;
        } else {
            self.current_position = self.current_position + 1;
        }

        return check;
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        DoubleCheckMax {
            current_position: LOWER_BOUND,
            has_repeated: false,
        }
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

        if !upper_is_even {
            if self.current_position >= upper_bound {
                self.current_position = LOWER_BOUND;
            } else {
                self.current_position = self.current_position + 1;
            }
        } else {
            if self.current_position >= upper_bound && self.has_repeated {
                self.current_position = LOWER_BOUND;
                self.has_repeated = false;
            } else if self.current_position >= upper_bound {
                self.has_repeated = true;
            } else {
                self.current_position = self.current_position + 1;
            }
        }

        return check;
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        ConditionalDoubleCheckMax {
            current_position: LOWER_BOUND,
            has_repeated: false,
        }
    }
}
