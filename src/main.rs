#![feature(int_roundings)]
#![feature(generic_arg_infer)]
mod mpfr_glue;
mod u_layer;
mod g_layer;
mod h_layer;

use u_layer::ubounds::*;
use h_layer::from_floats::*;

fn main() {
    unsafe {
        let x : Ubound<u8,u8> = 0.1.into();

        println!("{:?}", &x);
    
    }
}

