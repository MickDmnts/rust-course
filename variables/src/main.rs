const STARTING_MISSILES:i32 = 8;
const READY_AMMOUNT:i32 = 2;

fn main() {
    //Part 1
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    
    //Part 2
    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    //Extra
    let (missiles, ready) : (i32,i32) = (STARTING_MISSILES, READY_AMMOUNT);
    println!("{} missiles left", (missiles - ready));

    let var = 10;
}
