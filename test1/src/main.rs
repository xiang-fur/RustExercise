//Rust练习：n阶斐波那契数列生成器

use std::io;
fn main() {
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    let mut n = String::new();
    let mut t: i64 = 1;

    println!("这是n阶斐波那契数列生成器，请输入n：");
    io::stdin().read_line(&mut n).expect("您在干嘛？");
    let n: i64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("您输入的n为{}",n);
    loop {
        if t > n {
            break;
        }
        println!("{}", b);
        let c:i64 = b;
        b = a + b;
        a = c;
        t = t + 1;
    }
}
