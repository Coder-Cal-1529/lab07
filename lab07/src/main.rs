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
}





























}
