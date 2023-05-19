use std::mem::MaybeUninit;

use crate::mpfr_glue::prelude::*;
use crate::u_layer::ubounds::*;
use crate::u_layer::backend_reprs::*;
use gmp_mpfr_sys::mpfr;

pub (super) struct NonNaNBound{
    pub (super) endpoint : MPFRFloatPtr, // Assume that this is NOT NaN
    pub (super) open     : bool
}

pub (crate) struct UboundToConvert<'a, MTL:MantissaBackend, MTR:MantissaBackend>(pub (crate) (&'a Ubound<MTL,MTR>, Endpoint));

impl<'a, MTL, MTR> Into<NonNaNBound> for UboundToConvert<'a, MTL, MTR>
where MTL: MantissaBackend,
      MTR: MantissaBackend
{   
    fn into(self) -> NonNaNBound {
        let ubound = self.0.0;
        let which_endpoint  = &self.0.1;

        let mut mpfr_float  = MaybeUninit::uninit();
        let mut open : bool = false;

        unsafe{

            if ubound.is_inf(which_endpoint){
                mpfr::init2(mpfr_float.as_mut_ptr(), ubound.precision(which_endpoint) as i64);
                mpfr::set_inf(mpfr_float.as_mut_ptr(), ubound.mpfr_sign(which_endpoint));
                open = true;
            } 
            
            else if ubound.is_inexact(which_endpoint){
                open = true;

                // If we are at the most positive or negative number and inexact, the bound should be infinite
                if ubound.is_most_positive_or_negative(which_endpoint){
                    mpfr::init2(mpfr_float.as_mut_ptr(), ubound.precision(which_endpoint) as i64);
                    mpfr::set_inf(mpfr_float.as_mut_ptr(), ubound.mpfr_sign(which_endpoint))
                } 

                // Otherwise, we need to add one ULP to get the "next largest" bound
                else{
                    // NaN and Inf cases already handled, so we can safely unwrap
                    mpfr_float = match which_endpoint{
                        Endpoint::Left  =>  ubound.left.unwrap().plus_one_ulp().into(),
                        Endpoint::Right => ubound.right.unwrap().plus_one_ulp().into()
                    };
                }
            } 

            // We have a finite, exact unum
            else{
                mpfr_float = match which_endpoint{
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

