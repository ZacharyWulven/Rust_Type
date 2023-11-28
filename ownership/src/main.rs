use std::os::macos::fs::MetadataExt;

fn test1() {

    let x1 = 42;
    let y1 = Box::new(88);

    {
        /*
            x1 实现了 Copy，这里只是发生了 Copy，所以下边还可以把 x1 赋值给 x2
            y1 则发生了移动

            删除的顺序
            第一条：y1 先声明，z 后声明，删除的时候先删除 z 后删除 y1
            第二条：先删除 x1 再删除 y1
         */
        let z = (x1, y1);
    }

    let x2 = x1;

    // 因为上边作用域 y1 移动给了 z，所以这里不能再使用 y1，所以报错
    // let y2 = y1;

}

/*
    input 与 output 指向不同的内存，
    因为 output 是可变的，它是独占的
    函数里，修改 output 不会影响 input

    但如果可变引用不是独占的，那么 input 与 output 就可能指向相同的内存，
    这时函数里对 output 的修改可能影响后边的代码逻辑
*/
fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}

fn test2() {
    let x = 42;
    // 1 可以让 y 指向其他的值，但不可以通过 y 来修改 x 的值
    let mut y = &x;   
    // 因为 z 没有 mut 修饰，所以它只能持有对 y 的可变引用，但可以通过 z 来修改 y 的值
    let z = &mut y;

    // *y = 10;  // Error

    let n = 20;
    *z = &n; // 修改 y 的值，y 指向 &n
}

fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay，as *s would be empty:

    /*
        这句不能执行
        如果这句可以那么值就移动了，但调用者依然认为他们拥有其所有权，
        所以他们还是会对这个值进行丢弃操作
     */
    // let was = *s;

    /*
        take 函数将 s 移动，并将 s 类型的默认值放到了原来的位置上了，
        这时调用者就会拥有那个新的默认的那个值，
        然后调用者再清理这个变量就不会有问题了
     */
    let was = std::mem::take(s);

    println!("was init is {}", was); // was init is 42
    println!("was init s is {}", s); // was init s is 0

    /*
        was 即其他一个合理的值，
        相当于又把原来 s 的值赋值回去了，又变成 42 了
     */
    *s = was;
    println!("*s = was is {}", s);   // *s = was is 42

    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

fn main() {
    let mut s = Box::new(42);
    replace_with_84(&mut s);
}
