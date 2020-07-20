use super::Params;
use crate::assets::{european::European, Asset, Discretisable, Vanilla};
use crate::utils::sqr;

fn lu_find_y(params: &Params) -> Vec<f64> {
    // Find the diagonal elements of the LU decomposition
    let mut ys = vec![0.; params.numx() as usize - 2];
    // Ys are used only for internal grid points.
    let mut prev = 0.; // Keep track of previous calculated element of vector
    for (i, loc) in ys.iter_mut().enumerate() {
        *loc = match i {
            0 => 1. + 2. * params.alpha(),
            _ => 1. + 2. * params.alpha() - sqr(params.alpha()) / prev,
        };
        prev = *loc;
        if prev == 0. {
            panic!("Problem is singular - bad luck!")
        };
    }
    ys
}

fn advance_solution(newu: &mut Vec<f64>, bs: &Vec<f64>, ys: &Vec<f64>, params: &Params) {
    // Take in newu array with spatial bcs set.

    let mut qs = vec![0.; params.numx() as usize - 2];
    // Calculation only for internal points

    let mut prev = 0.; // Want second term to be zero for i = 0
    let mut y_prev: f64;
    for (i, loc) in qs.iter_mut().enumerate() {
        y_prev = match i {
            0 => 1.,
            _ => ys[i - 1],
        }; // For first step want to set qs[0] = bs[0] so ignore ys here.
        prev = bs[i] + params.alpha() * prev / y_prev;
        *loc = prev;
    }
    prev = 0.;
    for i in (1..params.numx() as usize).rev() {
        prev = (qs[i] + prev * params.alpha()) / ys[i];
        newu[i] = prev;
    }
}

fn lu_solve<T: Discretisable + Vanilla>(
    to_price: T,
    underlying: &Asset,
    time_remaining: f64,
    params: &Params,
) -> Vec<f64> {
    let ys = lu_find_y(&params);

    // Set temporal boundary conditions
    let mut u: Vec<f64> = super::get_boundary_t0(&to_price, &underlying, &params);

    let mut bs: Vec<f64>;

    for j in 1..params.numt(to_price.dimless_time(underlying, time_remaining)) {
        let tau = j as f64 * params.dt;
        bs = u.as_slice().to_owned();

        super::set_boundary_spatial(&mut u, &to_price, &underlying, tau, &params);

        bs[1] += params.alpha() * *u.first().unwrap();
        bs[params.numx() as usize - 2] += params.alpha() * *u.last().unwrap();
        advance_solution(&mut u, &bs, &ys, &params);
    }
    u
}

pub fn price(
    to_price: European,
    underlying: &Asset,
    time_remaining: f64,
    spot: f64,
    params: &Params,
) -> f64 {
    //First generate the result array from the explicit fwd difference
    let results = lu_solve(to_price, &underlying, time_remaining, &params);
    let loc = params.spot_to_array_loc(to_price.log_moneyness(spot));
    to_price.u_to_value(&underlying, time_remaining, spot, results[loc])
}
