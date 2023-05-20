mod mpfr_glue;
mod u_layer;
mod g_layer;
mod h_layer;

use u_layer::ubounds::*;
use h_layer::from_f64::*;

fn main() {
    let x : UBound<u16,u64> = (120000000345.521).into();

    println!("{:?}", x.left.unwrap());
    println!("{}", x);

}

