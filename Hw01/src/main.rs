
use std::io;
fn main() {
let mut player_input:i32 = 0 ;
let mut back1: i32 = 0;   
let mut input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
let input: char = input.trim().parse().expect("not a number");
Hw01::read_input(6,1,input); 
if back1 == 1{
println!("back1 was {}", back1); 
}
println!( "input was {}", player_input);



}
