use crate::u_layer::backend_reprs::*;
use bitvec::prelude::*;
use num_traits::Zero;
use gmp_mpfr_sys::mpfr;
use std::mem::MaybeUninit;

//   Structure of the Universal Numbers implemented here:
//
//   |---------- fraction bits ---------||----exponent----||SUNI 0000|
//            unsigned int                   signed int      u8 bits
//
// SNUI -> ubit, NaN, and infinite convenience bits


type Sign_UBit_NaN_Inf_Type = u8;

#[derive(Debug, Copy, Clone)]
pub struct Unum<MT: MantissaBackend>
{
    pub mantissa : Option<MT>,
    pub exponent : DefaultExponentBackend,
    pub extra    : Sign_UBit_NaN_Inf_Type,
}

pub (crate) const UNUM_SIGN_MASK : u8 = 0b1000_0000;
pub (crate) const UNUM_UBIT_MASK : u8 = 0b0100_0000;
pub (crate) const UNUM_NAN_MASK  : u8 = 0b0010_0000;
pub (crate) const UNUM_INF_MASK  : u8 = 0b0001_0000;

pub type Cup    = Unum<u8>;
pub type Pint   = Unum<u16>;
pub type Quart  = Unum<u32>;
pub type Pottle = Unum<u64>;
pub type Gallon = Unum<u128>;

pub enum UnumSize{
    Null,
    Cup,
    Pint,
    Quart,
    Pottle,
    Gallon
}

impl<MT: MantissaBackend> Unum<MT>{
    pub fn precision(&self) -> usize {
        match self.mantissa{
            None => 0,
            Some(mantissa) => 8 * std::mem::size_of::<MT>()
        }
    }

    pub fn size(&self) -> UnumSize{
        match self.precision(){
            0   => UnumSize::Null,
            8   => UnumSize::Cup,
            16  => UnumSize::Pint,
            32  => UnumSize::Quart,
            64  => UnumSize::Pottle,
            128 => UnumSize::Gallon,
            _   => panic!("Major error. This case should never occur.")
        }
    }

    pub fn is_exact(&self) -> bool{
        !self.is_inexact()
    }
 
    pub fn is_inexact(&self) -> bool{
        (self.extra & UNUM_UBIT_MASK) > 0
    }

    pub fn  plus_one_ulp(&self) -> Unum<MT>{
        let mut new_unum = self.clone();

        // Should also have a mantissa that we can unwrap
        new_unum.mantissa = Some(self.mantissa.unwrap() + MT::one());
        new_unum
    }

    // Zero is represented by anything with a zero mantissa.
    // No canonical choice, but I try to implement a zero exponent
    pub fn zero() -> Unum<MT>{
        Unum { 
            mantissa: Some(MT::zero()), 
            exponent: DefaultExponentBackend::zero(), 
            extra: 0 
        }
    }

    pub fn is_zero(&self) -> bool{
        match self.mantissa{
            Some(mantissa) => mantissa == MT::zero(),
            _ => false
        }
    }

    pub fn is_positive(&self) -> bool{
        self.extra & UNUM_SIGN_MASK == 0
    }

    pub fn most_positive() -> Unum<MT>{
        Unum { 
            mantissa: Some(MT::max_value()), 
            exponent: DefaultExponentBackend::MAX, 
            extra: 0 }
    }

    pub fn is_most_positive_or_negative(&self) -> bool{
        match self.mantissa{
            Some(mantissa) => {
                mantissa      == MT::max_value() && 
                self.exponent == DefaultExponentBackend::MAX
            },
            _ => false
        }
    }

    pub fn most_negative() -> Unum<MT>{
        Unum { 
            mantissa: Some(MT::max_value()), 
            exponent: DefaultExponentBackend::MAX, 
            extra: UNUM_SIGN_MASK }
    }

    pub (crate) fn empty() -> Unum<MT>{
        Unum { mantissa: None,
               exponent: DefaultExponentBackend::MAX, 
               extra: 0 }
    }

    pub fn nan() -> Unum<MT>{
        let mut nan = Self::empty();
        nan.extra = UNUM_NAN_MASK;
        nan
    }

    pub fn is_nan(&self) -> bool{
        self.extra & UNUM_NAN_MASK >0 
    }

    pub fn posinf() -> Unum<MT>{
        let mut inf = Self::empty();
        inf.extra = UNUM_INF_MASK;
        inf
    }

    pub fn neginf() -> Unum<MT>{
        let mut inf = Self::empty();
        inf.extra  = UNUM_INF_MASK;
        inf.extra |= UNUM_SIGN_MASK;
        inf
    }

    pub fn is_inf(&self) -> bool{
        self.extra & UNUM_INF_MASK > 0 
    }

    pub (crate) fn mpfr_precision(&self) -> gmp_mpfr_sys::mpfr::prec_t{
        self.precision() as gmp_mpfr_sys::mpfr::prec_t
    }

    pub (crate) fn mpfr_exponent(&self) -> gmp_mpfr_sys::mpfr::exp_t{
        self.exponent
    }

    pub (crate) fn mpfr_sign(&self) -> core::ffi::c_int{
        (self.extra & 0b1000_0000) as core::ffi::c_int
    }
}

