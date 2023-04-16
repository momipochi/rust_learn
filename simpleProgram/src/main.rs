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
    println!("Hello, world!");
    let s = String::from("alalalala");
    let is_pan = is_pandulum(&s);
    println!("Is pandulum: {is_pan}");

    println!("Is prime: {:?}", is_prime(&11));

    multiplication_table(&12);

    // infinite_wave();

    my_mod::print_hello();

    outermodule::outer_module::say_hello();
}

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
