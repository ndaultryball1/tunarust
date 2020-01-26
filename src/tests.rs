use crate::assets::{European, Asset, Vanilla};
use statrs;

#[test]
fn put_call_parity(){
    // Function testing implementation of exact solution obeys put-call parity

    let spot: f64 = 60.;
    let remaining: f64 = 0.5;
    let strike:f64 = 50.;
    let underlying = Asset {vol: 0.2, rate:0.05};
    let test_call = European::new(50., true);
    let test_put = European::new(50., false);

    let call_price = test_call.exact_solution(&underlying, spot, remaining);
    let put_price = test_put.exact_solution(&underlying, spot, remaining);
    let discounted_strike = (-1. * remaining * underlying.rate).exp() * strike;

    statrs::assert_almost_eq!(put_price + spot - call_price, discounted_strike, 1.);
}
