
pub struct RandomGenerator {
    seed: u64,
}

impl RandomGenerator {
    pub fn new() -> Self {
        RandomGenerator {
            seed: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .expect("Failed to get system time")
                .as_millis() as u64,
        }
    }

    pub fn get_random_int(&mut self) -> i32 {
        self.seed = (self.seed.wrapping_mul(1103515245) + 12345) & 0x7fffffff;
        self.seed as i32
    }
}
