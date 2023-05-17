
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unums::*;
use crate::u_layer::ubounds::*;

use std::mem::MaybeUninit;
use std::mem;
use std::num::TryFromIntError;
use gmp_mpfr_sys::mpfr;

type LeftOpen_RightOpen = u8;
struct IntervalOpenness(LeftOpen_RightOpen);

pub (crate) struct Gbound{
    left  : mpfr::mpfr_t,
    right : mpfr::mpfr_t,
    open  : IntervalOpenness
}


impl<MT1,MT2> Into<Gbound> for Ubound<MT1,MT2>
where MT1: MantissaBackend,
      MT2: MantissaBackend
{
    fn into(self) -> Gbound {
        let left  = self.left;
        let right = self.right; 
    }
}


