mod functions;
fn main() {
// main variables
let mut seed: u64;
let mut gold_amount: u64 = 100;
let mut gold_amount_mid: u64 =0;
let mut food_amount: u64 = 0;
let mut lumber_amount: u64 =0;
let mut herbs_amount: u64 = 0;
let mut days: u64= 0;
let mut days_mid: u64 = 0;
let mut can_purchase: bool = false;
let mut player_input: i32 = 0;
let mut cart: bool = false;
let mut axe_level: u64 =1;
let mut gun_level: u64 =1;
let mut knife_level: u64 =1;
let mut roll_amount: u64=0;
// start of main function

println!("Please input a seed between 1 and 999, input 0 to end session");
seed = functions::read_input(999,1) as u64;
if seed == 0{
return;
}
println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
println!("Welcome to Trail Traders");
println!("press 1 to start the game"); 
println!("press 0 to end session");
println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
player_input = functions::read_input(1,0);
if player_input == 0{
println!("session terminated");
return;
}
while player_input > 0{
functions::print_menu(1,gold_amount);
player_input = functions::read_input(4,1);
if player_input == 0{
println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
println!("Are you sure you want to quit?");
println!("Input 0 again to confirm");
println!("Input 1 to continue playing");
println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
player_input = functions::read_input(1,0);
	if player_input == 0{
	println!("session terminated");
	return;
	}
}
if player_input == 1{
}
if player_input == 2{
functions::print_menu(3,gold_amount);
player_input = functions::read_input(2,1);
	if player_input == 0{
	player_input = 1;
	}
	if player_input == 1{
	food_amount += functions::hunt(seed+roll_amount,gun_level,knife_level,cart);
	days+=1;
	}
	if player_input == 2{
	lumber_amount += functions::gather1(seed+roll_amount,axe_level,cart);
	herbs_amount += functions::gather2(seed+roll_amount,knife_level,cart);
	days+=2;
	}
println!("Press 0 to continue");
functions::read_input(0,0);
}
if player_input == 3{
days_mid = functions::roll_dice(seed+roll_amount);
roll_amount += 1;
gold_amount_mid = functions::roll_many_dice(seed+roll_amount,3)*2;
roll_amount += 3;
println!("You spend {} days trading and gained {}g",days_mid ,gold_amount_mid); 
println!("press 0 to continue");
gold_amount += gold_amount_mid;
days += days_mid;
functions::read_input(0,0);
}
if player_input == 4{
functions::print_status(food_amount,gold_amount,lumber_amount,herbs_amount,days);
}

}

}



