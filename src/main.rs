use std::io;
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
    print!("Guess the number");
    println!("Please input you guess");
    // let 默认不可变的  加了mut是可变的, string::new() 返回 utf-8编码内容
    let mut guess  = String::new();
    io::stdin().read_line(&mut guess).expect("failed to line");
    println!("you guess: {guess}")
    /*
        & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
        引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。完成当前程序并不需要了解如此多细节。
        现在，我们只需知道它像变量一样，默认是不可变的。因此，需要写成 &mut guess 来使其可变，而不是 &guess。
    */
}
