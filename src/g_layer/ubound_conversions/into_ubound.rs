use crate::mpfr_glue::prelude::*;
use crate::u_layer::ubounds::*;
use crate::u_layer::unsigned_floats::*;
use crate::u_layer::utag::*;
use crate::g_layer::gbounds::*;

use gmp_mpfr_sys::mpfr;

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
 impl From<&GBound> for UBound<int_left, int_right>
 {
     fn from(gbound: &GBound) -> Self {
     unsafe{
         // Handle the NaN cases
         if mpfr::nan_p(gbound.left.as_ptr()) > 0 || mpfr::nan_p(gbound.right.as_ptr()) > 0{
             return UBound::<int_left, int_right>::nan();
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
         if (gbound.open.0 & LEFT_OPEN_MASK) > 0{
             utag |= LEFT_UBIT_MASK;
         }
         if (gbound.open.0 & RIGHT_OPEN_MASK) > 0{
             utag |= RIGHT_UBIT_MASK;
         }
 
         if mpfr::sgn(gbound.left.as_ptr()) < 0{
             utag |= LEFT_SIGN_MASK;
         }
         if mpfr::sgn(gbound.right.as_ptr()) < 0{
             utag |= RIGHT_SIGN_MASK;
         }
 
         UBound { 
             left  : ubound_left, 
             right : ubound_right,
             utag  : UTag(utag)
         }
     }}
 }
 
 
 
 