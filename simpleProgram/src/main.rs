mod my_mod {
    pub fn print_hello() {
        println!("Module Says Hello!");
        print_hello_within_my_mod();
    }

    fn print_hello_within_my_mod() {
        println!("Private function says hello")
    }
}
mod outermodule;

fn main() {
    // println!("Hello, world!");
    // let s = String::from("alalalala");
    // let is_pan = is_pandulum(&s);
    // println!("Is pandulum: {is_pan}");

    // println!("Is prime: {:?}", is_prime(&11));

    // multiplication_table(&12);

    // // infinite_wave();

    // my_mod::print_hello();

    // outermodule::outer_module::say_hello();

    // controller_function();
    c4();
}

fn c1() {
    let mut tmp = 0.0;
    for i in (1..30001).step_by(3) {
        tmp += f64::from(i) * (f64::from(i) + 1.0) / (f64::from(i) + 2.0);
    }
    println!("Answer: {tmp}");
}

fn c2() {
    let mut s = 387420489;
    let p = 0.8;
    for i in 0..48 {
        s = (f64::from(s) * p) as i32;
    }
    println!("Answer: {s}")
}

fn c3() {
    let mut count = 0;
    for i in (1..4999996).step_by(5) {
        let a = i;
        let b = i + 1;
        let c = i + 2;
        let d = i + 3;
        let e = i + 4;
        let sum = a + b + c + d + e;
        let tmp = format!("{}{}{}{}{}", a, b, c, d, e);
        // println!("T: {tmp}");
        let t: i128 = tmp.trim().parse::<i128>().unwrap();
        if t % sum == 0 {
            count += 1;
        }
    }
    println!("Answer: {count}");
}

// An integer consisting of only one kind of digit (excluding 0), such as 77777, is called a 'monotone integer'.

// There is only one 'monotone integer' with less than 9 digits that is divisible by 1434. Find that integer.
fn c4() {
    let tmp = 1434;

    let mut fac = 1;
    let mut ratio = 10;
    for i in 1..100000000 {
        if i >= ratio {
            fac += ratio;
            ratio *= 10;
        }
        if i % fac != 0 {
            continue;
        }
        // println!("fac: {fac}");
        if i % tmp == 0 && i != tmp  {
            println!("Answer: {}", i);
            break;
        }
    }
}

// A triangle whose three sides have lengths 3, 4 and 5 is a right-angled triangle. This is shown by the Pythagorean Theorem.

// For a right-angled triangle, if the two sides across the right angle are of length a and b and the hypotenuse is of length c, then a2 + b2 = c2.

// In other words, 32 + 42 = 52 (= 25) holds, so it can be said to be a right-angled triangle.

// Find how many right-angled triangles have integer lengths on three sides and an area less than 6000.

// Congruent triangles (e.g. "3, 4, 5" and "5, 4, 3") are counted as one type without any distinction.


fn is_pandulum(s: &String) -> bool {
    let tmp: String = s.chars().rev().collect();

    match tmp.eq(s) {
        true => return true,
        _ => return false,
    }
}

fn is_prime(n: &u32) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _ => {
            for i in 3..*n {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

fn multiplication_table(n: &u32) {
    for i in 1..*n {
        for j in 1..*n {
            print!("\t{:}", i * j)
        }
        println!()
    }
}

fn infinite_wave() {
    let mut n: f64 = 0.0;
    loop {
        let j = ((&n.sin()).to_radians() * 1000.0_f64).round() as u32;
        // print!("{j}");
        for i in 0..j {
            print!("*")
        }
        println!();
        n += 0.1;
    }
}

fn controller_function() {
    let mut n = sub_funciton_one(2);
    let mut str = sub_funciton_two(String::from("What is up"));
    println!("What do you think of {n} if I {str}")
}

fn sub_funciton_one(n: u32) -> u32 {
    n * 2
}
fn sub_funciton_two(str: String) -> String {
    str.chars().rev().collect()
}
