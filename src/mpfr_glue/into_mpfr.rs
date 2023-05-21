use crate::mpfr_glue::mpfr_types::*;
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unsigned_floats::*;
use crate::u_layer::ubounds::*;
use crate::g_layer::gbounds::*;

use std::mem::MaybeUninit;
use bitvec::{bitarr, prelude::BitArray};
use gmp_mpfr_sys::{mpfr, mpc::RNDAN};

type Inexact = bool;
struct UnumConversionResult(MPFRFloatPtr, Inexact);

impl<MT> Into<MPFRFloat> for UnsignedFloat<MT>
where MT: MantissaBackend
{
    fn into(self) -> MPFRFloat {
        unsafe{
            <UnsignedFloat<MT> as Into<MPFRFloatPtr>>::into(self).assume_init()
        }
    }
}

impl<MT> Into<MPFRFloatPtr> for UnsignedFloat<MT>
where MT: MantissaBackend
{
    fn into(self) -> MPFRFloatPtr{
        let unsigned_float = self;
        let mut mpfr_float = MaybeUninit::uninit();

        unsafe{
            mpfr::init2(mpfr_float.as_mut_ptr(), unsigned_float.mpfr_precision());

            if unsigned_float.is_zero(){
                mpfr::set_zero(mpfr_float.as_mut_ptr(), 0);
            } 
            
            else {
                // Set the number to 1.00000... x 2^0 first 
                let has_been_rounded = mpfr::set_ui(mpfr_float.as_mut_ptr(), 1, mpfr::rnd_t::RNDZ);
                assert!(has_been_rounded == 0); // One should always be exactly expressable
                
                let exponent_offset_from_zero = mpfr::get_exp(mpfr_float.as_mut_ptr());
                
                let new_exponent = unsigned_float.mpfr_exponent() - exponent_offset_from_zero;
                mpfr::set_exp(mpfr_float.as_mut_ptr(), new_exponent);
                
                let mut mantissa_c_array = mpfr_float.assume_init().d.as_ptr();
                
                match unsigned_float.precision(){
                    128 => {
                        let first_slice  = (unsigned_float.mantissa >> 64).try_into();
                        let second_slice = (unsigned_float.mantissa >> 0).try_into();
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
                        let mantissa = unsigned_float.mantissa.try_into();
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

        return mpfr_float
    }
}
