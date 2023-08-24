struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_tuple (&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

fn main() {
    let p = Point { x: 20, y: 22,};
    let tuple = p.get_tuple();
    println!("{}, {}", tuple.0, tuple.1);
}
