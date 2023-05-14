#![feature(int_roundings)]
#![feature(generic_arg_infer)]
mod u_layer;
mod g_layer;

use crate::u_layer::unums::*;

use std::mem::MaybeUninit;
use gmp_mpfr_sys::mpfr;
fn main() {
    unsafe {
        let mut x = MaybeUninit::uninit();
        mpfr::init2(x.as_mut_ptr(), 64);
        
        let overunderflow = mpfr::set_ui(x.as_mut_ptr(), 5, mpfr::rnd_t::RNDN);


        let unum : Quart = x.into();
        println!("{:?}", unum);
    
        // // let old_exponent = mpfr::get_exp(x.as_mut_ptr());
        // // let new_exponent : i64 = -2;

        // let mantissa_bits = x.assume_init().d;
        // let first_limb = mantissa_bits.as_ptr().offset(0).as_ref();
        // println!("{:b}", first_limb.unwrap());


        // let x = rug::Float::from_raw(x.assume_init());

    
    }
}

