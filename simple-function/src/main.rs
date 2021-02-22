use std::io::Write;
use std::str::FromStr;

/**
 * Find the greatest common divisor using Euklid's algorithm.
 */
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // assert!() checks are always executed and the program panics if they fail
    // debug_assert!() skips checks if the program is compiled in performant mode
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }

    // "return" keyword can be skipped if there is no semicolon - that
    // value will be returned
    n
}

/**
 * Tests can be run with `cargo test`
 *
 * #[test] marks code below for skipping during default compilation process.
 */
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
