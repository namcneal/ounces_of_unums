
use crate::u_layer::backend_reprs::*;
use crate::u_layer::unums::*;


pub (crate) struct UBound<MT1,MT2>
where MT1: MantissaBackend,
      MT2: MantissaBackend
{
    pub (crate) left  : Unum<MT1>,
    pub (crate) right : Unum<MT2>

}


