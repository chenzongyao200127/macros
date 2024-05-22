use macros::EnumFrom;

#[allow(unused)]
#[derive(EnumFrom, Debug)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right(u32, u32),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    println!("{:?}", up);
    let left: Direction = Direction::Left(42);
    println!("{:?}", left);
}
