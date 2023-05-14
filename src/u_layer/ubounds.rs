
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unums::*;


pub (crate) struct UBound<const L1: usize, const L2: usize>
{
    pub (crate) left  : Unum<L1>,
    pub (crate) right : Unum<L2>

}


