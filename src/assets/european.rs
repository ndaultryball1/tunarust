use super::*;
use super::utils::sqr;

#[derive(Copy, Debug, Clone)]
pub struct European {
    pub strike: f64,
    pub side: Side, // This represents call vs put options
}
impl European {
    // Functions to extract the two dimensionless parameters of the pricing problem
    pub fn sign(&self) -> f64 {
        self.side as i32 as f64
    }

    pub fn dimless_k(&self, underlying: &Asset) -> f64 {
        underlying.rate / (0.5 * sqr(underlying.vol))
    }
    pub fn exact_solution(&self, underlying: &Asset, price: f64, time_remaining: f64) -> f64 {
        // Black-Scholes formula for European options
        let dist = Normal::new(0.0, 1.0).unwrap();

        let d1: f64 = (self.log_moneyness(price)
            + (underlying.rate + 0.5 * sqr(underlying.vol)) * time_remaining)
            / (underlying.vol * time_remaining.sqrt());
        let d2: f64 = d1 - (underlying.vol * time_remaining.sqrt());
        self.sign()
            * (price * dist.cdf(self.sign() * d1)
                - self.strike
                    * (-1. * underlying.rate * time_remaining).exp()
                    * dist.cdf(self.sign() * d2))
    }
    pub fn log_moneyness(&self, spot: f64) -> f64 {
        (spot / self.strike).ln()
    }
}
impl Vanilla for European {
    fn payoff(&self, spot: f64) -> f64 {
        (self.sign() * (spot - self.strike)).max(0.)
    }
    fn new(strike: f64, side: Side) -> European {
        // Gets around non-obvious code of sign/type
        European { strike, side }
    }
}
impl Discretisable for European {
    fn boundary_t0(&self, underlying: &Asset, x: f64) -> f64 {
        (self.sign()
            * ((0.5 * (self.dimless_k(&underlying) + 1.) * x).exp()
                - (0.5 * (self.dimless_k(&underlying) - 1.) * x).exp()))
        .max(0.)
    }

    fn boundary_spatial_p(&self, underlying: &Asset, x: f64, tau: f64) -> f64 {
        match self.side {
            Side::Call => (0.5 * (self.dimless_k(&underlying) + 1.) * x
                + 0.25 * sqr(self.dimless_k(&underlying) + 1.) * tau)
                .exp(),
            Side::Put => 0.,
        }
    }
    fn boundary_spatial_m(&self, underlying: &Asset, x: f64, tau: f64) -> f64 {
        // Boundary condition at log_moneyness of -inf, or price of 0.
        match self.side {
            Side::Call => 0.,
            Side::Put => (0.5 * (self.dimless_k(&underlying) + 1.) * x
            + 0.25 * sqr(self.dimless_k(&underlying) + 1.) * tau)
            .exp(),
        }
    }

    fn u_to_value(&self, underlying: &Asset, time_remaining: f64, spot: f64, u: f64) -> f64 {
        // Given a u-value read from the result array of finite difference, returns the corresponding option value
        // Requires that your difference scheme converts spot price into an array location,
        // given its discretisation parameters
        let exponent = -0.5 * (self.dimless_k(&underlying) - 1.) * self.log_moneyness(spot)
            - 0.25
                * sqr(self.dimless_k(&underlying) + 1.)
                * self.dimless_time(&underlying, time_remaining);
        self.strike * exponent.exp() * u
    }
}

mod tests {
    use super::{Asset, European, Side::*, Vanilla, Discretisable};

    #[test]
    fn exact_put_call_parity() {
        // Function testing implementation of exact solution obeys put-call parity

        let spot: f64 = 60.;
        let remaining: f64 = 0.5;
        let underlying = Asset {
            vol: 0.2,
            rate: 0.05,
        };
        let strike = 50.;
        let test_call = European::new(strike, Call);
        let test_put = European::new(strike, Put);

        let call_price = test_call.exact_solution(&underlying, spot, remaining);
        let put_price = test_put.exact_solution(&underlying, spot, remaining);
        let discounted_strike = (-1. * remaining * underlying.rate).exp() * strike;

        assert_eq!(put_price + spot - call_price, discounted_strike);
    }

    #[test]
    fn boundaries_t0() {
        // Test the spatial bcs agree with time bc at tau = 0
        let test_call = European::new(50., Call);
        let underlying = Asset {
            vol: 0.2,
            rate: 0.05,
        };
        let _remaining = 0.5;

        let bm_min = test_call.boundary_spatial_m(&underlying, -10., 0.);
        let bm_plus = test_call.boundary_spatial_p(&underlying, 10., 0.);
        let t0_min = test_call.boundary_t0(&underlying, -10.);
        let t0_plus = test_call.boundary_t0(&underlying, 10.);

        assert_eq!(bm_min, 0.);
        assert_eq!(t0_min, 0.);
        statrs::assert_almost_eq!(bm_plus, t0_plus, 0.001 * bm_plus);
    }

}