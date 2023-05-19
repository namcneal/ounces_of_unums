mod mpfr_glue;
mod u_layer;
mod g_layer;
mod h_layer;

use u_layer::ubounds::*;
use h_layer::from_f64::*;

fn main() {
    unsafe {
        let x : Ubound<u8,u8> = (-10.156).into();

        println!("{}", x);
    
    }
}

