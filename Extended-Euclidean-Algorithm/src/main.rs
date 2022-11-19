use std::collections::VecDeque;
fn main() {
    let mut modulus = String::new();
    let mut num = String::new();
    println!("Enter the modulus:");
    let _modulus_read = std::io::stdin().read_line(&mut modulus).unwrap();
    println!("Enter the Number:");
    let _num_read = std::io::stdin().read_line(&mut num).unwrap();
    let modulus = modulus.trim().parse::<i32>().unwrap();
    let num = num.trim().parse::<i32>().unwrap();

    let mut p_arr = VecDeque::from([0, 1]);
    let mut a_arr = VecDeque::<i32>::new();
    let mut b = 0;
    let mut n = modulus;
    let mut x = num;

    let mut index = 0;

    let mut no_inverse = false;
    loop {
        let mut a_i = 0;
        if (n > 0) {
            a_i = (n / x);
        } else {
            a_i = (n / x) - 1;
        };
        b = ((n % x) + x) % x;
        a_arr.push_back(a_i);
        // println!("{n} = {a_i}*{x} + {b}");

        if b == 0 {
            if x != 1 {
                println!("No multiplicative Inverse!");
                no_inverse = true;
                break;
            } else {
                break;
            }
        } else {
            n = x;
            x = b;
        }
        index += 1;
    }

    let mut p = 0;
    while !no_inverse {
        if (index >= 0) {
            p = p_arr.pop_front().unwrap();
            p = (((p - p_arr[0] * a_arr.pop_front().unwrap()) % modulus) + modulus) % modulus;
            p_arr.push_back(p);
            index -= 1;
        } else {
            println!("inverse is {}", p_arr[0]);
            break;
        }
    }
}
