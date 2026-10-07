use rand::Rng;	
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];
let mut highest_index :usize = 0;
 
println!("The highs for the week are:");
for i in 0..7 {
if i <=4{
 println!("{:?}:{:?}", DAYS[i], highs[i]);
}else{
let mut rng = rand::rng();
let roll: i32 = rng.random_range(60..=100);
highs.push(roll);
println!("{:?}:{:?}", DAYS[i], highs[i]);
}
}
println!("the average temp for this week is {:.2}. ", average_temp(&highs));

let mut hot_day: usize;
hot_day= hottest_day(&highs);
println!("the hottest day for this week is {:?} at {:?}.", DAYS[hot_day], highs[hot_day]);
println!(" {:?}/7 days were above 75 degrees.",count_above(&highs,75));
}


fn hottest_day(log: &Vec<i32>) -> usize {
let mut highest_index: usize = 0;
let length_of_array: usize = 7;
let mut highest_temp: i32 = log[0];
if length_of_array >= 0 as usize{
for i in 0..7{
if log[i] > highest_temp{
highest_temp = log[i];
highest_index = i as usize;
}
else{}
}
return highest_index;
}
else{
highest_index = 0 as usize;
return highest_index
}
}




fn average_temp(log: &Vec<i32>)->f64{
let mut total: f64 = 0.0;
let log_len: f64 = log.len() as f64;
if log_len >0.0 {
        for i in 0..log_len as i32{
        total += log[i as usize] as f64;
        }
total = total/log_len;
}else{
total = 0.0;
}
return total;
}


fn count_above(log: &Vec<i32>, threshold: i32) -> usize{
let mut count: usize = 0;
let log_size: i32 = log.len() as i32;
if log_size > 0{
for i in 0..log_size{
if log[i as usize] >= threshold{
count += 1;
}
}
}else{
count = 0 as usize;
}
return count;
}
