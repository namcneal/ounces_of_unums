
use crate::mpfr_glue::prelude::*;
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unums::*;
use crate::u_layer::ubounds::*;
use crate::g_layer::one_sided_bound::*;


use std::mem::MaybeUninit;
use std::mem;
use std::num::TryFromIntError;
use gmp_mpfr_sys::mpfr;

type LeftOpen_RightOpen = u8;
pub (crate)const LEFT_OPEN_MASK  : LeftOpen_RightOpen = 0b1000_0000;
pub (crate)const RIGHT_OPEN_MASK : LeftOpen_RightOpen = 0b0000_1000;
pub (crate) struct IntervalOpenness(pub (crate) LeftOpen_RightOpen);

pub (crate) struct Gbound{
    pub (crate) left  : MPFRFloatPtr,
    pub (crate) right : MPFRFloatPtr,
    pub (crate) open  : IntervalOpenness
}

impl Drop for Gbound{
    fn drop(&mut self) {
    unsafe{
        mpfr::clear(self.left.as_mut_ptr());
        mpfr::clear(self.right.as_mut_ptr());
    }}
}

impl<MT1,MT2> Into<Gbound> for Ubound<MT1,MT2>
where MT1: MantissaBackend,
      MT2: MantissaBackend
{
    fn into(self) -> Gbound {    
    unsafe{
        // Handle the NaN cases first:
        if self.left.is_nan() || self.right.is_nan(){
            return Gbound{
                left  : self.left.into(),
                right : self.right.into(),
                open  : IntervalOpenness(LEFT_OPEN_MASK | RIGHT_OPEN_MASK)
            }
        } else{
            let mut left_bound  : NonNaNBound = self.left.into();
            let mut right_bound : NonNaNBound = self.right.into();
            let mut left_right_open : LeftOpen_RightOpen = 0;

            // Check to see whether the order is wrong
            if mpfr::greater_p(left_bound.endpoint.as_ptr(), right_bound.endpoint.as_ptr()) > 0{
                (left_bound, right_bound) = (right_bound, left_bound)
            }

            if left_bound.open{
                left_right_open |= LEFT_OPEN_MASK;
            }
            if right_bound.open{
                left_right_open |= RIGHT_OPEN_MASK;
            }

            return Gbound{
                left : left_bound.endpoint,
                right: right_bound.endpoint,
                open : IntervalOpenness(left_right_open)
            }

        }
    }}
}


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
impl From<&Gbound> for Ubound<int_left, int_right>
{
    fn from(value: &Gbound) -> Self {
        // Round the left-endpoint down and the right endpoint up
        let mut unum_left  : Unum<int_left>  = MPFRForConversion(value.left,  mpfr::rnd_t::RNDD).into();
        let mut unum_right : Unum<int_right> = MPFRForConversion(value.right, mpfr::rnd_t::RNDU).into();

        // The u-bits might already be set due to rounding above, but we still
        // need to handle the case that the conversion was exact, but the bounds were open
        if value.open.0 & LEFT_OPEN_MASK > 0{
            unum_left.extra |= UNUM_UBIT_MASK;
        }
        if value.open.0 & RIGHT_OPEN_MASK > 0{
            unum_right.extra |= UNUM_UBIT_MASK;
        }

        Ubound { 
            left  : unum_left, 
            right : unum_right 
        }
    }
}



