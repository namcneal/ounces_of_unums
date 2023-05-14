#include <stdio.h>
#include <gmp.h>
#include <mpfr.h>

long get_num_limbs(mpfr_t* number){
    // mpfr.h line 165
    // Worst case length-type for '_mpfr_prec_t' alias: signed long
    long precision = (long) (*number)->_mpfr_prec;

    // gmp.h line 46
    // GMP_NUMB_BITS has a type int, make it into an unsigned long to match the precision
    const long BITS_PER_LIMB = (long) GMP_NUMB_BITS;
   
    long num_limbs;
    if (precision <= BITS_PER_LIMB){
        num_limbs = 1;

    } else if (precision % BITS_PER_LIMB == 0){
        num_limbs = precision / BITS_PER_LIMB;

    } else{
        num_limbs = 1 + precision / BITS_PER_LIMB;
    }

    return num_limbs;
}

long index_of_highest_limb(mpfr_t* number){
    return get_num_limbs(number) - 1;
}

