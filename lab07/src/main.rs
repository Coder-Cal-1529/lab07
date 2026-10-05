use rand::rng;	
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];
let mut i: i32 = 0;

println!("The high for the week are:");
for i in 0..7 {
if i <=4{
 println!("{:?}:{:?}", DAYS[i], highs[i]);
}else{
highs.push(68);
println!("{:?}:{:?}", DAYS[i], highs[i]);
}
println!("the average temp for this week is ");
println!("the hottest day for this week is {:?} at {:?}", DAYS[hottest_day(Vec<i32>)], highs[hottest_day(Vec<i32>)]);



fn hottest_day(log: &Vec<i32>) -> usize {
let mut v: i32 = 0;
let mut highest_index: usize = 0;
let mut highest_temp: i32 = log[0];
if log.ln > 0{
for i in 0..7{
if log[i] > highest_temp{
highest_temp = log[i]
highest_index = i;
}else{
}
}return highest_index
}
else{
highest_index = 0 
}
}
}





fn average_temp(log: &Vec<i32>)->f64{
let mut total: f64 = 0.0;
if log.len() >0 {
        for i in 0..7{
        total += log[i];
        }total = total/log.len();
}else{
total = 0.0
}















}
