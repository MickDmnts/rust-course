struct Redfox{
    is_enemy: bool,
    life: u8,
}

impl Redfox{
    fn new()-> Self{
        Self {
            is_enemy: true,
            life: 70,
        }
    }
}

fn main(){
    let fox: Redfox = Redfox::new();
    let life_left: u8 = fox.life;
}