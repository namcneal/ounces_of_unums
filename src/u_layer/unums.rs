use crate::u_layer::backend_reprs::*;
use bitvec::prelude::*;
use num_traits::Zero;
use gmp_mpfr_sys::mpfr;
use std::mem::MaybeUninit;

//   Structure of the Universal Numbers implemented here:
//
//   |---------- fraction bits ---------||----exponent----||SUNI --type--|
//            unsigned int                   signed int      u8 bits
//
// SNUI -> ubit, NaN, and infinite convenience bits


type Sign_UBit_NaN_Inf_Type = u8;

#[derive(Copy, Clone)]
pub struct Unum<const Limbs: usize>
{
    pub mantissa : Option<DefaultMantissaBackend<Limbs>>,
    pub exponent : DefaultExponentBackend,
    pub extra    : Sign_UBit_NaN_Inf_Type,
}

pub (crate) const UNUM_SIGN_MASK : u8 = 0b1000_0000;
pub (crate) const UNUM_UBIT_MASK : u8 = 0b0100_0000;
pub (crate) const UNUM_NAN_MASK  : u8 = 0b0010_0000;
pub (crate) const UNUM_INF_MASK  : u8 = 0b0001_0000;

impl<const L: usize> Unum<L>{

    pub (crate) fn empty() -> Unum<L>{
        Unum { mantissa: None,
               exponent: DefaultExponentBackend::MAX, 
               extra: 0 }
    }

    pub fn nan() -> Unum<L>{
        let mut nan = Self::empty();
        nan.extra = UNUM_NAN_MASK;
        nan
    }

    pub fn posinf() -> Unum<L>{
        let mut inf = Self::empty();
        inf.extra = UNUM_INF_MASK;
        inf
    }

    pub fn neginf() -> Unum<L>{
        let mut inf = Self::empty();
        inf.extra  = UNUM_INF_MASK;
        inf.extra |= UNUM_SIGN_MASK;
        inf
    }

    pub (crate) fn is_zero(&self) -> bool{
        match self.mantissa{
            Some(mantissa) => mantissa[0],
            _ => false
        }
    }

    pub (crate) fn mpfr_precision(&self) -> gmp_mpfr_sys::mpfr::prec_t{
        match self.mantissa{
            Some(mantissa) =>         
                    // Conversion okay because the precision should always be <<<< its maximum allowed value
                    mantissa.len() as gmp_mpfr_sys::mpfr::prec_t,

            _ => 0
        }

    }

    pub (crate) fn mpfr_exponent(&self) -> gmp_mpfr_sys::mpfr::exp_t{
        self.exponent
    }

    pub (crate) fn mpfr_sign(&self) -> core::ffi::c_int{
        (self.extra & 0b1000_0000) as core::ffi::c_int
    }
}

