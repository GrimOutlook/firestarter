use crate::period::Period;

#[derive(Clone, Copy, Debug)]
pub struct Amount {
    value: f64,
    period: Period,
}

impl Amount {
    pub fn new(value: f64, period: Period) -> Amount {
        Amount { value, period }
    }

    pub fn to_period(&self, period: Period) -> f64 {
        self.period.conversion(period) * self.value
    }
}
