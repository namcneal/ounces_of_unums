
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



impl<const L1: usize, const L2: usize> From<UBound<L1,L2>> for GBound{
    fn from(ubound: UBound<L1,L2>) -> Self {
        let uleft  : UnumBase<L1> = ubound.left;
        let uright : UnumBase<L2> = ubound.right;
        

        todo!()
    }
}
