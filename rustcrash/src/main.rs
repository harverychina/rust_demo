// const X:i32 = 42;  // 定义常量，标识符采用大写形式

fn main() {
    // let x = 1;  // 定义不可变量
    // let mut x = 1;
    // x = 42;  // 不可变量不能两次赋值，使用 mut关键字定义可变变量就可以重新赋值
    // print!("{}", X);

    // let x = 2.0;   // f64
    // let y: f32 = 3.0;   // f32

    // let bool_true = true;
    // let bool_false = false;

    // let char_ascii = 'a';
    // let char_emoji = '👊';

    // print!("{}{}", char_ascii, char_emoji);

    let array = [1, 2, 3, 4, 5];
    println!("{}", array[0]);
    println!("{}", array[1]);

    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");

    fn gcd(m: u64, n: u64) -> u64 {
        assert!(m != 0 && n != 0);
        if m > n {
            return gcd(m - n, n);
        };
        if m < n {
            return gcd(n - m, m);
        };
        m
    }

    println!("gcd(2, 1) = {}", gcd(2, 1));


    mod tests{
        // #[test]
        
        // fn exploratioin(){
        //     assert_eq!(2 + 2, 4);
        // }
        
        // // #[test]
        // fn another() {
        //     panic!("Make this test fail");
        // }
    }

}
