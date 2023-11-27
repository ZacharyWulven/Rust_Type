use std::mem::drop; // 可手动进行释放

fn main() {


    // 一共 4 个值：42、43、&x、&y
    // 一共 4 个变量：x、y、var1、var2
    // var1、var2 是指针类型，也叫引用，它们是两个变量，两个槽
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;

    // s 就是一个指针，它指向第一个字符的位置
    let s = "Hello World";

    // let c: i32;
    // println!("c is {}", c);


    let pw = "jackdss";
    let is_strong = is_strong(pw);


    let a: i32 = 40; // Stack
    let b: Box<i32> = Box::new(30); // Heap

    // Error，因为 b 在 heap 上只能通过指针来访问
    // let result = a + b; 

    let result = a + *b;

    println!("{} + {} = {}", a, b, result); // 40 + 30 = 70


    // 
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;
    
    drop(a);  // 手动释放 a

    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("result1={}, result2={}", result1, result2);


}

// &str -> Stack; String -> Heap
// 只能接收 String 类型，传入 &str 会报错
// fn is_strong(password: String) -> bool {
//     password.len() > 5
// }


// T 是实现了 AsRef<str>，即将它作为到 AsRef<str> 的引用
// fn is_strong<T: AsRef<str>>(password: T) -> bool {
//     password.as_ref().len() > 5
// }

// T 可以转化为 String 类型
fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}
