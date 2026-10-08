use std::io;

pub fn read_input(max: i32,min: i32, input: char,){
let mut back1: i32 = 0;
let mut player_input: i32 = input as i32;
let mut count: i32 = 0;
while player_input > max || player_input < min || player_input == 0{
if player_input == 0{
return back1 = 1;
}else{
println!("please input one of the given options");
count += 1;
} if count == 5{
println!("returning to previous screen in 5 more invalid responses"); 
}
if count == 10{
println!("returning to previus screen");
return back1 = 1;
}
let mut input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
let player_input: char = input.trim().parse().expect("not a number");
}
return player_input = player_input
}
