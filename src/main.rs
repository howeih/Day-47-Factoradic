use std::i32;

fn div_mod(num: i32, divisor: i32) -> (i32, i32) {
    (num / divisor, num % divisor)
}

fn fac(i: i32) -> Vec<i32> {
    let mut factoradic = Vec::<i32>::new();
    let mut num = i;
    let mut divisor = 1i32;
    loop {
        let (quotient, remainder) = div_mod(num, divisor);
        num = quotient;
        divisor += 1;
        factoradic.push(remainder);
        if quotient == 0 {
            break;
        }
    }
    factoradic
}

fn main() {
    let factoriadic = fac(463);
    let mut n = 0;
    print!("463 =");
    for (i, v) in factoriadic.iter().enumerate() {
        print!(" {}*{}! ", v, n);
        n += 1;
        if i < factoriadic.len() - 1 {
            print!("+");
        }
    }
}
