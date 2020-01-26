use options_pricing::{explicit, assets::{European, Asset, Vanilla}};
use explicit::price;

fn main() {
    let deriv_ex = European::new(60., true);
    let underlying = Asset {vol: 0.2, rate:0.05};
    let spot = 60.;
    let result = price(deriv_ex, &underlying, 0.5, spot);
    println!("{}", result);
}
