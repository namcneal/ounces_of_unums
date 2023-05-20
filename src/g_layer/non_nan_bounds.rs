use std::mem::MaybeUninit;

use crate::mpfr_glue::prelude::*;
use crate::u_layer::ubounds::*;
use crate::u_layer::backend_reprs::*;
use gmp_mpfr_sys::mpfr;

pub (super) struct NonNaNBound{
    pub (super) endpoint : MPFRFloatPtr, // Assume that this is NOT NaN
    pub (super) open     : bool
}

pub (crate) struct UboundToConvert<'a, MTL:MantissaBackend, MTR:MantissaBackend>(pub (crate) (&'a UBound<MTL,MTR>, Endpoint));

impl<'a, MTL, MTR> Into<NonNaNBound> for UboundToConvert<'a, MTL, MTR>
where MTL: MantissaBackend,
      MTR: MantissaBackend
{   
    fn into(self) -> NonNaNBound {
        let ubound = self.0.0;
        let left_or_right_endpoint  = &self.0.1;

        let mut mpfr_float  = MaybeUninit::uninit();
        let mut open : bool = false;

        unsafe{

            if ubound.is_inf(left_or_right_endpoint){
                open = true;

                mpfr::init2(mpfr_float.as_mut_ptr(), ubound.precision(left_or_right_endpoint) as i64);
                mpfr::set_inf(mpfr_float.as_mut_ptr(), ubound.mpfr_sign(left_or_right_endpoint));
            } 
            
            else if ubound.is_inexact(left_or_right_endpoint){
                open = true;

                // If we are at the most positive or negative number and inexact, the bound should be infinite
                if ubound.is_most_positive_or_negative(left_or_right_endpoint){
                    mpfr::init2(mpfr_float.as_mut_ptr(), ubound.precision(left_or_right_endpoint) as i64);
                    mpfr::set_inf(mpfr_float.as_mut_ptr(), ubound.mpfr_sign(left_or_right_endpoint))
                } 

                // Otherwise, we need to handle the cases where we've added a ULP.
                // An inexact unum on the left represents (L, L + 1ULP)  => Taking the leftmost (smaller) bound means using the exact value
                // An inexact unum on the right represents (R, R + 1ULP) => Taking the rightmost (larger)  bound means using the +ULP value
                else{
                    // NaN and Inf cases already handled, so we can safely unwrap
                    mpfr_float = match left_or_right_endpoint{
                        // Smaller of the two numbers represented by the inexact left unum
                        Endpoint::Left  =>  ubound.left.unwrap().into(),

                        // Larger of the two numbers represented by the inexact right unum
                        Endpoint::Right => ubound.right.unwrap().plus_one_ulp().into()
                    };
                }
            } 

            // We have a finite, exact unum
            else{
                mpfr_float = match left_or_right_endpoint{
                    Endpoint::Left  =>  ubound.left.unwrap().into(),
                    Endpoint::Right => ubound.right.unwrap().into()
                };
            }
        }

        NonNaNBound{
            endpoint : mpfr_float,
            open    : open
        }
    }
    
}

