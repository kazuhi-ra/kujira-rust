#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn add(&mut self, pt: Self) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn main() {
    let mut p1 = Point::new(3, 8);
    let p2 = Point::new(-2, 4);

    p1.add(p2);

    println!("{:?}", p1)
}
