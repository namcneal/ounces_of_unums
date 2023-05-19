use crate::u_layer::backend_reprs::*;
use crate::u_layer::ubounds::*;
use crate::g_layer::gbounds::*;
use crate::g_layer::non_nan_bounds::*;
use crate::g_layer::gbounds::*;

use std::mem::MaybeUninit;
use gmp_mpfr_sys::mpfr;

impl<MT1,MT2> Into<GBound> for UBound<MT1,MT2>
where MT1: MantissaBackend,
      MT2: MantissaBackend
{
    fn into(self) -> GBound {    
    unsafe{
        // Handle the NaN cases first:
        if self.is_nan(){
            let mut left  = MaybeUninit::uninit();
            let mut right = MaybeUninit::uninit();
            mpfr::init2(left.as_mut_ptr(), <MT1 as MantissaBackend>::precision() as i64);
            mpfr::init2(right.as_mut_ptr(), <MT2 as MantissaBackend>::precision() as i64);
    
            return GBound{
                left  : left,
                right : right,
                open  : IntervalOpenness(LEFT_OPEN_MASK | RIGHT_OPEN_MASK)
            }
        } else{
            let mut left_endpoint  : NonNaNBound = UboundToConvert((&self, Endpoint::Left)).into();
            let mut right_endpoint : NonNaNBound = UboundToConvert((&self, Endpoint::Right)).into();

            // Check to see whether the order is wrong
            if mpfr::greater_p(left_endpoint.endpoint.as_ptr(), right_endpoint.endpoint.as_ptr()) > 0{
                (left_endpoint, right_endpoint) = (right_endpoint, left_endpoint);
            }

            let mut left_right_open : LeftOpen_RightOpen = 0;
            if left_endpoint.open{
                left_right_open |= LEFT_OPEN_MASK;
            } 
            if right_endpoint.open{
                left_right_open |= RIGHT_OPEN_MASK;
            }

            return GBound{
                left :  left_endpoint.endpoint,
                right: right_endpoint.endpoint,
                open : IntervalOpenness(left_right_open)
            }

        }
    }}
}


