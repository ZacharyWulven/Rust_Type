// 例子一
// static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
// static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

// fn main() {
//     let a = 42;
//     let b = &B;
//     let c = &C;
//     /*
//         {:p} 表示打印指针的地址
//      */
//     println!("a: {}, b: {:p}, c: {:p}", a, b, c); // a: 42, b: 0x10c7ea44c, c: 0x10c7ea456
// }

// 例子二
// use std::mem::size_of;
// static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
// static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

// fn main() {
//     let a: usize = 42;
//     let b: Box<[u8]> = Box::new(B); // B 里值的所有权就跟着 Box 走了
//     let c: &[u8; 11] = &C;

//     println!("a (unsigned 整数)：");
//     println!("  地址: {:p}", &a);
//     println!("  大小: {:?} bytes", size_of::<usize>());
//     println!("  值:   {:?}\n", a);

//     println!("b (B 装在 Box 里)：");
//     println!("  地址: {:p}", &b);
//     println!("  大小: {:?} bytes", size_of::<Box<[u8]>>());
//     println!("  指向: {:p}\n", b);

//     println!("c (C 的引用)：");
//     println!("  地址: {:p}", &c);
//     println!("  大小: {:?} bytes", size_of::<&[u8; 11]>());
//     println!("  指向: {:p}\n", c);


//     println!("B (10 bytes 的数组)：");
//     println!("  地址: {:p}", &B);
//     println!("  大小: {:?} bytes", size_of::<&[u8; 10]>());
//     println!("  值: {:?}\n", B);


//     println!("C (11 bytes 的数组)：");
//     println!("  地址: {:p}", &C);
//     println!("  大小: {:?} bytes", size_of::<&[u8; 11]>());
//     println!("  值: {:?}\n", C);
// }


// 例子三
// Cow 是一个智能指针，代表 copy on right, 
// 只有需要写入的时候才需要复制,平时读的时候就不需要复制了
// use std::borrow::Cow;
// CStr 类似 C 语言的字符串，允许读取以 0 结尾的字符串
// use std::ffi::CStr;
// c_char 就是 Rust 中的 i8 的别名
// use std::os::raw::c_char;

// static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
// static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];


// fn main() {
//     let a = 42;
//     let b: String;    // 智能指针
//     let c: Cow<str>;  // 智能指针

//     unsafe {
//         // 需要再 unsafe 块中执行
//         // *mut u8 一个可变的原始指针
//         let b_ptr = &B as *const u8 as *mut u8;  // 原始指针
//         b = String::from_raw_parts(b_ptr, 10, 10);

//         let c_ptr = &C as *const u8 as *const c_char;
//         c = CStr::from_ptr(c_ptr).to_string_lossy();
//     }
//     println!("a: {} b: {} c: {}", a, b, c);  // a: 42 b: carrytowel c: thanksfish
// }

// 例子四
// fn main() {
//     let a: i64 = 42;
//     // *const i64，即将其引用 &a 转为 *const i64
//     let a_ptr = &a as *const i64;

//     println!("a: {}, ({:p})", a, a_ptr); // a: 42, (0x7ff7b358c1b0)
// }


// 例子五
fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    
    let a_addr: usize = unsafe {
        // 使用 transmute 转为 usize 形式
        std::mem::transmute(a_ptr)
    };
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}
