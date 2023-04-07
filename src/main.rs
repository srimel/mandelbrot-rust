/*!
    The mandelbro set is defined as the set of complex numbers `c`
    for which `z` does not fly out to infinity. Complex numbers have
    real `re` and imaginary `im` components which we'll treat as a x y
    coordinates on the Cartesian plane. We want to color the point black
    if “c is in the Mandelbrot set, or a lighter color otherwise. So for each
    pixel in our image, we must run the preceding loop on the corresponding point
    on the complex plane. 
*/

use::num::Complex;
use::std::str::FromStr;

/// ### Returns
/// `Some(i)` on failure, otherwise `None`
/// 
/// ### Overview
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide. 
/// 
/// If `c` is not a member, return `Some(i)`, where `i` is the number of 
/// iterations it took for `c` to leave the circle of radius two centered
/// on the origin. If `c` seems to be a member (more precisely, if we reach
/// the iteration limit without being able to prove that `c` i not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 { // distance from origin
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn main() {

}