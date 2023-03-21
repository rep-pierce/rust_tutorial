#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";
//     io::stdin().read_line(&mut name)
//         .expect("Didn't Recieve Input");

//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.141592;
//     let age = "47";
//     let mut age: u32 = age.trim().parse()
//         .expect("Age wasn't assigned a number");
//     age = age + 1;
//     println!("I'm {} and I want ${}", age, ONE_MIL);
// }

// fn main() {
//     // Unsigned integer : u8, u16, u32, u64, u128, usize
//     // Signed integer : i8, i16, i32, i64, i128, isize
//     println!("Max u32 : {}", u32::MAX);
//     println!("Max u64 : {}", u64::MAX);
//     println!("Max u128 : {}", u128::MAX);
//     println!("Max usize : {}", usize::MAX);
//     println!("Max u32 : {}", u32::MAX);
//     println!("Max f32 : {}", f32::MAX);
//     println!("Max f64 : {}", f64::MAX);
// }

// fn main() {
//     let is_true = true;
//     let my_grade = 'A';
// }

// fn main() {
//     let num_1: f32 = 1.111111111111111;
//     println!("f32 : {}", num_1 + 0.111111111111111); //answer: f32 : 1.2222223
//     let num_2: f64 = 1.111111111111111;
//     println!("f364 : {}", num_2 + 0.111111111111111); //answer: f64 : 1.2222222222222219
// }

// fn main() {
//     let mut num_3: u32 = 5;
//     let num_4: u32 = 4;
//     println!("5 + 4 = {}", num_3 + num_4);
//     println!("5 - 4 = {}", num_3 - num_4);
//     println!("5 * 4 = {}", num_3 * num_4);
//     println!("5 / 4 = {}", num_3 / num_4);
//     println!("5 % 4 = {}", num_3 % num_4);
//     num_3 += 1; //can be done with any of the above math operators so like num_3 *= 3
// }

// fn main() {
//     let random_num = rand::thread_rng().gen_range(1..101);
//     println!("Random Number: {}", random_num);
// }

// fn main() {
//     let age = 8;
//     if (age >= 1) && (age <=18){
//         println!("Important Birthday");
//     } else if (age == 21) || (age == 50){
//         println!("Important Birthday");
//     } else if age >= 65 {
//         println!("Important Birthday");
//     } else {
//         println!("Not an Important Birthday");
//     }
// }

// fn main() {
//     let mut my_age = 25;
//     let can_vote = if my_age >= 18{
//         true
//     } else {
//         false
//     };
//     println!("Can Vote : {}", can_vote);
// }

// fn main() {
//     let age2 = 25;
//     match age2 {
//         1..=18 => print!("Important Birthday"),
//         21 | 50 => print!("Important Birthday"),
//         65..=i32::MAX => print!("Important Birthday"),
//         _ => print!("Not an Important Birthday"),
//     };
// }

// fn main() {
//     let my_age = 19;
//     let voting_age = 18;
//     match my_age.cmp(&voting_age){
//         Ordering::Less => print!("Can't Vote"),
//         Ordering::Greater => print!("Can Vote"),
//         Ordering::Equal => print!("You gained the right to vote"),
//     };
// }

fn main() {
    let arr_1 = [1,2,3,4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());
}