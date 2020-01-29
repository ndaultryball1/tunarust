use crate::assets::{European, Asset, Vanilla, Discretisable};
use statrs;
use crate::explicit::price;

#[test]
fn exact_put_call_parity(){
    // Function testing implementation of exact solution obeys put-call parity

    let spot: f64 = 60.;
    let remaining: f64 = 0.5;
    let underlying = Asset {vol: 0.2, rate:0.05};
    let strike = 50.;
    let test_call = European::new(strike, true);
    let test_put = European::new(strike, false);

    let call_price = test_call.exact_solution(&underlying, spot, remaining);
    let put_price = test_put.exact_solution(&underlying, spot, remaining);
    let discounted_strike = (-1. * remaining * underlying.rate).exp() * strike;

    assert_eq!(put_price + spot - call_price, discounted_strike);
}

#[test]
fn explicit_fwd_value(){
    // Test of the result of the explicit fwd difference scheme
    let test_call = European::new(50., true);
    let underlying = Asset {vol: 0.2, rate:0.05};
    let remaining = 0.5;
    let spot = 60.;
    let result = price(test_call, &underlying, remaining, spot);
    let exact = test_call.exact_solution(&underlying, spot, remaining);

    statrs::assert_almost_eq!(result, exact, 5.);
}

#[test]
fn boundaries_t0(){
    // Test the spatial bcs agree with time bc at tau = 0
    let test_call = European::new(50., true);
    let underlying = Asset {vol: 0.2, rate:0.05};
    let remaining = 0.5;
    let bm_min = test_call.boundary_spatial(&underlying, -10., 0.);
    let bm_plus = test_call.boundary_spatial(&underlying, 10., 0.);
    let t0_min = test_call.boundary_t0(&underlying, -10.);
    let t0_plus = test_call.boundary_t0(&underlying, 10.);

    statrs::assert_almost_eq!(bm_min, t0_min, 1.);
    statrs::assert_almost_eq!(bm_plus, t0_plus, 1.);
}
