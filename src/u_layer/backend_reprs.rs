use crate::mpfr_glue::prelude::*;
use duplicate;
use num_traits::Zero;

pub trait MantissaBackend : std::ops::Shr<usize> + 
                            num_traits::PrimInt  + 
                            TryInto<u64>         +
                            std::fmt::Debug
{
    fn precision() -> usize;
}

#[duplicate::duplicate_item(
    int_type;
    [ u8 ];
    [ u16 ];
    [ u32 ];
    [ u64 ];
    [ u128];
)]
impl MantissaBackend for int_type{
    fn precision() -> usize{
        8 * std::mem::size_of::<int_type>()
    }
}

pub type DefaultExponentBackend = MPFRExponentBackend;


