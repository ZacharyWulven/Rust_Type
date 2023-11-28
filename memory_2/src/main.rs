// fn main() {

//     let mut n_nonzero = 0;

//     /*
//         扫描运行内存，从 0 开始扫描
//         当 i == 0 时，就是 null 指针，是非法的
//     */
//     for i in 0..10000 {
//         let ptr = i as *const u8;
//         let byte_at_addr = unsafe { *ptr };

//         if byte_at_addr != 0 {
//             n_nonzero += 1;
//         }
//     }

//     println!("内存中的非 0 字节：{}", n_nonzero); // 68355 segmentation fault  cargo run

// }

// 静态全局变量
static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 123456;
    &noop_local as *const i32
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL:        {:p}", &GLOBAL as *const i32);    // GLOBAL:        0x10df4b190
    println!("local_str:     {:p}", local_str as *const str);  // local_str:     0x10df4b194
    println!("local_int:     {:p}", &local_int as *const i32); // local_int:     0x7ff7b1ff3f8c
    println!("boxed_int:     {:p}", Box::into_raw(boxed_int)); // boxed_int:     0x7fefeef05b50
    println!("boxed_str:     {:p}", Box::into_raw(boxed_str)); // boxed_str:     0x7fefeef05b40
    println!("fn_int:        {:p}", fn_int);                   // fn_int:        0x7ff7b1ff3ebc
}