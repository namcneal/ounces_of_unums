use crate::u_layer::backend_reprs::*;
use bitvec::prelude::*;
use dashu::integer;
use num_traits::{Zero, FromPrimitive};
use gmp_mpfr_sys::mpfr;
use std::error::Error;
use std::mem::MaybeUninit;
use std::{fmt, num};
use std::str::FromStr;

//   Structure of the Universal Numbers implemented here:
//
//   |---------- fraction bits ---------||----exponent----||SUNI 0000|
//            unsigned int                   signed int      u8 bits
//
// SNUI -> ubit, NaN, and infinite convenience bits


pub enum UnsignedFloatSizes{
    XS,
    S,
    M,
    L,
    XL
}

#[derive(Debug, Copy, Clone)]
pub struct UnsignedFloat<MT: MantissaBackend>
{
    pub mantissa : MT,
    pub exponent : DefaultExponentBackend,
}

impl<MT> UnsignedFloat<MT>
where MT: MantissaBackend
{
    pub fn plus_one_ulp(&self) -> UnsignedFloat<MT>{
        let mut new_unum = self.clone();
        new_unum.mantissa = self.mantissa + MT::one();
        new_unum
    }

    // Zero is represented by anything with a zero mantissa.
    // No canonical choice, but I try to implement a zero exponent
    pub fn zero() -> UnsignedFloat<MT>{
        UnsignedFloat { 
            mantissa: MT::zero(), 
            exponent: DefaultExponentBackend::zero(), 
        }
    }

    pub fn is_zero(&self) -> bool{
        self.mantissa == MT::zero()
    }

    pub fn largest() -> UnsignedFloat<MT>{
        UnsignedFloat { 
            mantissa: MT::max_value(), 
            exponent: DefaultExponentBackend::MAX, 
        }
    }

    pub fn smallest() -> UnsignedFloat<MT>{
        UnsignedFloat { 
            mantissa: MT::one() << <MT as MantissaBackend>::precision() - 1, 
            exponent: DefaultExponentBackend::MIN }
    }

    pub fn precision(&self) -> usize {
        <MT as MantissaBackend>::precision()
    }

    pub (crate) fn mpfr_precision(&self) -> gmp_mpfr_sys::mpfr::prec_t{
        self.precision() as gmp_mpfr_sys::mpfr::prec_t
    }

    pub (crate) fn mpfr_exponent(&self) -> gmp_mpfr_sys::mpfr::exp_t{
        self.exponent
    }
}

impl<MT: MantissaBackend> Into<String> for &UnsignedFloat<MT>{
    fn into(self) -> String {
        // Convert the mantissa to a binary string representation
        let mut mantissa_str = format!("{:b}", self.mantissa);

        if self.exponent >= 0{
            mantissa_str.insert(self.exponent as usize + 1, '.');
            return mantissa_str

        } else{
            let zeros_after_decimal = (-self.exponent - 1) as usize;
            let padding : String = vec![0; zeros_after_decimal]
                .into_iter()
                .map(|i| i.to_string())
                .collect();

            let mut start_of_decimal = String::from("0.");
            start_of_decimal.push_str(&padding);
            start_of_decimal.push_str(&mantissa_str);

            return start_of_decimal
        }
    }
}

fn binary_to_decimal(binary_string: &str) -> Result<String, ()> {
    // Split the binary string into integer and fractional parts
    let parts: Vec<&str> = binary_string.split('.').collect();
    let integer_part = parts[0];
    let fractional_part = parts[1];

    // Convert the integer part from binary to decimal
    let integer_decimal = match dashu::integer::UBig::from_str_radix(integer_part, 2){
        Ok(integer) => integer,
        Err(_) => return Err(())
    };

    let fractional_decimal = fractional_part
        .chars()
        .enumerate()
        .fold(dashu::rational::RBig::ZERO, |acc, (i, c)| {
            let digit = c.to_digit(2).unwrap() as usize;
            

            let numerator   = dashu::integer::IBig::from_usize(digit).unwrap();
            let power_of_two= dashu::integer::UBig::from_usize(2).unwrap().pow(i + 1);

            acc + dashu::rational::RBig::from_parts(numerator, power_of_two)
        });

    // Combine the integer and fractional parts and format the result
    let decimal_result = integer_decimal + fractional_decimal;
    Ok(decimal_result.to_string())
}


impl<MT> std::fmt::Display for UnsignedFloat<MT>
where MT: MantissaBackend,
{       
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut mantissa_str = format!("{:b}", self.mantissa);

        if self.exponent >= 0{
            // Convert the mantissa to a binary string representation
            let extra_zeros_for_shifting: String = vec![0; self.exponent as usize]
                .into_iter()
                .map(|i| i.to_string())
                .collect();

            mantissa_str.push_str(&extra_zeros_for_shifting);
            mantissa_str.insert(self.exponent as usize + 1, '.');
            match binary_to_decimal(&mantissa_str){
                Ok(converted) => {
                    return write!(f, "{}", converted)
                },
                Err(()) => return Err(std::fmt::Error)
            };

        } else{
            let mantissa_str = format!("{:b}", self.mantissa);


            let zeros_after_decimal = -(self.exponent + 1) as usize;
            let padding : String = vec![0; zeros_after_decimal]
                .into_iter()
                .map(|i| i.to_string())
                .collect();

            let mut start_of_decimal = String::from("0.");
            start_of_decimal.push_str(&padding);
            start_of_decimal.push_str(&mantissa_str);

            match binary_to_decimal(&start_of_decimal){
                Ok(converted) => {
                    return write!(f, "{}", converted)
                },
                Err(()) => return Err(std::fmt::Error)
            };
        }
        

    }
}






