use crate::u_layer::backend_reprs::*;
use bitvec::prelude::*;
use num_traits::Zero;
use gmp_mpfr_sys::mpfr;
use std::mem::MaybeUninit;
use std::fmt;

//   Structure of the Universal Numbers implemented here:
//
//   |---------- fraction bits ---------||----exponent----||SUNI 0000|
//            unsigned int                   signed int      u8 bits
//
// SNUI -> ubit, NaN, and infinite convenience bits


pub enum UnsignedFloatSizes{
    XS,
    S,
    M,
    L,
    XL
}

#[derive(Debug, Copy, Clone)]
pub struct UnsignedFloat<MT: MantissaBackend>
{
    pub mantissa : MT,
    pub exponent : DefaultExponentBackend,
}

impl<MT> UnsignedFloat<MT>
where MT: MantissaBackend
{
    pub fn plus_one_ulp(&self) -> UnsignedFloat<MT>{
        let mut new_unum = self.clone();
        new_unum.mantissa = self.mantissa + MT::one();
        new_unum
    }

        // Zero is represented by anything with a zero mantissa.
    // No canonical choice, but I try to implement a zero exponent
    pub fn zero() -> UnsignedFloat<MT>{
        UnsignedFloat { 
            mantissa: MT::zero(), 
            exponent: DefaultExponentBackend::zero(), 
        }
    }

    pub fn is_zero(&self) -> bool{
        self.mantissa == MT::zero()
    }

    pub fn largest() -> UnsignedFloat<MT>{
        UnsignedFloat { 
            mantissa: MT::max_value(), 
            exponent: DefaultExponentBackend::MAX, 
        }
    }

    pub fn precision(&self) -> usize {
        <MT as MantissaBackend>::precision()
    }

    pub (crate) fn mpfr_precision(&self) -> gmp_mpfr_sys::mpfr::prec_t{
        self.precision() as gmp_mpfr_sys::mpfr::prec_t
    }

    pub (crate) fn mpfr_exponent(&self) -> gmp_mpfr_sys::mpfr::exp_t{
        self.exponent
    }
}

use std::ffi::c_long;

impl<MT> std::fmt::Display for UnsignedFloat<MT>
where MT: MantissaBackend,
{       
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert the mantissa to a binary string representation
        let mantissa_str = format!("{:b}", self.mantissa);

        // Determine the number of digits before the decimal point
        let num_digits = if self.exponent >= 0 {
            self.exponent as usize + 1
        } else {
            1
        };

        todo!()
    }
}






