#[derive(Clone, Copy, Debug, Default, clap::ValueEnum)]
pub enum Period {
    #[default]
    Yearly,
    Monthly,
    Biweekly,
    Weekly,
    Daily,
}

impl Period {
    pub fn conversion(&self, period: Period) -> f64 {
        self.from_yearly_conversion() / period.from_yearly_conversion()
    }

    fn from_yearly_conversion(&self) -> f64 {
        match self {
            Period::Yearly => 1.0,
            Period::Monthly => 12.0,
            Period::Biweekly => 26.0,
            Period::Weekly => 52.0,
            Period::Daily => 365.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    use crate::Period::Biweekly;
    use crate::Period::Daily;
    use crate::Period::Monthly;
    use crate::Period::Weekly;
    use crate::Period::Yearly;
    use crate::period::Period;

    #[test_case(Yearly, Yearly, 1.0; "yearly -> yearly")]
    #[test_case(Yearly, Monthly, 1.0 / 12.0; "yearly -> monthly")]
    #[test_case(Weekly, Daily, 1.0 / 7.0; "weekly -> daily")]
    fn test_conversion(input: Period, output: Period, expected: f64) {
        assert_eq!(expected, input.conversion(output))
    }
}
