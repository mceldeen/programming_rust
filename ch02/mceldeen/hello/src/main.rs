#![feature(advanced_slice_patterns, slice_patterns)]

use std::io::Write;
use std::str::FromStr;

fn main() {
    let args = std::env::args().skip(1);
    let usage = String::from("Usage: gcd NUMBER ...");
    let answer_res = parse_numbers(args).and_then(|numbers| {
        let num_slice = numbers.as_slice();
        compute_gcd(num_slice).map(|gcd| {
            format!("The greatest common divisor of {:?} is {}", num_slice, gcd)
        }).ok_or(usage)
    });

    match answer_res {
        Ok(answer) => {
            println!("{}", answer)
        },
        Err(err) => {
            writeln!(std::io::stderr(), "{}", err).unwrap();
            std::process::exit(1)
        },
    };
}

fn parse_numbers<I>(num_strs: I) -> Result<Vec<u64>, String>
    where I: Iterator<Item=String> + std::marker::Sized {
    num_strs.map(|num_str| {
        u64::from_str(&num_str).map_err(|_| format!("error parsing \"{}\"", num_str))
    }).collect()
}

#[test]
fn test_parse_numbers() {
    let args: Vec<String> = vec![
        String::from("dog"),
        String::from("1")
    ];

    assert_eq!(parse_numbers(args.clone().into_iter()), Err(String::from("error parsing \"dog\"")));

    assert_eq!(parse_numbers(args.clone().into_iter().skip(1)), Ok(vec![1]));
}

fn compute_gcd(numbers: &[u64]) -> Option<u64> {
    match numbers {
        &[] => None,
        &[x] => Some(gcd(x, x)),
        &[x, ref xs..] => Some(xs.iter().fold(x, |x, y| gcd(x, *y))),
    }
}

#[test]
fn test_compute_gcd() {
    let data: Vec<u64> = vec![4, 2, 6, 8];

    assert_eq!(compute_gcd(&data[..0]), None);

    assert_eq!(compute_gcd(&data[..1]), Some(4));

    assert_eq!(compute_gcd(data.as_slice()), Some(2));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2 * 5 * 11 * 17, 3 * 7 * 13 * 19), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

