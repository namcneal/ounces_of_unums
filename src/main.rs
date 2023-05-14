#![feature(int_roundings)]
#![feature(generic_arg_infer)]
mod u_layer;
mod g_layer;


use std::mem::MaybeUninit;
use gmp_mpfr_sys::mpfr;
fn main() {
    unsafe {
        let mut x = MaybeUninit::uninit();
        mpfr::init2(x.as_mut_ptr(), 200);
        
        let overunderflow = mpfr::set_ui(x.as_mut_ptr(), 5, mpfr::rnd_t::RNDN);
    
        // let old_exponent = mpfr::get_exp(x.as_mut_ptr());
        // let new_exponent : i64 = -2;

        let mantissa_bits = x.assume_init().d;
        let first_limb = mantissa_bits.as_ptr().offset(2).as_ref();


        // let x = rug::Float::from_raw(x.assume_init());
        println!("{:b}", first_limb.unwrap());

    
    }
}





// impl Unum{
//     fn from_f64(val:f64, rounding:RoundingMode){
//         let mut num = MaybeUninit::uninit();
        
//         use RoundingMode::*;
//         let rounding : mpfr::rnd_t = match rounding {
//             Nearest => mpfr::rnd_t::RNDN,
//             Up      => mpfr::rnd_t::RNDU,
//             Down    => mpfr::rnd_t::RNDD,
//             TowardZero => mpfr::rnd_t::RNDZ,
//             AwayFromZero => mpfr::rnd_t::RNDA
//         };

//         unsafe { 
//             let mpfr_rounding_direction = mpfr::set_d(num.as_mut_ptr(), val, rounding); 
        

//             let ubit : bool;
//             if mpfr_rounding_direction == 0{
//                 ubit = false;
//             }

//             Unum{
//                 num : num,
//                 u
//             }

//         }
//     }
// }
