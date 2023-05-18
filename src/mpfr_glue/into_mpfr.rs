use crate::mpfr_glue::mpfr_types::*;
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unums::*;
use crate::u_layer::ubounds::*;
use crate::g_layer::gbounds::*;

use std::mem::MaybeUninit;
use bitvec::{bitarr, prelude::BitArray};
use gmp_mpfr_sys::{mpfr, mpc::RNDAN};

type Inexact = bool;
struct UnumConversionResult(MPFRFloatPtr, Inexact);

impl<MT> Into<MPFRFloat> for Unum<MT>
where MT: MantissaBackend
{
    fn into(self) -> MPFRFloat {
        unsafe{
            <Unum<MT> as Into<MPFRFloatPtr>>::into(self).assume_init()
        }
    }
}

impl<MT> Into<MPFRFloatPtr> for Unum<MT>
where MT: MantissaBackend
{
    fn into(self) -> MPFRFloatPtr{
        let unum = self;
        let mut mpfr_float = MaybeUninit::uninit();
        

        unsafe{
            mpfr::init2(mpfr_float.as_mut_ptr(), unum.mpfr_precision());

            match unum.extra{
                // NaN
                UNUM_NAN_MASK => (),

                // Inf 
                UNUM_INF_MASK => mpfr::set_inf(mpfr_float.as_mut_ptr(), unum.mpfr_sign()),

                // Zero
                _ => match unum.is_zero(){
                    true => mpfr::set_zero(mpfr_float.as_mut_ptr(), 0),

                // Everything else
                    _    => {
                        // Set the number to 1.00000... x 2^0 first 
                        let has_been_rounded = mpfr::set_ui(mpfr_float.as_mut_ptr(), 1, mpfr::rnd_t::RNDZ);
                        assert!(has_been_rounded == 0); // One should always be exactly expressable
                        
                        let exponent_offset_from_zero = mpfr::get_exp(mpfr_float.as_mut_ptr());
                        
                        let new_exponent = unum.mpfr_exponent() - exponent_offset_from_zero;
                        mpfr::set_exp(mpfr_float.as_mut_ptr(), new_exponent);
                        
                        let mut mantissa_c_array = mpfr_float.assume_init().d.as_ptr();
                        
                        match unum.size(){
                            UnumSize::Null   => panic!("An unum with no mantissa should not have gotten this far"),
                            UnumSize::Gallon => {
                                let first_slice  = (unum.mantissa.unwrap() >> 64).try_into();
                                let second_slice = (unum.mantissa.unwrap() >> 0).try_into();
                                match (first_slice, second_slice){
                                    (Ok(first), Ok(second)) => {
                                        *mantissa_c_array.as_mut().unwrap() = first;

                                        *mantissa_c_array.offset(1)
                                                         .as_mut()
                                                         .unwrap() = second;
                                    },
                                    _ => panic!("Could not convert Unum to MPFR_T.")
                                }
                            },
                                
                            _ => {
                                let mantissa = unum.mantissa.unwrap().try_into();
                                match mantissa{
                                    Ok(mantissa) => {
                                        *mantissa_c_array.as_mut().unwrap() = mantissa;
                                    },

                                    _ => panic!("Error: Could not convert Unum to MPFR_T")
                                
                                }
                            }
                        };
                    }
                }
            }
        }

        return mpfr_float
    }
}

#[duplicate::duplicate_item(
    mantissa_type;
    [ u8 ];
    [ u16 ];
    [ u32 ];
    [ u64 ];
)]
impl From<MPFRFloatPtr> for Unum<mantissa_type>{
    fn from(value: MPFRFloatPtr) -> Unum<mantissa_type> {        
        unsafe{

            // NaN
            if mpfr::nan_p(value.as_ptr()) != 0{
                return Unum::nan();     

            // Inf
            } else if mpfr::inf_p(value.as_ptr()) != 0{
                if mpfr::sgn(value.as_ptr()) > 0{
                    return Unum::posinf();
                } else{
                    return Unum::neginf();
                }

            // everything else
            } else{
                let mut rounded_mpfr = MaybeUninit::uninit();
                let target_precision = 8 * std::mem::size_of::<mantissa_type>();
                
                mpfr::init2(rounded_mpfr.as_mut_ptr(), target_precision as i64);
                // TODO: make this rounding more selectable, perhaps using the last for bits in the extra unum tag
                let rounding_direction = mpfr::set(rounded_mpfr.as_mut_ptr(), value.as_ptr(), gmp_mpfr_sys::mpfr::rnd_t::RNDA);

                let left_padded_mantissa : MPFRLimbBackend = *rounded_mpfr.assume_init().d.as_ptr().as_ref().unwrap();
                println!("{:b}", left_padded_mantissa);
                
                let mantissa : mantissa_type = (left_padded_mantissa >> (MPFR_BITS_PER_LIMB - target_precision)) as mantissa_type;
                println!("{:b}", mantissa);

                let mut extra_bits : u8 = 0;
                if rounding_direction != 0{
                    extra_bits |= UNUM_UBIT_MASK;
                }
                if mpfr::sgn(rounded_mpfr.as_ptr()) < 0{
                    extra_bits |= UNUM_SIGN_MASK;
                }

                let exponent = mpfr::get_exp(rounded_mpfr.as_ptr());
                mpfr::clear(rounded_mpfr.as_mut_ptr());

                return Unum::<mantissa_type>{
                    mantissa : Some(mantissa),
                    exponent : exponent - 1,
                    extra    : extra_bits
                }

            }
        }
    }
}



