use options_pricing::{explicit, assets::{European, Asset, Vanilla}};


fn main() {
    let deriv_ex = European::new(60., true);
    let underlying = Asset {vol: 0.2, rate:0.05};
    let values = explicit::explicit_fwd(deriv_ex, underlying, 0.5);
    println!("{}", values[5]);
}
