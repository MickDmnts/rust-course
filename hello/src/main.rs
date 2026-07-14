use rand::{Rng, thread_rng};

fn main(){
    let x:i32 = thread_rng().gen_range(0, 10);
    print!("{}",x)
}