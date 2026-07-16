enum Color {
    Red,
    Green,
    Blue,
}

enum Custom<T> {
    Some(T),
    None,
}

fn main() {
    let color = Color::Red;
}
