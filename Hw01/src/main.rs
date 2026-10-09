mod functions;
fn main() {
// main variables
let mut gold_amount: u64 = 100;
let mut food_amount: u64 = 0;
let mut lumber_amount: u64 =0;
let mut herbs_amount: u64 = 0;
let mut days: u64= 0;
let mut can_purchase: bool = false;
let mut player_input: i32 = 0;

// initial code
player_input = functions::read_input(4,1); 
if player_input == 0{
println!("return was true"); 
}else{
println!("input was {}", player_input);
}
player_input = functions::read_input(999,0);
let mut dice_roll:u64 = functions::roll_dice(player_input as u64);
println!("dice roll results: {}", dice_roll); 
player_input*=5;
functions::print_menu(2,gold_amount);
can_purchase = functions::can_afford(player_input as u64, gold_amount);
if can_purchase == true{
gold_amount -= player_input as u64;
println!("you have the funds for this item, you now have {:.2}$ remaining", gold_amount);
functions::print_status(food_amount,gold_amount,lumber_amount,herbs_amount,days);
}
}

