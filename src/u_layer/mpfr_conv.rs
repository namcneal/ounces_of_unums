use crate::u_layer::unums::*;
use std::mem::MaybeUninit;
use bitvec::{bitarr, prelude::BitArray};
use gmp_mpfr_sys::mpfr;

use super::backend_reprs::MPFR_BITS_PER_LIMB_as_PREC_T;

type MPFRFloatPtr = MaybeUninit<gmp_mpfr_sys::mpfr::mpfr_t>;

impl<const Limbs: usize> Into<MPFRFloatPtr> for Unum<Limbs>{
    fn into(self) -> MaybeUninit<gmp_mpfr_sys::mpfr::mpfr_t> {
        let mut mpfr_float = MaybeUninit::uninit();

        unsafe{
            mpfr::init2(mpfr_float.as_mut_ptr(), self.mpfr_precision());

            match self.extra{
                // NaN
                UNUM_NAN_MASK => (),

                // Inf 
                UNUM_INF_MASK => mpfr::set_inf(mpfr_float.as_mut_ptr(), self.mpfr_sign()),

                // Zero
                _ => match self.is_zero(){
                    true => mpfr::set_zero(mpfr_float.as_mut_ptr(), self.mpfr_sign()),

                // Everything else
                    _    => {
                        mpfr::set_ui(mpfr_float.as_mut_ptr(), 1, mpfr::rnd_t::RNDN);

                        let old_exponent = mpfr::get_exp(mpfr_float.as_mut_ptr());
                        let new_exponent = old_exponent + self.mpfr_exponent();
                        mpfr::set_exp(mpfr_float.as_mut_ptr(), new_exponent);

                        // Even though this is fine, this should be fine as the values should always be positive
                        // let total_num_limbs = (self.mpfr_precision()).div_ceil(MPFR_BITS_PER_LIMB_as_PREC_T);
                        
                        let mantissa_c_array = mpfr_float.assume_init().d;
                        
                        // iterate through the C array
            
                        for (idx, bitarray_backend_chunk) in self.mantissa.unwrap().as_raw_slice().iter().enumerate(){
                            let next_element_in_mpfr_c_array = mantissa_c_array.as_ptr()
                                                                               .offset(idx as isize)
                                                                               .as_mut().unwrap();
                            
                            *next_element_in_mpfr_c_array = *bitarray_backend_chunk;
                        }
                    }
                }
            }

        }

        return mpfr_float
    }
}

impl<const Limbs: usize> From<MPFRFloatPtr> for Unum<Limbs>{
    fn from(value: MPFRFloatPtr) -> Unum<Limbs> {
        let empty_unum = Unum::< Limbs>::empty();
        
        unsafe{
            if mpfr::nan_p(value.as_ptr()) != 0{
                return Unum::nan();     

            } else if mpfr::inf_p(value.as_ptr()) != 0{
                if mpfr::sgn(value.as_ptr()) > 0{
                    return Unum::posinf();
                } else{
                    return Unum::neginf();
                }
            } else{
                
                let precision = mpfr::get_prec(value.as_ptr());
                let num_limbs = precision.div_ceil(MPFR_BITS_PER_LIMB_as_PREC_T);
                let mantissa_c_array = value.assume_init().d;

                let mantissa = std::slice::from_raw_parts(mantissa_c_array.as_ptr(), num_limbs as usize);

                Unum::<Limbs>{
                    mantissa : BitArray::from(mantissa.try_into()),

                }
            }
        }
    }
}



