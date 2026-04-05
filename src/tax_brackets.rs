// US Tax brackets always use round numbers so the underlying datatype in is
// an unsigned integer, rather than using a floating point integer.
pub struct TaxBrackets {
    brackets: Vec<TaxBracket>,
}

impl TaxBrackets {
    pub fn applied_taxes(&self, gross_value: f64) -> f64 {
        self.brackets
            .iter()
            .filter(|bracket| bracket.start as f64 <= gross_value)
            .fold(0.00, |acc, bracket| acc + bracket.applied_taxes(gross_value))
    }
}

impl From<Vec<(usize, usize)>> for TaxBrackets {
    fn from(value: Vec<(usize, usize)>) -> Self {
        let mut brackets = Vec::<TaxBracket>::new();
        for (idx, (start, percentage)) in value.into_iter().enumerate() {
            if idx > 0 {
                brackets.get_mut(idx - 1).unwrap().end = Some(start - 1);
            }
            brackets.push(TaxBracket { start, percentage, end: None });
        }
        TaxBrackets { brackets }
    }
}

pub struct TaxBracket {
    start: usize,
    end: Option<usize>,
    percentage: usize,
}

impl TaxBracket {
    pub fn applied_taxes(&self, gross_value: f64) -> f64 {
        if gross_value < self.start as f64 {
            return 0.00;
        }

        let applied_to_bracket = if let Some(end) = self.end {
            f64::min(gross_value, end as f64) - self.start as f64
        } else {
            gross_value - self.start as f64
        };

        applied_to_bracket * (self.percentage as f64 / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    use super::*;

    #[test_case(0.00, 0.00; "$0 gross")]
    #[test_case(10000.00, 1000.00; "$10,000 gross")]
    #[test_case(100000.00, 16913.66; "$100,000 gross")]
    #[test_case(1000000.00, 327018.63; "$1,000,000 gross")]
    fn test_applied(gross_income: f64, expected: f64) {
        let federal_tax_brackets: TaxBrackets = vec![
            (0, 10),
            (11_926, 12),
            (48_476, 22),
            (103_351, 24),
            (197_301, 32),
            (250_526, 35),
            (626_351, 37),
        ]
        .into();
        let actual = federal_tax_brackets.applied_taxes(gross_income);
        assert_eq!(expected, actual);
    }
}
