use rand::prelude::*;

pub fn d20() -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(1..=20)
}
