// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
/*
    std 标准库 

    mut 是可以变
*/

// fn main() {
//     let stdout = stdout();
//     let message = String::from("hellow fellow Rustceans!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(&message, width, &mut writer).unwrap();

//     println!("Hello, world!");
// }


fn main (){
    println!("Guess the number");
    // let secret_number =rand::thread_rng().gen_range(1..=100);
    // loop {

    //     println!("Please input you guess");
    
    //     // let 默认不可变的  加了mut是可变的, string::new() 返回 utf-8编码内容
    //     let mut guess  = String::new();
    //     io::stdin().read_line(&mut guess).expect("failed to line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     /*
    //         这里创建了一个叫做 guess 的变量。不过等等，不是已经有了一个叫做 guess 的变量了吗？
    //         确实如此，不过 Rust 允许用一个新值来 隐藏 （Shadowing） guess 之前的值。
    //         这个功能常用在需要转换值类型之类的场景。它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量，诸如 guess_str 和 guess 之类
    //     */
    //     match guess.cmp(&secret_number){
    //         Ordering::Less => println!("Too small"),
    //         Ordering::Greater => println!("Too big"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    //     println!("you guess: {guess}")
    // }
    /*
        & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
        引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。完成当前程序并不需要了解如此多细节。
        现在，我们只需知道它像变量一样，默认是不可变的。因此，需要写成 &mut guess 来使其可变，而不是 &guess。
    */

    /*
    
        数字相加
        u32 后缀来表明字面量是一个 32 位无符号整数 即只包含整数
        i32 后缀表明字面量是一个 32 位有符号整数 有符号整数可以表示正数、负数和零，而32位有符号整数的范围通常是从 -2^31 到 2^31-1
    */ 
    // 整数想加

    println!("1 + 2 = {}",1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}",1i32  - 2);

    let pair = (32,true);
    println!("the reversed pair is {:?}", reverse(pair));

}

//元组可以充当函数的参数和返回值
fn reverse(pair:(i32,bool)) ->(bool,i32) {
    // 元组以 （） 表示
    let (integer,boolean) = pair;
    (boolean,integer)
}
