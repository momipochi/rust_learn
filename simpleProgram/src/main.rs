use std::collections::HashSet;
use std::{collections::HashMap, fmt::format};

use regex::Regex;
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
    t6();
}

// ある整数の 2 乗に等しい整数のことを「平方数」といいます。例えば、

// 1112 = 12321

// なので、12321 は平方数です。

// 615432987 以下の最大の平方数を求めてください。
fn t1() {
    let mut max = 0;
    for i in 1..615432988 {
        let tmp = i * i;
        if tmp > max && tmp < 615432987 {
            max = tmp;
        }
        if tmp > 615432987 {
            break;
        }
    }
    println!("Answer: {max}");
}

// 1027026000の正の約数のうち、10000 以上 99999 以下の範囲にあるものを全て足し合わせたときの和を求めてください。
fn t2() {
    let mut sum = 0;
    for i in 10000..100000 {
        if 1027026000 % i == 0 {
            sum += i;
        }
    }
    println!("Answer: {sum}");
}

// 次の条件の何れか 1 つ以上を満たす整数のことを「三的な数」と呼ぶことにします。

// 3 の倍数である。
// 十進法で表すと 3 桁である。
// 十進法で表すと「3」が含まれる。
// 例えば、9、31、42、100、135 は何れも「三的な数」ですが、14 は「三的な数」ではありません。

// 1 から 3000 までの整数のうち「三的な数」だけを足し合わせたときの合計を求めてください。
fn t3() {
    let mut sum = 0;
    for n in 1..3001 {
        if is_factor_of_3(n) {
            sum += n;
            continue;
        }
        if contains_three_digits(n) {
            sum += n;
            continue;
        }
        if contains_three(n) {
            sum += n;
            continue;
        }
    }
    println!("Answer: {sum}");
}

// 以下に示すのは円周率の小数部分の最初の1000桁の数字列です
// 1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989
// この中の「隣り合う2桁の数字の部分列」（全部で999個あります）を取り出します。

// 14, 41, 15, 59, 92, 26, ⋯, 19, 98, 89
// この列の中に、「03」は9回出現します。では、最も多く出現する2桁数字は何であるかを、求めてください。

// ※最も多く出現する2桁数字は1種類しかありません。

//ans-1
fn t4() {
    let mut str = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989";

    let mut my_map: HashMap<&str, i32> = HashMap::new();
    loop {
        if str.len() < 2 {
            break;
        }
        let tmp = &str[..2];
        // println!("Len: {}", tmp);
        *my_map.entry(tmp).or_insert(-1) += 1;

        str = &str[1..];
    }
    println!("Answer: {:?}", my_map.iter().max_by(|a, b| a.1.cmp(&b.1)))
}

// 整数に対して、「一桁ことにばらして各桁の数字の積を求める」という操作を考えます。

// 123 → 1 × 2 × 3 = 6
// 666 → 6 × 6 × 6 = 216
// 1024 → 1 × 0 × 2 × 4 = 0
// 任意の整数について、「操作」を繰り返し適用すると、最終的には一桁の数に到達します。

// 77 → 49 → 36 → 18 → 8
// 123456 → 720 → 0
// 一桁の数に到達するのに必要な「操作」の回数は、77では4回、123456では2回となります。

// それでは、1000000以下の整数のうち、一桁の数に到達するのに必要な「操作」の回数が7回となるものはいくつあるかを、求めてください。

fn t5() {
    let mut count = 0;
    for i in 1..1000001 {
        let mut tmp = number_product(i);
        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);
        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);
        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);
        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);
        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);
        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);

        if tmp < 10 {
            count += 1;
            continue;
        }
    }
    println!("Answer: {count}");
}

#[derive(Debug)]
struct Comp {
    num: u32,
    div: u32,
}
// 2から1000000までの整数を以下の規則に従ってソート（整列）をします。

// 「1以外の最小の約数」が大きい順に並べる。例えば、143（約数は11）は147（約数は3）よりも前になる。
// 「1以外の最小の約数」が同じ場合は、その数自身が大きい順に並べる。例えば、119（約数は7）は91（約数は7）よりも前になる。
// この規則でソートした場合、先頭から230001番目にある整数（先頭は“1番目”と数えます）は何かを求めてください。
fn t6() {
    let mut v = vec![];
    for i in 2..1000001 {
        let mut div = i;
        for j in 2..i {
            if i % j == 0 {
                div = j;
                v.push(Comp { num: i, div });
                break;
            }
        }
    }
    v.sort_unstable_by_key(|e| (e.num, e.div));
    println!("Answer: {:?}", &v[230001])
}
// 以下の式の値を計算して、答えの小数部を切り捨てた整数値を答えてください。

