
use crate::mpfr_glue::prelude::*;

use gmp_mpfr_sys::mpfr;

pub (super) type LeftOpen_RightOpen = u8;
pub (crate)const LEFT_OPEN_MASK  : LeftOpen_RightOpen = 0b1000_0000;
pub (crate)const RIGHT_OPEN_MASK : LeftOpen_RightOpen = 0b0000_1000;
pub (crate) struct IntervalOpenness(pub (crate) LeftOpen_RightOpen);

pub (crate) struct GBound{
    pub (crate) left  : MPFRFloatPtr,
    pub (crate) right : MPFRFloatPtr,
    pub (crate) open  : IntervalOpenness
}

impl Drop for GBound{
    fn drop(&mut self) {
    unsafe{
        mpfr::clear(self.left.as_mut_ptr());
        mpfr::clear(self.right.as_mut_ptr());
    }}
}

