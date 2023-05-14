#include <stdio.h>
#include <math.h>
#include <gmp.h>
#include <mpfr.h>


/* 

                Understanding the floating points of MPFR 4.1.0

The mpfr_t data structure is defined in mpfr.h. It has four fields: 

    (mpfr.h lines 232 - 237)
    1. _mpfr_prec  => The number of significant digits in the whole mantissa, the implicit one included
    2. _mpfr_sign  => The sign
    3. _mpfr_exp   => The exponent
    4. *_mpfr_d    => A pointer to an array of limbs. Each limb is an integer
                      whose bit representation holds part of the whole mantissa bits

To read and eventually modify the mantissa, we must learn how to work with the array of limbs that come together to form the mantissa. 
The mantissa sits across multiple limbs like this:

Example:           1.010010110110010100011010101111000101001101100111
                   | |                                              
                   | |                              .------ More significant digits overflow to another limb to the left
                   | |                              v               
                   1.010010110110010100011010101111 000101001101100111 
                   | |                            | |                |
                   | |                            | |                |.---Trailing zeros after the example precision ends, to fill in the least bits left
                   | |                            | |                |v           v
Limb split shown   1.010010110110010100011010101111 0001010011011001110000000000000
                   |----------- limb 1 -----------| |----------- limb 2 -----------|
*/


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

// geeksforgeeks.org/binary-representation-of-a-given-number/#
void print_coefficient_binary(long unsigned fraction)
{
    /* step 1 */
    if (fraction > 1)
        print_coefficient_binary(fraction / 2);
 
    /* step 2 */
    printf("%lu", fraction % 2);
}

int main (void){
    printf ("MPFR library: %-12s\nMPFR header: %s (based on %d.%d.%d)\n\n",
            mpfr_get_version (), MPFR_VERSION_STRING, MPFR_VERSION_MAJOR,
            MPFR_VERSION_MINOR, MPFR_VERSION_PATCHLEVEL);


    mpfr_t x;

    mpfr_init2 (x, 129);
    mpfr_set_d (x, 2.0, MPFR_RNDD);

    mp_limb_t *mantissa =  x->_mpfr_d;

    printf(" ");
    for (long i = index_of_highest_limb(&x); i>=0; i--){
        unsigned long long limb = mantissa[i];
        // printf("Limb #%li: ",i);
        print_coefficient_binary(limb);
    }
    printf("\n");


    mpfr_out_str(stdout, 2, 0, x, MPFR_RNDD);

    printf("\nExponent as signed int: %i\n", x->_mpfr_exp);

    mpfr_clear(x);
    return 0;
}