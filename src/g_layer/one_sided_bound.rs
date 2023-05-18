use crate::mpfr_glue::prelude::*;
use crate::u_layer::{unums::*, backend_reprs::MantissaBackend};
use gmp_mpfr_sys::mpfr;

pub (super) struct NonNaNBound{
    pub (super) endpoint : MPFRFloatPtr, // Assume that this is NOT NaN
    pub (super) open     : bool
}

impl<MT> Into<NonNaNBound> for Unum<MT>
where MT: MantissaBackend
{   
    fn into(self) -> NonNaNBound {
        let endpoint : MPFRFloatPtr;
        let open : bool;

        // Check for infinity 
        if self.is_inf(){
            endpoint = self.into(); // Conversion handled in the trait implementation
            open     = true;
        } 

        // Now check for a finite, inexact number
        else if self.is_inexact(){
            open = true; // Inexact => open endpoint


            // If we are at the most positive or negative number and inexact, the bound should be infinite
            if self.is_most_positive_or_negative(){
                endpoint = match self.is_positive(){
                    true  => Unum::<MT>::posinf().into(),
                    false => Unum::<MT>::neginf().into()
                };
            } 
            // Otherwise, we need to add one ULP to get the "next largest" bound
            else{
                endpoint = self.plus_one_ulp().into();
            }
        } 

        // We have a finite, exact unum
        else{
            endpoint = self.into();
            open     = false;
        }

        NonNaNBound{
            endpoint : endpoint,
            open    : open
        }
    }
    
}

