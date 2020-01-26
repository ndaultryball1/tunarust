use statrs::distribution::{Normal, Univariate};
#[derive(Copy, Debug, Clone)]
pub struct Asset {
    // An Asset represents the state of the world at a timestep
    // Hopefully will allow for models of non-constant vol and interest rate
    pub vol: f64,
    pub rate: f64,
}

#[derive(Copy, Debug, Clone)]
pub struct European {
    pub strike: f64,
    pub sign: f64, // This represents call vs put options
}
impl European {

    // Functions to extract the two dimensionless parameters of the pricing problem
    pub fn dimless_time(&self, underlying: &Asset, time_remaining: f64) -> f64 {
        0.5 * underlying.vol * underlying.vol * time_remaining
    }
    pub fn dimless_k(&self, underlying:&Asset) -> f64 {
        underlying.rate / (0.5 * underlying.vol * underlying.vol)
    }
    pub fn exact_solution(&self, underlying: &Asset, price:f64, time_remaining:f64) -> f64 {
        let dist = Normal::new(0.0, 1.0).unwrap();

        let d1: f64 = ((price / self.strike).ln()  + (underlying.rate + 0.5*sqr(underlying.vol))*time_remaining) / (underlying.vol * time_remaining.sqrt());
        let d2: f64 = d1 - (underlying.vol*time_remaining.sqrt());
        self.sign * (price*dist.cdf(self.sign * d1) - self.strike*(-1. * underlying.rate * time_remaining).exp()*dist.cdf(self.sign * d2))
    }
}

pub trait Vanilla {
    // implement this to provide a new type of option with a payoff Function
    // TODO: Figure out what this will mean for path dependence, different exercise times etc
    fn new(strike: f64, call: bool) -> Self;
    fn payoff(&self, spot: f64) -> f64; // Payoff in financial variables

}
pub trait Discretisable {
    // Implement this to provide boundary conditions for a finite difference scheme
    // Ideally this would be linked to the payoff function in some way
    fn boundary_spatial(&self, underlying: &Asset, price:f64, time_remaining:f64) -> f64;
    fn boundary_t0(&self, underlying: &Asset, price:f64) -> f64;
    // Boundary condition in the variables
    //relevant to the problem i.e heat equation formulation
}
impl Vanilla for European {
    fn payoff(&self, spot: f64) -> f64 {
        (self.sign * (spot - self.strike)).max(0.)
    }
    fn new(strike: f64, call: bool) -> European {
        // Generic creator for one off option without asset.
        // Also gets around non-obvious code of sign/type
        European {
            strike: strike,
            sign: if call {1.} else {-1.},
        }
    }
}
impl Discretisable for European {
    fn boundary_t0(&self, underlying: &Asset, x: f64) -> f64 {
        ((self.sign * 0.5 * (self.dimless_k(&underlying) + 1.) * x).exp()
            - self.sign * (0.5 * (self.dimless_k(&underlying) - 1.) * x).exp())
        .max(0.)
        }

    fn boundary_spatial(&self, underlying:&Asset,x: f64, tau: f64) -> f64 {
        self.strike
            * (0.5 * (self.dimless_k(&underlying) + 1.) * x
                + 0.25 * (self.dimless_k(&underlying) + 1.) * (self.dimless_k(&underlying) + 1.) * tau)
    }
}
fn sqr(x:f64) -> f64 {
    x * x
}
