mod functions;
use std::io;
fn main() {
let mut gold_amount: u64 = 100;
let mut can_purchase: bool = true;
let mut player_input:i32 = functions::read_input(4,1); 
if player_input == 0{
println!("return was true"); 
}else{
println!( "input was {}", player_input);
}
player_input = functions::read_input(999,0);
functions::print_menu(2);
can_purchase = functions::can_afford(player_input as u64, gold_amount);
if can_purchase == true{
gold_amount -= player_input as u64;
println!("you have the funds for this item, you now have {:.2}$ remaining", gold_amount);
functions::print_status(gold_amount,2,3,5,3);
}
}
