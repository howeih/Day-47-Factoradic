Day 47: Factoradic
<br>
Factorial number system is an interesting showcase of a mixed radix. Instead of powers of homogeneous base as we are usually used to, factoradic uses factorials.
<br>
For example,
<br>
6281 = 6*10³ + 2*10² + 8*10¹ + 10⁰
11412210 = 7! + 6! + 4*5! + 4! + 2*3! + 2*2! + 1! + 0\*0!
<br>
What’s interesting about representations of a number in different [homogeneous or mixed] radices? If a numeral representation in radix R shows a high entropy, it will keep high entropy in all the representations. This is very powerful idea that brings notion of what information is. The information contained in the number can be transferred [to radix system] or hidden, but cannot vanish.
<br>
run:

```
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
```

result:

```
463 = 0*0! + 1*1! + 0*2! + 1*3! + 4*4! + 3*5!
```
