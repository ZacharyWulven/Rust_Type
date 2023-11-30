// use rand::prelude::*;
 
// fn main() {
//     let mut x = Box::new(42);
//     let r = &x;      // (1) 'a，生命周期开始

//     /*
//         可以看到 'a 生命周期并没有延伸到 if 中，
//         所以生命周期不一定与作用域重合
//      */
//     if random::<f32>() > 0.5 {
//         *x = 84;   // (2) 需要 x 的可变引用，检查到这是合法的
//         // println!("{}", r); // 如果这行在这就 Error，因为违反了借用规则
//     } else {
//         println!("{}", r); // (3) 'a，因为编译器指定 if else 只会走一个所以这里是 Ok 的
//     }

// }
// // (4)

// 生命周期的漏洞
// fn main() {
//     let mut x = Box::new(42);

//     let mut z = &x;  // (1) 'a，生命周期开始，这里在获得引用的时候开始
//     for i in 0..100 {           
//         println!("{}", z);      // (2) 'a
//         x = Box::new(1);        // (3)，其实到这第一个生命周期就结束了，因为 x 重新赋值了
//         /*
//             如果最后这句注释掉，上边就会报错，
//             x 一直处于被借用的状态，这时对它赋值就不行了
//             所以这个例子生命周期是由间歇性的失效
//             然后又有重启的操作，这就是所谓的 `漏洞`
//          */
//         // z = &x;                 // (4) 'a，即重启了新的生命周期，所以后续循环是 Ok 的
//     }
//     println!("{}", z);
// }

use std::path::Iter;

// struct StrSplit<'s, 'p> {
//     delimiiter: &'p str,   // 分隔符
//     document: &'s str,     // 文档
// }

// // 实现 Iterator trait 
// impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
//     type Item = &'s str;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// fn str_before(s: &str, c: char) -> Option<&str> {
//     StrSplit {
//         document: s,
//         delimiiter: &c.to_string(),
//     }
//     .next()
// }


// struct StrSplit<'s> {
//     delimiiter: &'s str,   // 分隔符
//     document: &'s str,     // 文档
// }

// // 实现 Iterator trait 
// impl<'s> Iterator for StrSplit<'s> {
//     type Item = &'s str;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// fn str_before(s: &str, c: char) -> Option<&str> {
//     StrSplit {
//         document: s,
//         delimiiter: &c.to_string(),
//     }
//     .next()
// }


// let x1: &'static str;       作用更大, 活的更长
// let x2: &'a      str;       作用小, 活的短

// fn take_func1(&'static str) 对参数要求比较严格，作用更小
// fn take_func2(&'s str)      对参数要求比较宽松，作用更大


// 多生命周期与 Variance

/*
    s 有两个生命周期
    'a 是可变的
    'b 不可变
*/
struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

fn main() {
    let mut r: &str = "hello";     // &'statiic str => &'a str
    /*
        MutStr { s: &mut r } 是创建实例
        .s 可以修改值，所以它是 'a mut 
        &mut r 传入后生命周期就结束了，下边才可以 println

        如果只有一个生命周期，就报错了，
        只有一个生命周期 到 &mut r 其实是缩短了，因为 &mut 是赋值时候发生的
        也就相当于对 r 缩短了，但是 &'statiic str 可替换为 &'a str
        但是 &mut 是精确类型，所以这里缩短借用就会失败
        所以使用一个生命周期就会 Error
     */
    *MutStr { s: &mut r }.s = "world";
    println!("{}", r);   // 是 'b 的生命周期
}