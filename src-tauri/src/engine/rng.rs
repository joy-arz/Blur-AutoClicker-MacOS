use rand::SeedableRng;
use rand::Rng;

/// Fast small RNG using xoshiro128+
pub struct FastRng {
    rng: rand::rngs::SmallRng,
}

impl FastRng {
    pub fn new() -> Self {
        Self {
            rng: rand::rngs::SmallRng::from_entropy(),
        }
    }

    /// Returns a random f64 in [0, 1)
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        self.rng.r#gen()
    }

    /// Returns a Gaussian-distributed random number using Box-Muller transform
    #[inline]
    pub fn next_gaussian(&mut self, mean: f64, std_dev: f64) -> f64 {
        let u1 = self.rng.r#gen::<f64>();
        let u2 = self.rng.r#gen::<f64>();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        mean + std_dev * z0
    }
}

impl Default for FastRng {
    fn default() -> Self {
        Self::new()
    }
}
