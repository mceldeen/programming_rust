#![feature(advanced_slice_patterns, slice_patterns)]

extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use std::str::FromStr;
use router::Router;
use urlencoded::UrlEncodedBody;


fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");


    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_: &mut Request) -> IronResult<Response> {
    Ok(Response::new()
        .set(status::Ok)
        .set(mime!(Text/Html; Charset=Utf8))
        .set(r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>GCD Calculator</title>
</head>
<body>
  <form action="/gcd" method="post">
    <input type="text" name="n" />
    <input type="text" name="n" />
    <button type="submit">Compute GCD</button>
  </form>
</body>
</html>
    "#))
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    Ok(
        request.get::<UrlEncodedBody>()
            .map_err(|e| format!("Error parsing form data: {:?}\n", e))
            .and_then(|mut hashmap| {
                hashmap.remove("n")
                    .ok_or(format!("form data has no 'n' parameter\n"))
                    .map(|num_strs| {
                        num_strs
                            .into_iter()
                            .filter(|num_str| num_str.len() > 0)
                    })
            })
            .and_then(parse_numbers)
            .and_then(|nums| {
                compute_gcd(nums.as_slice())
                    .ok_or(format!("Could not compute GCD for empty list"))
                    .map(|gcd| {
                        Response::new()
                            .set(status::Ok)
                            .set(mime!(Text/Plain; Charset=Utf8))
                            .set(format!("The greatest common divisor of the numbers {:?} is {}\n"
                                         , nums, gcd))
                    })
            })
            .unwrap_or_else(|e| {
                Response::new()
                    .set(status::BadRequest)
                    .set(mime!(Text/Plain; Charset=Utf8))
                    .set(format!("Error: {}", e))
            })
    )
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

