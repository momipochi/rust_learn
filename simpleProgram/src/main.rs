fn main() {
    println!("Hello, world!");
    let s = String::from("alalalala");
    let is_pan = is_pandulum(&s);
    println!("Is pandulum: {is_pan}");

    println!("Is prime: {:?}", is_prime(&11));

    multiplication_table(&12);

    infinite_wave();
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
        0 => return false,
        1 | 2 => return true,
        _ => {
            let mut start: u32 = 3;
            loop {
                if start == *n {
                    return true;
                }
                if n % start == 0 {
                    return false;
                }
                start += 1;
            }
        }
    }
}

fn multiplication_table(n:&u32){
    for i in 1..*n{
        for j in 1..*n{
            print!("\t{:}",i*j)
        }
        println!()
    }
}

fn infinite_wave(){
    let mut n:f64 = 0.0;
    loop {
        let j = ((&n.sin()).to_radians()*1000.0_f64).round() as u32;
        // print!("{j}");
        for i in 0..j{
            print!("*")
        }
        println!();
        n+=0.1;
    }
}