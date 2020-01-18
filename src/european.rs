
use crate::assets::Option;
use statrs::distribution::{Normal, Univariate};

const DX: f64 = 0.01;
const M: i32 = 100;
const MINUS: i32 = -500;
const PLUS: i32 = 500;
const NUMX: usize = (-MINUS + PLUS) as usize + 1;
fn boundary_t0(to_price: &Option, x: f64) -> f64 {
    match to_price {
        Option::EuropeanCall(deriv) => ((0.5 * (deriv.dimless_k() + 1.) * x).exp()
            - (0.5 * (deriv.dimless_k() - 1.) * x).exp())
        .max(0.),
        Option::EuropeanPut(deriv) => ((-0.5 * (deriv.dimless_k() + 1.) * x).exp()
            + (0.5 * (deriv.dimless_k() - 1.) * x).exp())
        .max(0.),
    }
}
fn boundary_spatial(to_price: &Option, x: f64, tau: f64) -> f64 {
    let deriv = to_price.unwrap();
    deriv.strike
        * (0.5 * (deriv.dimless_k() + 1.) * x
            + 0.25 * (deriv.dimless_k() + 1.) * (deriv.dimless_k() + 1.) * tau)
}
pub fn to_financial(option: &Option, x: f64) -> f64 {
    option.unwrap().strike
}
pub fn explicit_fwd(to_price: Option) -> [f64; NUMX] {
    // Implements explicit forward difference scheme
    // to solve transformed BS equation.

    let dt =
        0.5 * to_price.unwrap().underlying.vol * to_price.unwrap().time_remaining / M as f64;
    let alpha = dt / (DX * DX);

    let mut oldu = [0.; NUMX];
    let mut newu = [0.; NUMX];

    for i in 0..NUMX {
        oldu[i] = boundary_t0(&to_price, i as f64 * DX);
    }

    for j in 1..M {
        let tau = j as f64 * dt;

        newu[0] = boundary_spatial(&to_price, MINUS as f64 * DX, tau);
        newu[NUMX - 1] = boundary_spatial(&to_price, PLUS as f64 * DX, tau);

        for n in 1..NUMX - 1 {
            newu[n] = oldu[n] + alpha * (oldu[n - 1] - 2.0 * oldu[n] + oldu[n + 1]);
        } // This is the explicit increment
        for n in 0..NUMX {
            oldu[n] = newu[n];
        }
    }
    oldu
}
fn sqr(x:f64) -> f64 {
    x * x
}

pub fn exact_solution(to_price: &Option, price:f64) -> f64 {
    let dist = Normal::new(0.0, 1.0).unwrap();
    let deriv = to_price.unwrap();
    let sign = match to_price {
        Option::EuropeanCall(_) => 1.,
        Option::EuropeanPut(_) => -1.,
    };
    let d1: f64 = sign *((price / deriv.strike).ln()  + (deriv.underlying.rate + sqr(deriv.underlying.vol))*deriv.time_remaining) / (deriv.underlying.vol * deriv.time_remaining);
    let d2: f64 = sign * (d1 - deriv.underlying.vol*deriv.time_remaining);
    sign * (price*dist.cdf(d1) - deriv.strike*(- deriv.underlying.rate * deriv.time_remaining).exp()*dist.cdf(d2))
}
