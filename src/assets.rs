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
    Put = -1
}

#[derive(Copy, Debug, Clone)]
pub struct European {
    pub strike: f64,
    pub side: Side // This represents call vs put options
}
impl European {

    // Functions to extract the two dimensionless parameters of the pricing problem
    pub fn sign(&self) -> f64 {
        self.side as i32 as f64
    }

    pub fn dimless_k(&self, underlying:&Asset) -> f64 {
        underlying.rate / (0.5 * sqr(underlying.vol))
    }
    pub fn exact_solution(&self, underlying: &Asset, price:f64, time_remaining:f64) -> f64 {
        let dist = Normal::new(0.0, 1.0).unwrap();

        let d1: f64 = (self.log_moneyness(price)  + (underlying.rate + 0.5*sqr(underlying.vol))*time_remaining) / (underlying.vol * time_remaining.sqrt());
        let d2: f64 = d1 - (underlying.vol*time_remaining.sqrt());
        self.sign() * (price*dist.cdf(self.sign() * d1) - self.strike*(-1. * underlying.rate * time_remaining).exp()*dist.cdf(self.sign() * d2))
    }
    pub fn log_moneyness(&self, spot: f64) -> f64 {
        (spot/self.strike).ln()
    }
}

pub trait Vanilla {
    // implement this to provide a new type of option with a payoff Function
    // TODO: Figure out what this will mean for path dependence, different exercise times etc
    fn new(strike: f64, side: Side) -> Self;
    fn payoff(&self, spot: f64) -> f64; // Payoff in financial variables
    fn dimless_time(&self, underlying: &Asset, time_remaining: f64) -> f64 {
        0.5 * sqr(underlying.vol) * time_remaining
    }

}
pub trait Discretisable {
    // Implement this to provide boundary conditions for a finite difference scheme
    // Ideally this would be linked to the payoff function in some way
    fn boundary_spatial_p(&self, underlying: &Asset, price:f64, time_remaining:f64) -> f64;
    fn boundary_spatial_m(&self, underlying: &Asset, price:f64, time_remaining:f64) -> f64;
    fn boundary_t0(&self, underlying: &Asset, price:f64) -> f64;
    // Boundary condition in the variables
    //relevant to the problem i.e heat equation formulation
    fn u_to_value(&self, underlying: &Asset, u:f64, time_remaining:f64, spot:f64) -> f64;
        // Implement this to convert from the results of the finite-difference scheme
        // back to value of the option.
}
impl Vanilla for European {
    fn payoff(&self, spot: f64) -> f64 {
        (self.sign() * (spot - self.strike)).max(0.)
    }
    fn new(strike: f64, side: Side) -> European {
        // Gets around non-obvious code of sign/type
        European {
            strike,
            side
        }
    }
}
impl Discretisable for European {
    fn boundary_t0(&self, underlying: &Asset, x: f64) -> f64 {
        (self.sign() * (
            (0.5 * (self.dimless_k(&underlying) +1.)*x).exp()
            - (0.5 * (self.dimless_k(&underlying) - 1.)*x).exp()
        ))
                .max(0.)
        }

    fn boundary_spatial_p(&self, underlying:&Asset,x: f64, tau: f64) -> f64 {
        match self.side {
            Side::Call => (0.5 * (self.dimless_k(&underlying) + 1.) * x
                + 0.25 * sqr(self.dimless_k(&underlying) + 1.)  * tau).exp(),
            Side::Put => 0.,
            }
        }
    fn boundary_spatial_m(&self, underlying: &Asset, x:f64, tau:f64) -> f64 {
        // Boundary condition at log_moneyness of -inf, or price of 0.
        match self.side {
            Side::Call => 0.,
            Side::Put => ((0.25 * sqr(self.dimless_k(&underlying) + 1.) + self.dimless_k(&underlying) )* tau + 0.5 * (self.dimless_k(&underlying) -1.) * x).exp(),
        }
    }


    fn u_to_value(&self, underlying:&Asset, time_remaining:f64, spot:f64, u:f64) -> f64 {
        // Given a u-value read from the result array of finite difference, returns the corresponding option value
        // Requires that your difference scheme converts spot price into an array location,
        // given its discretisation parameters
        let exponent = -0.5 * (self.dimless_k(&underlying) - 1.)*self.log_moneyness(spot) - 0.25 * sqr(self.dimless_k(&underlying) + 1.) * self.dimless_time(&underlying, time_remaining);
        self.strike * exponent.exp() * u
    }
}
fn sqr(x:f64) -> f64 {
    // maybe refactor to a utils file
    x * x
}
