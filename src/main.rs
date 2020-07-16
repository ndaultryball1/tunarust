use explicit::price;
use options_pricing::{
    assets::{Asset, European, Side, Vanilla},
    explicit,
};

fn main() {
    let deriv_ex = European::new(50., Side::Call);
    let underlying = Asset {
        vol: 0.2,
        rate: 0.05,
    };
    let spot = 60.;
    let result = price(deriv_ex, &underlying, 0.5, spot);
    println!("{}", result);
}
