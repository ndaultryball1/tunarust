// Integration test for finite difference pricing against the exact solution

extern crate options_pricing;

use options_pricing::assets::{european::*, Asset, Side, Vanilla};
use options_pricing::finite_difference::*;

#[test]
fn explicit_fwd_value() {
    // Test of the result of the explicit fwd difference scheme
    let test_call = European::new(50., Side::Call);
    let test_put = European::new(50., Side::Put);
    let underlying = Asset {
        vol: 0.2,
        rate: 0.05,
    };
    let remaining = 0.5;
    let spot = 70.;

    let params = Params::reasonable_defaults();

    let result_call = explicit::price(test_call, &underlying, remaining, spot, &params);
    let result_put = explicit::price(test_put, &underlying, remaining, spot, &params);
    let exact_call = test_call.exact_solution(&underlying, spot, remaining);
    let exact_put = test_put.exact_solution(&underlying, spot, remaining);

    statrs::assert_almost_eq!(result_put, exact_put, 1.);
    statrs::assert_almost_eq!(result_call, exact_call, 1.);
}

#[test]
fn implicit_fwd_value() {
    // Test of the result of the implicit fwd difference scheme
    let test_call = European::new(50., Side::Call);
    let test_put = European::new(50., Side::Put);
    let underlying = Asset {
        vol: 0.2,
        rate: 0.05,
    };
    let remaining = 0.5;
    let spot = 70.;

    let params = Params::reasonable_defaults();

    let result_call = implicit::price(test_call, &underlying, remaining, spot, &params);
    let result_put = implicit::price(test_put, &underlying, remaining, spot, &params);
    let exact_call = test_call.exact_solution(&underlying, spot, remaining);
    let exact_put = test_put.exact_solution(&underlying, spot, remaining);

    statrs::assert_almost_eq!(result_put, exact_put, 1.);
    statrs::assert_almost_eq!(result_call, exact_call, 1.);
}
