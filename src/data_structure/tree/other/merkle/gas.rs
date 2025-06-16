/*!
Gas
*/

#[derive(Debug)]
pub struct GasMeter {
    limit: u64,
    used: u64,
}

impl GasMeter {
    pub fn new(limit: u64) -> Self {
        Self { limit, used: 0 }
    }

    pub fn consume(&mut self, amount: u64) -> Result<(), String> {
        if self.used + amount > self.limit {
            return Err("Out of gas".to_string());
        }

        self.used += amount;
        Ok(())
    }

    pub fn remaining(&self) -> u64 {
        self.limit - self.used
    }

    pub fn used(&self) -> u64 {
        self.used
    }
}
