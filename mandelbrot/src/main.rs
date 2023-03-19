use num::Complex;
use std::str::FromStr;


fn main() {
    let c = Complex{ re:4.0, im:3.0};
    let limit = 255;
    println!("escape_time({}, {}) = {:?}", //https://doc.rust-lang.org/rust-by-example/std/option.html
        c, limit, escape_time(c, limit)
    );

    let sep = ',';

    let input_str = format!("{}{}{}", c.re, sep, c.im);

    // TODO : Why :: before <f64>?
    println!(r#"parse_pair("{}", '{}') = {:?}"#,
        input_str, sep, parse_pair::<f64>(&input_str, 'x')
    );

    println!(r#"parse_complex("{}", '{}') = {:?}"#,
        input_str, sep, parse_complex(&input_str)
    );
}


// Mandelbrot set escape time
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i); // TODO : What does Some mean?
        }
        z = z * z + c;
    }
    None
}


// TODO : has to be used in main()?
// TODO : what does match do? How to use it?
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match(T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}


fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex{re, im}),
        None => None
    }
}


#[test] // TODO : role of #[test] directive?
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",            ','), None);
    assert_eq!(parse_pair::<i32>("10,",         ','), None);
    assert_eq!(parse_pair::<i32>(",10",         ','), None);
    assert_eq!(parse_pair::<i32>("10,20",       ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy",     ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",        'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",     'x'), Some((0.5, 1.5)));
}


#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {re:1.25, im:-0.0625})
    );
    assert_eq!(
        parse_complex(",-0.0625"), None);
}
