
use crate::u_layer::backend_reprs::*;
use crate::u_layer::utag::*;
use crate::u_layer::unsigned_float::*;

#[derive(Debug,Copy, Clone)]
pub (crate) struct Ubound<MTL,MTR>
where MTL: MantissaBackend,
      MTR: MantissaBackend
{
    pub (crate) left  : Option<UnsignedFloat<MTL>>,
    pub (crate) right : Option<UnsignedFloat<MTR>>,
    pub (crate) utag  : UTag
}

pub (crate) enum Endpoint{
    Left,
    Right
}

use Endpoint::*;
impl<MTL,MTR> Ubound<MTL, MTR>
where MTL: MantissaBackend,
      MTR: MantissaBackend
{
    pub fn precision(&self, endpoint:&Endpoint) -> usize {
        match endpoint{
            Left  => <MTL as MantissaBackend>::precision(),
            Right => <MTR as MantissaBackend>::precision()
        }
    }

    pub fn is_inexact(&self, endpoint:&Endpoint) -> bool{
        match endpoint{
            Left  => self.utag.0 & LEFT_UBIT_MASK > 0,
            Right => self.utag.0 & RIGHT_UBIT_MASK > 0
        }
    }

    pub fn is_exact(&self, endpoint:&Endpoint) -> bool{
        !self.is_inexact(endpoint)
    }

    pub fn is_positive(&self, endpoint:&Endpoint) -> bool{
        match endpoint{
            Left  => self.utag.0  & LEFT_SIGN_MASK > 0,
            Right => self.utag.0 & RIGHT_SIGN_MASK > 0
        }
    }

    pub fn is_most_positive_or_negative(&self, endpoint:&Endpoint) -> bool{
        match endpoint{
            Left  => match self.left{
                    Some(unsigned_float) => 
                        unsigned_float.mantissa == MTL::max_value()  && unsigned_float.exponent  == DefaultExponentBackend::MAX,
                    None => false,
            }
            
            Right => match self.right{
                    Some(unsigned_float) => 
                        unsigned_float.mantissa == MTR::max_value()  && unsigned_float.exponent  == DefaultExponentBackend::MAX,
                    None => false
            }

        }
    }

    pub fn nan() -> Ubound<MTL,MTR>{
        Ubound { 
            left  : None, 
            right : None, 
            utag  : UTag(NAN_MASK)
        }
    }

    pub fn set_nan(&mut self){
        self.left  = None;
        self.right = None;
        self.utag  = UTag(NAN_MASK)
    }


    pub fn is_nan(&self) -> bool{
        self.utag.0 & NAN_MASK > 0 
    }

    pub fn set_posinf(&mut self, endpoint:&Endpoint){
        match endpoint{
            Left => {
                self.left = None;
                self.utag.clear_left();
                self.utag.0 |= LEFT_INF_MASK;
            },

            Right =>{
                self.right = None;
                self.utag.clear_right();
                self.utag.0 |= RIGHT_INF_MASK;
            }
        }
    }

    pub fn set_neginf(&mut self, endpoint:&Endpoint){
        self.set_posinf(endpoint);
        match endpoint{
            Left => self.utag.0  |= LEFT_SIGN_MASK,
            Right => self.utag.0 |= RIGHT_SIGN_MASK
        }
    }

    pub fn is_inf(&self, endpoint:&Endpoint) -> bool{
        match endpoint{
            Left  => self.utag.0 & LEFT_INF_MASK > 0,
            Right => self.utag.0 & RIGHT_INF_MASK > 0
        }
    }

    pub (crate) fn mpfr_sign(&self, endpoint:&Endpoint) -> core::ffi::c_int{
        match endpoint {
            Left  => <u8 as Into<core::ffi::c_int>>::into(self.utag.0 &  LEFT_SIGN_MASK),
            Right => <u8 as Into<core::ffi::c_int>>::into(self.utag.0 & RIGHT_SIGN_MASK)
        }
    }
}

impl<MTL,MTR> std::fmt::Display for Ubound<MTL,MTR>
where MTL: MantissaBackend,
      MTR: MantissaBackend
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nan(){
            return write!(f, "NaN");    
        }

        if self.utag.0 & LEFT_UBIT_MASK > 0{
            write!(f, "("); 
        } else{
            write!(f, "["); 
        }

        if self.utag.0 & LEFT_SIGN_MASK > 0{
            write!(f, "-");
        }

        if self.is_inf(&Endpoint::Left){
            write!(f, "∞");
        } else{
            // NaN and Inf cases handled, so unwrapping is alright
            write!(f, "{}, ", self.left.unwrap());
        }

        if self.utag.0 & RIGHT_SIGN_MASK > 0{
            write!(f, "-");
        }

        if self.is_inf(&Endpoint::Right){
            write!(f, "∞");
        } else{
            // NaN and Inf cases handled, so unwrapping is alright
            write!(f, "{}", self.right.unwrap());
        }

        if self.utag.0 & RIGHT_UBIT_MASK > 0{
            return write!(f, ")"); 
        } else{
            return write!(f, "]"); 
        }

        todo!()
    }
}