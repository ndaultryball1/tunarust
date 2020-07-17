use options_pricing::{
    assets::{Asset, european::European, Side, Vanilla},
    finite_difference::explicit::price,
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

    let test_call = European::new(50., Side::Call);
    let test_put = European::new(50., Side::Put);
    let underlying = Asset {
        vol: 0.2,
        rate: 0.05,
    };
    let remaining = 0.0005;
    let spot = 80.;

    let result_call = price(test_call, &underlying, remaining, spot);
    println!("{}", result_call);
}
