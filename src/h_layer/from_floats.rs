use crate::u_layer::backend_reprs::MantissaBackend;
use crate::u_layer::unums::*;
use crate::u_layer::ubounds::*;
use crate::g_layer::gbounds::*;
use gmp_mpfr_sys::mpfr;
use num_traits::Signed;
use std::mem::MaybeUninit;


#[duplicate::duplicate_item(
    int_left    int_right ;
     [ u8  ]   [ u8 ];   
     [ u16 ]   [ u8 ];
     [ u32 ]   [ u8 ];
     [ u64 ]   [ u8 ];
 
     [ u8  ]   [ u16 ];   
     [ u16 ]   [ u16 ];
     [ u32 ]   [ u16 ];
     [ u64 ]   [ u16 ];
 
     [ u8  ]   [ u32 ];   
     [ u16 ]   [ u32 ];
     [ u32 ]   [ u32 ];
     [ u64 ]   [ u32 ];
 
     [ u8  ]   [ u64 ];   
     [ u16 ]   [ u64 ];
     [ u32 ]   [ u64 ];
     [ u64 ]   [ u64 ];
 )]
impl From<f64> for Ubound<int_left, int_right>{
    fn from(value: f64) -> Self {
        // Handle the NaN cases
        if value.is_nan(){
            return Ubound{
                left  : Unum::<int_left>::nan(),
                right : Unum::<int_right>::nan()
            }
        } else if value.is_infinite(){
            if value > 0.0{
                return Ubound{
                    left  : Unum::<int_left>::posinf(),
                    right : Unum::<int_right>::posinf()
                }
            } else{
                return Ubound{
                    left  : Unum::<int_left>::neginf(),
                    right : Unum::<int_right>::neginf()
                }
            }
        } else{
            unsafe{
                let mut left_mpfr = MaybeUninit::uninit();
                mpfr::init2(left_mpfr.as_mut_ptr(), <int_left as MantissaBackend>::precision() as i64);
                let left_rounded_down = mpfr::set_d(left_mpfr.as_mut_ptr(), value, mpfr::rnd_t::RNDD);

                let mut right_mpfr = MaybeUninit::uninit();
                mpfr::init2(right_mpfr.as_mut_ptr(), <int_right as MantissaBackend>::precision() as i64);
                let right_rounded_up = mpfr::set_d(right_mpfr.as_mut_ptr(), value, mpfr::rnd_t::RNDU);

                let mut left_right_open : u8 = 0;
                if left_rounded_down != 0{
                    left_right_open |= LEFT_OPEN_MASK;
                }
                if right_rounded_up != 0{
                    left_right_open |= RIGHT_OPEN_MASK;
                }

                let gbound = Gbound{
                    left  : left_mpfr,
                    right : right_mpfr,
                    open  : IntervalOpenness(left_right_open)
                };

                let ubound : Ubound<int_left, int_right> = (&gbound).into();
                // drop(gbound);

                return ubound

            }

        }
    }
}


