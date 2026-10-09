use std::io;

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

pub fn roll_dice(seed: u64){
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

pub fn print_status(food: u64,gold:u64,lumber:u64,herbs:u64,days:u64){
println!("=======================");
println!("Gold: {}g", gold);
println!("Food: {} lbs", food);
println!("Lumber: {} logs", lumber); 
println!("Medicinal Herbs: {}", herbs);
println!("Days in Game: {}", days);
println!("=======================");
}

pub fn print_menu(menu: i32){
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
println!("1 -- Upgrade Gun");
println!("2 -- Upgrade Axe");
println!("3 -- Upgrade Knife");
println!("=-=-=-=-=-=-=-=-=-=-=-=");
}
}
