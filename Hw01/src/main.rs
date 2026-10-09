mod functions;
use std::io;
fn main() {

let mut player_input:i32 = functions::read_input(4,1); 
if player_input == 0{
println!("return was true"); 
}else{
println!( "input was {}", player_input);
}
player_input = functions::read_input(4,0);
functions::print_menu(player_input);
}
