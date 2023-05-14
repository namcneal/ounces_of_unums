
use crate::u_layer::unums::*;
use crate::u_layer::ubounds::*;

use std::mem::MaybeUninit;
use std::mem;
use std::num::TryFromIntError;
use gmp_mpfr_sys::mpfr;

type LeftOpen_RightOpen = u8;
struct IntervalOpenness(LeftOpen_RightOpen);

struct GBound{
    left  : mpfr::mpfr_t,
    right : mpfr::mpfr_t,
    open  : IntervalOpenness
}

