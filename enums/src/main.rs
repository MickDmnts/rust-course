use std::{fs::File, io::Read};

#[must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let res = File::open("foo");
    match res {
        Ok(f) => {
            println!("OK");
        }
        Err(e) => {
            println!("ERROR");
        }
    }
}
