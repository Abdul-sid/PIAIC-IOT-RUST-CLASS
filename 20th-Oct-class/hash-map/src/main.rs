use std::collections::HashMap;

fn main() {

// First method

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);

// println!("{:?}" , scores);
println!("{:?}" , score);


//Second method

//Hashmap accessing values

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// match scores.get(&String::from("Yellow")){
//     Some(score) => println!("Your score is {}" , score),
//     None => println!("Team does not exist")
// }
// }