// 1×2÷3+4×5÷6+7×8÷9+⋯+29998×29999÷30000

// ※式の中にある割り算は実数の割り算です。例えば

// 1×2÷3+4×5÷6+7×8÷9 であれば式の値は

// 0.66666 ⋯ + 3.33333 ⋯ + 6.22222 ⋯ = 10.22222 ⋯

// となります。
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
        if i % tmp == 0 && i != tmp {
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

fn c5() {
    let mut count = 0;
    for a in 1..6000 {
        for b in 1..6000 {
            let c = a * a + b * b;
            if area_under_6000(a, b) {
                count += 1;
            }
        }
    }
    println!("Answer: {count}");
}

fn area_under_6000(a: i32, b: i32) -> bool {
    return (a * b) / b < 6000;
}

// A と B をそれぞれ 1 以上 3 以下の整数として、A ÷ B の商と余りを求めます。

// 1÷1=1あまり0

// 1÷2=0あまり1

// 1÷3=0あまり1

// 2÷1=2あまり0

// 2÷2=1あまり0

// 2÷3=0あまり2

// 3÷1=3あまり0

// 3÷2=1あまり1

// 3÷3=1あまり0

// この 9 個の式の余りを全て足すと 5 になります。

// それでは、A と B をそれぞれ 1 以上 512 以下の整数としたときに、A ÷ B の余りを全て足した値を求めてください。

fn c6() {
    let mut sum = 0;
    for i in 1..2001 {
        for j in 1..2001 {
            sum += i % j;
        }
    }
    println!("Answer: {sum}");
}

// 以下の式で表される級数は円周率 π に収束することが知られています。つまり、式に現れる分数を次々と足していくと、その和は限りなく π に近づいていきます。

// 8
// 1 × 3
//   +
// 8
// 5 × 7
//   +
// 8
// 9 × 11
//   +
// 8
// 13 × 15
//   + ⋯
// しかし収束は非常に遅く、例えば、最初の 100 個の分数を足しただけでは、まだ 3.14 にも達しません。

// 8
// 1 × 3
//   +
// 8
// 5 × 7
//   + ⋯
// 8
// 397 × 399
//   = 3.13659...
// 最初の N 個の分数の総和が3.1414を超えるような最小の N を求めてください。

fn c7() {
    let mut count = 0;
    let mut sum = 0.0;
    let mut rate = 1.0;
    loop {
        if sum > 3.1414 {
            println!("Answer: {count}");
            break;
        }
        sum += 8.0 / (rate * (rate + 2.0));
        // println!("Check: {sum}");
        rate += 4.0;
        count += 1;
    }
}

// 次の条件の何れか 1 つ以上を満たす整数のことを「三的な数」と呼ぶことにします。

// 3 の倍数である。
// 十進法で表すと 3 桁である。
// 十進法で表すと「3」が含まれる。
// 例えば、9、31、42、100、135 は何れも「三的な数」ですが、14 は「三的な数」ではありません。

// 1 から 3000 までの整数のうち「三的な数」だけを足し合わせたときの合計を求めてください。

fn c8() {
    let mut sum = 0;
    for n in 1..3001 {
        if is_factor_of_3(n) {
            sum += n;
            continue;
        }
        if contains_three_digits(n) {
            sum += n;
            continue;
        }
        if contains_three(n) {
            sum += n;
            continue;
        }
    }
    println!("Answer: {sum}");
}

fn is_factor_of_3(n: i32) -> bool {
    return n % 3 == 0;
}
fn contains_three_digits(n: i32) -> bool {
    let reg = Regex::new(r"^\d{3}$");
    let str = format!("{}", n);
    match reg {
        Ok(i) => i.is_match(&str[..]),
        Err(_i) => false,
    }
}
fn contains_three(n: i32) -> bool {
    let reg = Regex::new(r".*3.*");
    let str = format!("{}", n);
    match reg {
        Ok(i) => i.is_match(&str[..]),
        Err(_i) => false,
    }
}

// 以下に示すのは円周率の小数部分の最初の1000桁の数字列です。
// 1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989

// この中の「隣り合う2桁の数字の部分列」（全部で999個あります）を取り出します。

// 14, 41, 15, 59, 92, 26, ⋯, 19, 98, 89
// この列の中に、「03」は9回出現します。では、最も多く出現する2桁数字は何であるかを、求めてください。

// ※最も多く出現する2桁数字は1種類しかありません。

fn c9() {
    let mut str = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989";

    let mut my_map: HashMap<&str, u32> = HashMap::new();
    loop {
        if str.len() < 2 {
            break;
        }
        let tmp = &str[..2];
        // println!("Len: {}", tmp);
        *my_map.entry(tmp).or_insert(0) += 1;

        str = &str[1..];
    }
    println!("Answer: {:?}", my_map.iter().max_by(|a, b| a.1.cmp(&b.1)))
}

// 以下のような規則で整数を並べます。

// 最初の3つの数は「1, 0, 5」である。
// それ以降は、直前の整数と 3 つ前の整数の和を並べる。つまり、
// 4番目の整数は 1 + 5 = 6
// 5番目の整数は 0 + 6 = 6
// 6番目の整数は 5 + 6 = 11
// この規則により、次のような列ができます。

// 1, 0, 5, 6, 6, 11, 17, 23, 34 ⋯

// この列の48番目の整数を求めてください。
fn c10() {
    let mut arr: [i32; 48] = [0; 48];
    arr[0] = 1;
    arr[1] = 0;
    arr[2] = 5;
    for i in 3..arr.len() {
        arr[i] = &arr[i - 3] + &arr[i - 1];
    }
    println!("Answer: {}", arr[arr.len() - 1]);
}

// 1027026000の正の約数のうち、10000 以上 99999 以下の範囲にあるものを全て足し合わせたときの和を求めてください。

fn c11() {
    let mut sum = 0;
    for i in 10000..100000 {
        if 1027026000 % i == 0 {
            sum += i;
        }
    }
    println!("Answer: {sum}");
}

fn c12() {
    let arr: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut str = format!("\n{}", arr[0]);
    for i in 1..arr.len() {
        for j in 0..i + 1 {
            if str.len() % 17 == 0 {
                str.push_str("\n");
            }
            let tmp = format!("{}", &arr[i]);
            str.push_str(tmp.as_str());
        }
    }
    println!("{}", str);
}

// 整数に対して、「一桁ことにばらして各桁の数字の積を求める」という操作を考えます。

// 123 → 1 × 2 × 3 = 6
// 666 → 6 × 6 × 6 = 216
// 1024 → 1 × 0 × 2 × 4 = 0
// 任意の整数について、「操作」を繰り返し適用すると、最終的には一桁の数に到達します。

// 77 → 49 → 36 → 18 → 8
// 123456 → 720 → 0
// 一桁の数に到達するのに必要な「操作」の回数は、77では4回、123456では2回となります。

// それでは、1000000以下の整数のうち、一桁の数に到達するのに必要な「操作」の回数が3回となるものはいくつあるかを、求めてください。

fn c13() {
    for i in 1..1000001 {
        let mut tmp = number_product(i);

        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);

        if tmp < 10 {
            continue;
        }
        tmp = number_product(tmp);

        if tmp < 10 {
            println!("Answer: {i}");
            break;
        }
    }
}

