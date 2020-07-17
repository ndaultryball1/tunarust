use super::Params;
use crate::utils::sqr;


fn lu_find_y(params: &Params) -> Vec<f64> {
    // Find the diagonal elements of the LU decomposition
    let mut ys = vec![0.; params.numx() as usize];
    let mut prev = 0.; // Keep track of previous calculated element of vector
    for (i, loc) in ys.iter_mut().enumerate() {
        *loc = match i {
            0 => 1. + 2. * params.alpha(),
            _ => 1. + 2. * params.alpha() - sqr(params.alpha()) / prev
        };
        prev = *loc;
        if prev == 0. { panic!("Problem is singular - bad luck!") };
    }
    ys
}
