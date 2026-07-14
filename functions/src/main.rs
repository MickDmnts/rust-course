// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width: i32 = 4;
    let height: i32 = 7;
    let depth: i32 = 10;

    let area = area_of(width, height);
    println!("Area is {}", area);

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 { x * y }

fn volume(width: i32, height: i32, depth:i32) -> i32 {
    width*height*depth
}