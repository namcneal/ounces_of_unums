use crate::u_layer::backend_reprs::DefaultExponentBackend;

use std::mem::MaybeUninit;
use gmp_mpfr_sys::mpfr;


pub (crate) type MPFRBitOrder         = bitvec::prelude::Msb0;
pub (crate) type MPFRPrecisionBackend = gmp_mpfr_sys::mpfr::prec_t;
pub (crate) type MPFRExponentBackend  = core::ffi::c_long;
pub (crate) type MPFRLimbBackend      = gmp_mpfr_sys::gmp::limb_t;

// conversion to the precision type to save us from doing this later
// when the two need to be divided to compute the number of limbs
pub (crate) const MPFR_BITS_PER_LIMB           : usize                = 8 * std::mem::size_of::<MPFRLimbBackend>();
pub (crate) const MPFR_BITS_PER_LIMB_as_PREC_T : MPFRPrecisionBackend = MPFR_BITS_PER_LIMB as MPFRPrecisionBackend;

pub (crate) type MPFRFloat    = mpfr::mpfr_t;
pub (crate) type MPFRFloatPtr = MaybeUninit<gmp_mpfr_sys::mpfr::mpfr_t>;

pub (crate) fn mpfr_exponent_offset_from_zero(precision: i64) -> DefaultExponentBackend{
    unsafe{
        let mut one = MaybeUninit::uninit();
        mpfr::init2(one.as_mut_ptr(), precision);

        let rounded = mpfr::set_ui(one.as_mut_ptr(), 1, mpfr::rnd_t::RNDD);
        assert!(rounded == 0);

        // The exponent on one should be zero, so anything else is an offset. 
        let exponent_offset = mpfr::get_exp(one.as_ptr());
        mpfr::clear(&mut one.assume_init());

        exponent_offset

    }
}

