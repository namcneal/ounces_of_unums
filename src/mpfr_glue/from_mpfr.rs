use crate::mpfr_glue::mpfr_types::*;
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unsigned_float::*;
use crate::u_layer::ubounds::*;
use crate::g_layer::gbounds::*;

use std::mem::MaybeUninit;
use bitvec::{bitarr, prelude::BitArray};
use gmp_mpfr_sys::{mpfr, mpc::RNDAN};

type RoundingDirection = mpfr::rnd_t;
pub (crate) struct MPFRForConversion(pub (crate) MPFRFloatPtr, pub (crate) RoundingDirection);

type RoundedDuringConversion = bool;
pub (crate) struct ConvertedMPFR<MT: MantissaBackend>(pub (crate) UnsignedFloat<MT>, pub (crate) RoundedDuringConversion);

#[duplicate::duplicate_item(
    mantissa_type;
    [ u8 ];
    [ u16 ];
    [ u32 ];
    [ u64 ];
)]
impl From<MPFRForConversion> for UnsignedFloat<mantissa_type>{
    fn from(_value: MPFRForConversion) -> UnsignedFloat<mantissa_type> {     
        let value = _value.0;
        let rounding_direction = _value.1;   
        let mut rounded_during_conversion = false;

        unsafe{
            let mut rounded_mpfr = MaybeUninit::uninit();
            let target_precision = <mantissa_type as MantissaBackend>::precision();
            mpfr::init2(rounded_mpfr.as_mut_ptr(), target_precision as i64);

            let rounded = mpfr::set(rounded_mpfr.as_mut_ptr(), value.as_ptr(), rounding_direction);
            if rounded != 0{
                rounded_during_conversion = true;
            }

            let left_padded_mantissa : MPFRLimbBackend = *rounded_mpfr.assume_init().d.as_ptr().as_ref().unwrap();
            println!("MPFR mantissa with left-padding {:b}", left_padded_mantissa);
            
            let mantissa : mantissa_type = (left_padded_mantissa >> (MPFR_BITS_PER_LIMB - target_precision)) as mantissa_type;
            println!("Mantissa with the precision we need {:b}", mantissa);

            let exponent = mpfr::get_exp(rounded_mpfr.as_ptr()) - mpfr_exponent_offset_from_zero(target_precision as i64);
            println!("Exponent: {:?}", mpfr_exponent_offset_from_zero(target_precision as i64));
            mpfr::clear(rounded_mpfr.as_mut_ptr());

            return UnsignedFloat::<mantissa_type>{
                mantissa : mantissa,
                exponent : exponent 
            }
        }
    }
}



