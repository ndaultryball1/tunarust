// Integration test for finite difference pricing against the exact solution

extern crate options_pricing;

use options_pricing::finite_difference::explicit::price;
use options_pricing::assets::{european::*, Vanilla, Asset, Side};

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

    let result_call = price(test_call, &underlying, remaining, spot);
    let result_put = price(test_put, &underlying, remaining, spot);
    let exact_call = test_call.exact_solution(&underlying, spot, remaining);
    let exact_put = test_put.exact_solution(&underlying, spot, remaining);

    statrs::assert_almost_eq!(result_put, exact_put, 1.);
    statrs::assert_almost_eq!(result_call, exact_call, 1.);
}