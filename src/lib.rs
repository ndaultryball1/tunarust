pub mod assets;
pub mod european;

#[cfg(test)]
mod tests {
    use european::{exact_solution};
    use assets::{Derivative, Option};
    use statrs;
    #[test]
    fn put_call_parity(){
        // Function testing implementation of exact solution obeys put-call parity
        let price: f64 = 60.;
        let test_option = Derivative::from_data(50.,0.5,0.2,0.05);
        let call_price = exact_solution(&Option::EuropeanCall(test_option), price);
        let put_price = exact_solution(&Option::EuropeanPut(test_option), price);
        let discounted_strike = (-1. * test_option.time_remaining * test_option.underlying.rate).exp() * test_option.strike;

        statrs::assert_almost_eq!(call_price - put_price - price + discounted_strike, 0., 1.);
    }
}
