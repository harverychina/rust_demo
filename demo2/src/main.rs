fn main() {
    // 流程控制
    // 条件表达式
    // let number =7;
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // rust中if是Expression 可以有返回值
    // C中是statement
    // rust是静态语态，在程序运行之前必须确认每一个变量的类型，不符合要求或不确定编译器是无法运行的
    // let condition = true;
    // let number = if condition {5} else {"six"};
    // let number = if condition { 5 } else { 6 };
    // println!("{}", number);

    // 循环
    // loop 循环，类似死循环
    // loop {
    //     println!("Hello");
    // }
    // while循环
    // let mut number = 4;
    // while number != 0 {
    //     println!("Hello");
    //     number -= 1;
    //     if number == 2 {
    //         break;
    //     }
    // }
    // for 循环 推荐这种循环方式
    let a = [10, 20, 30, 40, 50];
    // for el in a.iter() {
    //     println!("the value is {}", el);
    // }

    for i in a.iter(){
        println!("值为：{}", i);
    } 
}
