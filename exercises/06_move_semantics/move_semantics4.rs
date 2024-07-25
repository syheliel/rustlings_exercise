enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let list = Cons(10,Box::new(Cons(20,Box::new(Cons(30, Box::new(Nil))))));    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y: &mut Vec<i32> = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
