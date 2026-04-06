use crate::period::Period;

pub mod amount;
pub mod period;
pub mod tax_brackets;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    gross_income: f64,

    #[clap(short, long, value_enum, default_value_t=Period::Yearly)]
    income_period: Period,

    #[clap(short, long, value_enum, default_value_t=Period::Yearly)]
    output_period: Period,
}

fn main() {
    let args = <Cli as clap::Parser>::parse();

    let federal_tax_brackets: tax_brackets::TaxBrackets = vec![
        (0, 10),
        (11_926, 12),
        (48_476, 22),
        (103_351, 24),
        (197_301, 32),
        (250_526, 35),
        (626_351, 37),
    ]
    .into();
    let actual = federal_tax_brackets.applied_taxes(args.gross_income);
    let amount = amount::Amount::new(actual, Period::Yearly);
    println!("Federal Taxes: {:.2}", amount.to_period(args.output_period))
}
