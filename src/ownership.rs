// fn main() {
//     let mut s = String::from("hello"); //the value of the s is change in the next line and this variable owner goes out of scope , that's why the value is dropped.
//     s = String::from("Hashir");

//     println!("This the value of s: {s} World",);
// }

// fn main () {
//     let s1 = String::from("Rust Programming Language");
//      let s2 = s1.clone();

//      println!("The value of s1 is : {s1} and the value of s2 is : {s2}") 
// }

// fn main() {
//     let s = String::from("hello"); 

//     takes_ownership(s);             
//                                    

//     let x = 5;                     

//     makes_copy(x);                 
//                                   
//     println!("{}", x);
// } 

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } 
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } 
// fn main() {
//     let s1 = gives_ownership();
//     let s2 = String::from("Hashir");
//     let s3 = takes_and_gives_back(s2);
//     println!("{}, {}", s1, s3);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// client.rs
// fn main  () {
//     let s = String::from("Rust Programming Language");
//     let len = calculate_length(&s);

//     println!("The length of : ' {s} ' is {len}");
// }
// fn calculate_length (s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("Hello");

//     change(&mut s); 

//     println!("{}", s); 
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", Hashir"); 
// }


// fn main() {
//     let s = String::from("Hello");
//     let r = &s;
//     return_reference(r);
//     println!("{}", r);
// }

// fn return_reference(s: &String) -> &String {
//     s
// }