// 巨大な紙の上に、以下の規則に従って文字を書いていきます。

// 1 から 10000000 までの整数を順に書いていく。
// ただし整数が 3 で割り切れる場合は “Fizz”、5 で割り切れる場合は “Buzz”、3 と 5 の両方で割り切れ る場合は “FizzBuzz” を(数を書く代わりに)書く。
// 先頭からの30文字は以下のようになり、この中に数字 “1” は3回出現します。

// 12Fizz4BuzzFizz78FizzBuzz11Fizz

// それでは、最後まで書き切ったときに、完成した文字列の中に数字 “1” は何回出現するかを求めてください

fn c15() {
    let mut str = String::from("");
    for i in 1..10000001 {
        if i % 3 == 0 && i % 5 == 0 {
            str.push_str("FizzBuzz");
            continue;
        }
        if i % 3 == 0 {
            str.push_str("Fizz");
            continue;
        }
        if i % 5 == 0 {
            str.push_str("Buzz");
            continue;
        }
        let tmp = format!("{}", i);
        str.push_str(&tmp);
    }
    let mut count = 0;
    for c in str.chars() {
        if c == '1' {
            count += 1;
        }
    }
    println!("Answer: {count}");
}

fn c16() {
    let mut books = HashSet::new();
    for i in 1..31 {
        for j in 1..41 {
            for k in 1..11 {
                let sum = i * 205 + j * 82 + k * 30;
                books.insert(sum);
            }
        }
    }
    println!("Answer: {}", books.len())
}

fn number_product(n: u32) -> u32 {
    if n < 10 {
        return 1;
    }
    let mut sum = 1;
    let mut num = n;
    while num >= 10 {
        let tmp = num % 10;

        sum *= tmp;
        num /= 10;

        if num < 10 {
            sum *= num;
            break;
        }
    }

    return sum;
}

// 1から101までの奇数の3乗の和を求めてください。

// 13 + 33 + 53 + ⋯ + 1013

fn c14() {
    let mut sum: i128 = 0;
    for i in (1..102).step_by(2) {
        sum += i * i * i;
    }
    println!("Answer: {sum}");
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
