use options_pricing::{european, assets::{Option, Derivative}};


fn main() {
    let deriv_ex = Derivative::from_data(50.,0.5, 0.1, 0.05);
    let example = Option::EuropeanCall(deriv_ex);
    let values = european::explicit_fwd(example);
    println!("{}", values[5]);
}
