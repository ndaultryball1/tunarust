

pub enum Option {
    EuropeanCall(Derivative),
    EuropeanPut(Derivative),
}
#[derive(Copy, Debug, Clone)]
pub struct Asset {
    pub vol: f64,
    pub rate: f64,
}
#[derive(Copy, Debug, Clone)]
pub struct Derivative {
    pub strike: f64,
    pub time_remaining: f64,
    pub underlying: Asset,
}
impl Derivative {
    pub fn from_data(strike: f64, time_remaining: f64, vol: f64, rate: f64) -> Derivative {
        // Generic creator for one off option without asset.
        Derivative {
            strike: strike,
            time_remaining: time_remaining,
            underlying: Asset {
                vol: vol,
                rate: rate,
            },
        }
    }
    // Functions to extract the two dimensionless parameters of the pricing problem
    pub fn dimless_time(&self) -> f64 {
        0.5 * self.underlying.vol * self.underlying.vol * self.time_remaining
    }
    pub fn dimless_k(&self) -> f64 {
        self.underlying.rate / (0.5 * self.underlying.vol * self.underlying.vol)
    }
}
impl Option {
    pub fn unwrap(&self) -> &Derivative {
        match self {
            Option::EuropeanCall(deriv) => deriv,
            Option::EuropeanPut(deriv) => deriv,
        }
    }
    pub fn payoff(&self, spot: f64) -> f64 {
        match self {
            Option::EuropeanCall(derivative) => (spot - derivative.strike).max(0.),
            Option::EuropeanPut(derivative) => (derivative.strike - spot).max(0.),
        }
    }
}
