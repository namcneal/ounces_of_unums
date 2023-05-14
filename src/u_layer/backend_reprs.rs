use duplicate;
use num_traits::Zero;

pub (crate) type MPFRBitOrder         = bitvec::prelude::Msb0;
pub (crate) type MPFRPrecisionBackend = gmp_mpfr_sys::mpfr::prec_t;
pub (crate) type MPFRExponentBackend  = core::ffi::c_long;
pub (crate) type MPFRLimbBackend      = gmp_mpfr_sys::gmp::limb_t;

// conversion to the precision type to save us from doing this later
// when the two need to be divided to compute the number of limbs
pub (crate) const MPFR_BITS_PER_LIMB           : usize                = std::mem::size_of::<MPFRLimbBackend>();
pub (crate) const MPFR_BITS_PER_LIMB_as_PREC_T : MPFRPrecisionBackend = MPFR_BITS_PER_LIMB as MPFRPrecisionBackend;


pub type DefaultMantissaBackend<const Limbs: usize> = bitvec::prelude::BitArray::<[MPFRLimbBackend; Limbs]>;

// pub trait MantissaBackend  {}
// #[duplicate::duplicate_item(
//     int_type;
//     [ u8 ];
//     [ u16 ];
//     [ u32 ];
//     [ u64 ];
//     [ u128];
// )]
// impl MantissaBackend for int_type{}


pub type DefaultExponentBackend = MPFRExponentBackend;


