fn main() {
    println!("Hello, world!");

    let x = 42;
    let y = &mut x;
}

// impl String {
//     pub fn contains(&self, p: impl Pattern) -> bool {
//         p.is_contained_in(self)
//     }
// }


struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;

}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, num_1: &i32, num_2: &i32) -> bool {
        (&self.0 == num_1) && (&self.1 == num_2)
    }

    fn first(&self) -> i32 { self.0 }

    fn last(&self) -> i32 { self.1 }

}