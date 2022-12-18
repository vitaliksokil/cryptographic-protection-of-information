#![allow(dead_code)]
#![allow(unused)]

fn main() {
    println!("------------------------------------------");
    let gcdex_args = (612, 342);
    println!("GCDEX args: {:?} \nGCDEX Results: {:?}", gcdex_args, gcdex(gcdex_args.0, gcdex_args.1));
    println!("------------------------------------------");
    println!("------------------------------------------");
    let inverse_element_args = (5, 18);
    println!("Inverse Element args: {:?} \nInverse Element Results: {:?}", inverse_element_args, inverse_element(inverse_element_args.0, inverse_element_args.1));
    println!("------------------------------------------");
    println!("------------------------------------------");
    let m = 20;
    println!("Euler args: {:?} \nEuler Results: {:?}", m, phi(m));
    println!("------------------------------------------");
    println!("------------------------------------------");
    let inverse_element2_args = (3, 34);
    println!("Inverse Element 2 args: {:?} \nInverse Element 2 Results: {:?}", inverse_element2_args, inverse_element_2(inverse_element2_args.0, inverse_element2_args.1));
    println!("------------------------------------------");
}

fn gcdex(a: i32, b: i32) -> (i32, i32, i32)
{
    let (mut a0, mut a1): (i32, i32) = (a, b);
    let (mut x0, mut x1): (i32, i32) = (1, 0);
    let (mut y0, mut y1): (i32, i32) = (0, 1);

    let mut q: i32 = 0;

    while a1 != 0 {
        q = a0 / a1;
        (a0, a1) = (a1, a0 - a1 * q);
        (x0, x1) = (x1, x0 - x1 * q);
        (y0, y1) = (y1, y0 - y1 * q);
    }

    return (a0, x0, y0);
}

fn inverse_element(a: i32, n: i32) -> i32
{
    let (d, x, y): (i32, i32, i32) = gcdex(a, n);
    println!("Gcdex result: {:?}", (d, x, y));
    if d == 1 {
        // x.rem_euclid(n) => analog in python is x % n
        return x.rem_euclid(n);
    }

    0
}

fn phi(n: i32) -> i32
{
    let mut n = n;

    let mut r = n;

    let mut i = 2;
    while i * i <= n {
        // n.rem_euclid(i) => analog in python is n % i
        if n.rem_euclid(i) == 0 {
            // n.rem_euclid(i) => analog in python is n % i
            while n.rem_euclid(i) == 0 {
                n = n / i;
            }
            r = r - (r / i);
        } else {
            i = i + 1;
        }
    }
    if n > 1 {
        r = r - r / n;
    }

    r
}

fn inverse_element_2(a: i32, p: i32) -> i32
{
    let phi = phi(p) - 1;
    a.pow(phi as u32).rem_euclid(p)
}