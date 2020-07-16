// This module implements explicit finite difference methods
// Plan - generalise to non-european options.
use crate::assets::{Discretisable,Asset, Vanilla, European};

const DX: f64 = 0.01;  // Size of spatial increment
const M: i32 = 300;  // Number of timesteps to divide the interval
const MINUS: i32 = 1000;  // Lower bound for x (log moneynness)
const PLUS: i32 = 1000;
const NUMX: usize = (MINUS + PLUS) as usize + 1;

fn explicit_fwd<T: Vanilla + Discretisable>(to_price: T, underlying: &Asset, time_remaining: f64) -> Vec<f64> {
    // Implements explicit forward difference scheme
    // to solve transformed BS equation.
    let dt =
        to_price.dimless_time(&underlying, time_remaining) / M as f64;
    let alpha = dt / (DX * DX);
    println!("Alpha for the problem is {}", alpha);
    if alpha>0.5 {
        println!("Solver is unstable for alpha > 0.5. Please increase time precision.");
    }

    let mut oldu = vec![0.; NUMX];
    let mut newu = vec![0.; NUMX];

    for (i, loc) in oldu.iter_mut().enumerate() {
        *loc = to_price.boundary_t0(&underlying, (i as i32 - MINUS) as f64 * DX);
    }

    for j in 1..M {
        let tau = j as f64 * dt;

        *newu.first_mut().unwrap() = to_price.boundary_spatial_m(&underlying, MINUS as f64 * DX * -1., tau);
        *newu.last_mut().unwrap() = to_price.boundary_spatial_p(&underlying, PLUS as f64 * DX, tau);

        for (n, loc) in newu.iter_mut().enumerate() {
            *loc = oldu[n] + alpha * (oldu[n - 1] - 2.0 * oldu[n] + oldu[n + 1]);
        }
        oldu = newu.as_mut_slice().to_owned();
    }
    oldu
}
fn spot_to_array_loc(log_moneyness:f64) -> usize {
    // Function to convert a log_moneyness at which the option price is desired, to an array location
    (log_moneyness/DX).round() as usize
}

pub fn price(to_price:European, underlying: &Asset, time_remaining: f64, spot:f64) -> f64 {
    //First generate the result array from the explicit fwd difference
    let results = explicit_fwd(to_price, &underlying, time_remaining);
    let loc = spot_to_array_loc(to_price.log_moneyness(spot));
    to_price.u_to_value(&underlying, time_remaining, spot, results[loc])
}
