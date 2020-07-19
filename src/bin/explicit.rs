use options_pricing::{
    assets::{european::European, Asset, Side, Vanilla},
    finite_difference::*,
};

fn main() {
    let deriv_ex = European::new(50., Side::Call);
    let underlying = Asset {
        vol: 0.2,
        rate: 0.05,
    };
    let spot = 60.;
    let params = Params::reasonable_defaults();

    let result = explicit::price(deriv_ex, &underlying, 0.5, spot, &params);
    println!("{}", result);

    let test_call = European::new(50., Side::Call);
    let _test_put = European::new(50., Side::Put);
    let underlying = Asset {
        vol: 0.2,
        rate: 0.05,
    };
    let remaining = 0.0005;
    let spot = 80.;

    let result_call = explicit::price(test_call, &underlying, remaining, spot, &params);
    println!("{}", result_call);
}
