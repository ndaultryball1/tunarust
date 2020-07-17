pub mod explicit;
pub mod implicit;

use crate::utils::sqr;

pub struct Params {
    // Unifies parameters for a discretisation, and provides
    // common helper functions to ease writing routines
    pub dx: f64, // Size of spatial increment
    pub dt: f64, // Size  of temporal increment
    pub minus: i32, // Lower bound for x (log moneynness)
    pub plus: i32
    
}

impl Params {
    pub fn numx(&self) -> i32 {
        self.plus - self.minus + 1
    }

    pub fn numt(&self, dimless_time: f64) -> i32 {
        (dimless_time / self.dt).round() as i32
    }
    
    pub fn alpha(&self) -> f64 {
        self.dt / sqr(self.dx)
    }

    pub fn spot_to_array_loc(&self, log_moneyness: f64) -> usize {
        // Function to convert a log_moneyness at which the option price is desired, to an array location
        ((log_moneyness - self.minus as f64* self.dx) / self.dx) as usize 
    }

    pub fn reasonable_defaults() -> Params {
        // Sensible defaults
        Params { dx: 0.01,
                 dt: 0.00003,
                 minus: -1000,
                 plus: 1000}
    }

}



#[cfg(test)]
mod tests {
    use super::Params;

    #[test]
    fn test_spot_to_array_loc() {
        assert_eq!(Params::reasonable_defaults().spot_to_array_loc(0.), 1000)
    }
    #[test]
    fn test_reasonable_alpha() {
        println!("{}", Params::reasonable_defaults().alpha() );
        assert!(Params::reasonable_defaults().alpha() < 0.5)
    }
}