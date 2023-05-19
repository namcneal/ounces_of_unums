
use crate::mpfr_glue::prelude::*;
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unsigned_float::*;
use crate::u_layer::utag::*;
use crate::u_layer::ubounds::*;
use crate::g_layer::non_nan_bounds::*;


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
        if self.is_nan(){
            let mut left  = MaybeUninit::uninit();
            let mut right = MaybeUninit::uninit();
            mpfr::init2(left.as_mut_ptr(), <MT1 as MantissaBackend>::precision() as i64);
            mpfr::init2(right.as_mut_ptr(), <MT2 as MantissaBackend>::precision() as i64);
    
            return Gbound{
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

            return Gbound{
                left :  left_endpoint.endpoint,
                right: right_endpoint.endpoint,
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
    fn from(gbound: &Gbound) -> Self {
    unsafe{
        // Handle the NaN cases
        if mpfr::nan_p(gbound.left.as_ptr()) > 0 || mpfr::nan_p(gbound.right.as_ptr()) > 0{
            return Ubound::<int_left, int_right>::nan();
        }

        let mut utag : u8 = 0;

        let ubound_left : Option<UnsignedFloat<int_left>>;
        if mpfr::inf_p(gbound.left.as_ptr()) > 0{
            ubound_left = None;
            utag  |= LEFT_INF_MASK;
        } else{
            let converted : ConvertedMPFR<int_left> = MPFRForConversion(gbound.left,  mpfr::rnd_t::RNDD).into();
            ubound_left = Some( converted.0 );
            if converted.1{
                utag |= LEFT_UBIT_MASK;
            }
        }

        let ubound_right : Option<UnsignedFloat<int_right>>;
        if mpfr::inf_p(gbound.right.as_ptr()) > 0{
            ubound_right = None;
            utag  |= RIGHT_INF_MASK;
        } else{
            let converted : ConvertedMPFR<int_right> = MPFRForConversion(gbound.right,  mpfr::rnd_t::RNDU).into();
            ubound_right = Some( converted.0 );
            if converted.1{
                utag |= RIGHT_UBIT_MASK;
            }
        }

        // The u-bits might already be set due to rounding above, but we still
        // need to handle the case that the conversion was exact, but the bounds were open
        if gbound.open.0 & LEFT_OPEN_MASK > 0{
            utag |= LEFT_UBIT_MASK;
        }
        if gbound.open.0 & RIGHT_OPEN_MASK > 0{
            utag |= RIGHT_UBIT_MASK;
        }

        if mpfr::sgn(gbound.left.as_ptr()) < 0{
            utag |= LEFT_SIGN_MASK;
        }


        if mpfr::sgn(gbound.right.as_ptr()) < 0{
            utag |= RIGHT_SIGN_MASK;
        }

        Ubound { 
            left  : ubound_left, 
            right : ubound_right,
            utag  : UTag(utag)
        }
    }}
}



