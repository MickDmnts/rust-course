struct Redfox{
    is_enemy: bool,
    life: u32,
}

trait Noisy{
    fn get_noise(&self) -> &str;
}

impl Noisy for Redfox{
    fn get_noise(&self) -> &str {
        "Meow"
    }
}

fn print_noise<T: Noisy>(item: T){
    println!("{}", item.get_noise());
}

fn main(){ 
}