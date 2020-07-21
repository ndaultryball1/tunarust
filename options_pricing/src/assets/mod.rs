pub mod european;

use super::utils;
use statrs::distribution::{Normal, Univariate};
#[derive(Copy, Debug, Clone)]
pub struct Asset {
    // An Asset represents the state of the world at a timestep
    // Hopefully will allow for models of non-constant vol and interest rate
    pub vol: f64,
    pub rate: f64,
}

#[derive(Copy, Debug, Clone)]
pub enum Side {
    Call = 1,
    Put = -1,
}

pub trait Vanilla {
    // implement this to provide a new type of option with a payoff Function
    // TODO: Figure out what this will mean for path dependence, different exercise times etc
    fn new(strike: f64, side: Side) -> Self;
    fn payoff(&self, spot: f64) -> f64; // Payoff in financial variables
    fn dimless_time(&self, underlying: &Asset, time_remaining: f64) -> f64 {
        0.5 * utils::sqr(underlying.vol) * time_remaining
    }
}
pub trait Discretisable {
    // Implement this to provide boundary conditions for a finite difference scheme
    // Ideally this would be linked to the payoff function in some way
    fn boundary_spatial_p(&self, underlying: &Asset, price: f64, time_remaining: f64) -> f64;
    fn boundary_spatial_m(&self, underlying: &Asset, price: f64, time_remaining: f64) -> f64;
    fn boundary_t0(&self, underlying: &Asset, price: f64) -> f64;
    // Boundary condition in the variables
    // relevant to the problem i.e heat equation formulation
    fn u_to_value(&self, underlying: &Asset, u: f64, time_remaining: f64, spot: f64) -> f64;
    // Implement this to convert from the results of the finite-difference scheme
    // back to value of the option.
}
