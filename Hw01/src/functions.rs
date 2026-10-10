use std::io;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
pub fn read_input(max: i32,min: i32)->i32{
let mut player_input: i32;
let mut input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
let input_check: Result<i32,_> = input.trim().parse();
match input_check{
Ok(i32) => player_input = input_check.expect("i32"),
Err(_) => player_input = max+10,
}
let mut count: i32 = 0;
while player_input < min || player_input > max || player_input==0{
if player_input == 0{
return player_input;
}else{
println!("please input one of the given options");
input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
let input_check: Result<i32,_> = input.trim().parse();
match input_check{
Ok(i32) => player_input = input_check.expect("i32"),
Err(_) => player_input = max+10,
}
count +=1;
}if count == 5{
println!("returning to previous screen in 5 more invalid responses"); 
}
if count == 10{
println!("returning to previus screen");
player_input = 0;
return player_input;
}
}
return player_input;
}

pub fn roll_dice(seed: u64)->u64{
let mut rng = StdRng::seed_from_u64(seed);
let dice: u64 = rng.random_range(1..=6);
return dice;
}

pub fn roll_many_dice(mut seed: u64,mut roll_count: u64)->u64{
let mut dice:u64 = 0;
while roll_count > 0{
dice += roll_dice(seed);
seed+=1;
roll_count-=1;
}
return dice;
}

pub fn can_afford(cost: u64, current_gold: u64)-> bool{
let mut can_afford: bool = false;
if current_gold < cost {
println!("You do not currently have the funds to purchase this item");
return can_afford; 
}else{
can_afford = true;
return can_afford;
}
}

pub fn hunt(mut seed: u64,mut gun_lvl: u64, knife_lvl: u64, cart: bool)->u64{
let mut food_gain_mid: u64 = 0;
let mut food_gain: u64 = 0;
let mut dice: u64; 
while gun_lvl > 0{
dice = roll_dice(seed);
if dice == 0 {
food_gain_mid = 0;
println!("The hunt went poorly, 0 food gained");
} else if dice <= 3{
food_gain_mid = 10;
println!("The hunt went ok, 10 food gained");
}else if dice <= 5{
food_gain_mid =20;
println!("The hunt went great, 20 food gained");
}else if dice == 6{
food_gain_mid = 40;
println!("The hunt went perfectly, 40 food gained");
}
seed+=1;
gun_lvl -=1;
}
food_gain += food_gain_mid*knife_lvl;
if food_gain > 60{
	if cart == true{
		if food_gain > 120{
		food_gain = 120; 
		}
	}else{
	food_gain = 60;
	}
}
println!("Total food gained {}X{}", food_gain, knife_lvl);
return food_gain;
}

pub fn gather1(mut seed: u64, mut axe_lvl: u64, cart: bool)-> u64{
let mut lumber_gain_mid: u64 =0;
let mut lumber_gain: u64 = 0;
let mut dice: u64;
while axe_lvl > 0{
dice = roll_dice(seed);
if dice == 0 {
lumber_gain_mid = 0;
} else if dice <= 3{
lumber_gain_mid =0;
}else if dice <= 5{
lumber_gain_mid = 10;
println!("you gatherd lots of resorces, gained 10 lumber and 15 herbs");
}else if dice == 6{
lumber_gain_mid = 20;
println!("you gatherd an absurd amount of resources, gained 20 lumber and 10 herbs");
}
seed+=1;
axe_lvl -=1;
}
lumber_gain += lumber_gain_mid;
if lumber_gain > 30{
	if cart == true{
		if lumber_gain > 60{
		lumber_gain = 60; 
		}
	}else{
	lumber_gain = 30;
	}
}
return lumber_gain;
}

pub fn gather2(mut seed: u64, mut knife_lvl: u64, cart: bool)-> u64{
let mut herb_gain_mid: u64 =0;
let mut herb_gain: u64 = 0;
let mut dice: u64;
while knife_lvl > 0{
dice = roll_dice(seed);
if dice == 0 {
herb_gain_mid = 0;
println!("you were unable to find any resorces");
} else if dice <= 3{
herb_gain_mid = 10;
println!("you gatherd some herbs, gained 10 herbs");
}else if dice <= 5{
herb_gain_mid = 15;
}else if dice == 6{
herb_gain_mid = 10;
}
seed+=1;
knife_lvl -=1;
}
herb_gain += herb_gain_mid;
return herb_gain;
}

pub fn print_status(food: u64,gold:u64,lumber:u64,herbs:u64,days:u64){
println!("=======================");
println!("Gold: {}g", gold);
println!("Food: {} lbs", food);
println!("Lumber: {} logs", lumber); 
println!("Medicinal Herbs: {}", herbs);
println!("Days in Game: {}", days);
println!("=======================");
}

pub fn print_menu(menu: i32, gold:u64){
if menu == 0{
println!("=========================");
println!("Welcome to Trail Traders");
println!("Press 1 to start the game.");
println!("Press 0 to end session.");
println!("=========================");
}else if menu == 1{
println!("=========================");
println!("1 -- Enter Store");
println!("2 -- Go on an outing");
println!("3 -- Trade with locals");
println!("4 -- Check current stats");
println!("0 -- End Game");
println!("=========================");
}else if menu == 2{
println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
println!("Current Gold: {}g", gold);
println!("1 -- Purchase nicer clothes: (gold amount)");
println!("2 -- Purchase improved cart: (gold amount)");
println!("3 -- Gear upgrades");
println!("4 -- Purchase food: 5g per 1lb");
println!("0 -- Main Menu");
println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
}else if menu == 3{
println!("------------------------");
println!("1 -- Go Hunting");
println!("2 -- Gather Resorces");
println!("0 -- Main Menu");
println!("------------------------");
}else if menu == 4{
println!("=-=-=-=-=-=-=-=-=-=-=-=");
println!("Current Gold: {}g", gold);
println!("1 -- Upgrade Gun");
println!("2 -- Upgrade Axe");
println!("3 -- Upgrade Knife");
println!("=-=-=-=-=-=-=-=-=-=-=-=");
}
}
