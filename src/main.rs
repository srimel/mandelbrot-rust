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

/// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("-123.23,420.230"), Some(Complex{re:-123.23, im:420.230}));
    assert_eq!(parse_complex(",420.230"), None);
}


/// ### Returns
/// `Some<(x,y)>` on success, otherwise `None`
/// 
/// ### Overview
/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
/// 
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn main() {

}