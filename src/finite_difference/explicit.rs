// This module implements explicit finite difference methods
// Plan - generalise to non-european options.
use crate::assets::{Asset, Discretisable, european::European, Vanilla};
use super::Params;



fn explicit_fwd<T: Vanilla + Discretisable>(
    to_price: T,
    underlying: &Asset,
    time_remaining: f64,
    params: &Params,
) -> Vec<f64> {
    // Implements explicit forward difference scheme
    // to solve transformed BS equation.
    
    
    println!("Alpha for the problem is {}", params.alpha());
    if params.alpha() > 0.5 {
        println!("Solver is unstable for alpha > 0.5. Please increase time precision.");
    }

    // Collect a range as a vector 
    let mut newu: Vec<f64> = (0..params.numx()).map(|x| x as f64).collect();

    // Set temporal boundary conditions
    let mut oldu = (0..params.numx())
        .map(|x| {
            let price = (x + params.minus) as f64 * params.dx;
            to_price.boundary_t0(underlying, price)
        }).collect::<Vec<f64>>();
    
    
    for j in 1..params.numt(to_price.dimless_time(underlying, time_remaining)) {
        let tau = j as f64 * params.dt;

        // Set spatial boundary conditions at each timestep
        *newu.first_mut().unwrap() =
            to_price.boundary_spatial_m(&underlying, params.minus as f64 * params.dx, tau);
        *newu.last_mut().unwrap() = to_price.boundary_spatial_p(&underlying, params.plus as f64 * params.dx, tau);

        // Populate new vector depending on last timestep
        for n in 1..newu.len() - 1 {
            newu[n] = oldu[n] + params.alpha() * (oldu[n - 1] - 2.0 * oldu[n] + oldu[n + 1]);
        }
        oldu = newu.as_mut_slice().to_owned();
    }
    oldu
}


pub fn price(to_price: European, underlying: &Asset, time_remaining: f64, spot: f64, params: &Params) -> f64 {
    //First generate the result array from the explicit fwd difference
    let results = explicit_fwd(to_price, &underlying, time_remaining, &params);
    let loc = params.spot_to_array_loc(to_price.log_moneyness(spot));
    to_price.u_to_value(&underlying, time_remaining, spot, results[loc])
}

