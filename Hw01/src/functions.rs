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


fn can_afford(cost: i32, current_gold: i32)
