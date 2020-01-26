
use crate::assets::{Discretisable,Asset};

const DX: f64 = 0.01;
const M: i32 = 100;
const MINUS: i32 = -500;
const PLUS: i32 = 500;
const NUMX: usize = (-MINUS + PLUS) as usize + 1;

pub fn explicit_fwd<T: Discretisable>(to_price: T, underlying: Asset, time_remaining: f64) -> [f64; NUMX] {
    // Implements explicit forward difference scheme
    // to solve transformed BS equation.

    let dt =
        0.5 * underlying.vol * time_remaining / M as f64;
    let alpha = dt / (DX * DX);

    let mut oldu = [0.; NUMX];
    let mut newu = [0.; NUMX];

    for i in 0..NUMX {
        oldu[i] = to_price.boundary_t0(&underlying, i as f64 * DX);
    }

    for j in 1..M {
        let tau = j as f64 * dt;

        newu[0] = to_price.boundary_spatial( &underlying, MINUS as f64 * DX, tau);
        newu[NUMX - 1] = to_price.boundary_spatial(&underlying, PLUS as f64 * DX, tau);

        for n in 1..NUMX - 1 {
            newu[n] = oldu[n] + alpha * (oldu[n - 1] - 2.0 * oldu[n] + oldu[n + 1]);
        } // This is the explicit increment
        for n in 0..NUMX {
            oldu[n] = newu[n];
        }
    }
    oldu
}
