pub struct Field {
    prime: u32,
}

impl Field {
    pub fn new(prime: u32) -> Self {
        Self { prime }
    }

    pub fn add(&self, a: u32, b: u32) -> u32 {
        return (a + b) % self.prime;
    }

    pub fn multiply(&self, a: u32, b: u32) -> u32 {
        return (a * b) % self.prime;
    }

    pub fn power(&self, base: u32, exponent: u32) -> u32 {
        if exponent == 0 {
            return 1;
        } else if exponent % 2 == 0 {
            let half = self.power(base, exponent / 2);
            return self.multiply(half, half);
        }
        return self.multiply(base, self.power(base, exponent - 1));
    }

    pub fn inverse(&self, a: u32) -> u32 {
        return self.power(a, self.prime - 2);
    }
}
