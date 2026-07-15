fn main() {
    my_loop();
}

fn my_loop(){
    println!("Loop");
    for num in [1,2,3].iter(){
        println!("{}", num);
    }

    println!("Range");
    for num in 0..=10{
        println!("{}", num);
    }
}