fn main() {
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
