// use std::io;

// fn fibbonaci(n: i32) -> i32 {
//     if n <= 1 {
//         return n;
//     }
//     return fibbonaci(n - 1) + fibbonaci(n - 2);
// }

// Working with user input
// fn main() {
//     let mut input = String::new();
//     println!("Enter a number: ");
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     let number: u32 = input.trim().parse::<u32>().unwrap();

//     for i in 0..number {
//         println!("{}", fibbonaci(i as i32));
//     }
// }

// Working with type conversion and casting

// fn main() {
//     let x: i32 = 10;
//     let y: u32 = 20;

//     let z = x as u32 + y;

//     println!("The value of z is: {}", z);
// }

// Control flow, loops, operators
// fn main() {
//     let x = 10;
//     let y = 20;

//     if x > y {
//         println!("x is greater than y");
//     } else {
//         println!("y is greater than x");
//     }

//     let mut counter = 0;

//     loop {
//         counter += 1;
//         println!("The value of counter is: {}", counter);

//         if counter == 10 {
//             break;
//         }
//     }

//     let mut number = 0;

//     while number < 10 {
//         number += 1;
//         println!("The value of number is: {}", number);
//     }

//     for i in 0..10 {
//         println!("The value of i is: {}", i);
//     }
// }

// Two Sum

// use std::collections::HashMap;

// fn jwo_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     // Use HashMap

//     let result: Vec<i32> = Vec::new();
//     let mut cache: HashMap<i32, i32> = HashMap::new();

//     for (index, num) in nums.iter().enumerate() {
//         let complement = target - num;
//         if cache.contains_key(&complement) {
//             return vec![*cache.get(&complement).unwrap(), index as i32];
//         }
//         cache.insert(*num, index as i32);
//     }
//     return result;
// }

// fn main() {
//     let nums = vec![2, 7, 11, 15];
//     let target = 9;
//     let result = two_sum(nums, target);
//     println!("{:?}", result);
// }

// fn main() {
//     let x = [1, 2, 3, 4, 5];

//     for (index, num) in x.iter().enumerate() {
//         println!("{}", num);
//     }
// }

// fn plus_one(digits: Vec<i32>) -> Vec<i32> {
//     let mut result: Vec<i32> = Vec::new();
//     let mut carry = 1;

//     for i in (0..digits.len()).rev() {
//         let sum = digits[i] + carry;
//         result.push(sum % 10);
//         carry = sum / 10;
//     }

//     if carry > 0 {
//         result.push(carry);
//     }

//     result.reverse();
//     return result;
// }

// fn main() {
//     let digits = vec![1, 2, 3];
//     let result = plus_one(digits);
//     println!("{:?}", result);
// }

// fn find(value: i32, vec: Vec<i32>) -> Option<usize> {
//     for (index, &item) in vec.iter().enumerate() {
//         if item == value {
//             return Some(index);
//         }
//     }
//     None
// }

// fn main() {
//     match find(99, vec![1, 2, 3, 4, 5]) {
//         Some(index) => println!("The index is: {}", index),
//         None => println!("The value is not found"),
//     }
// }

// Rust OWNERSHIP and BORROWING

// fn main() {
//     let st1 = String::from("Hello, World");
//
//     let length = calculate_length(st1);
//
//     println!("The length of the string is: {}", st1);
//
//     let numbers = vec![1, 2, 3, 4, 5];
//
//     let new_numbers = add_one_incorrect(numbers);
//     print!("{:?}", new_numbers);
// }
//
// fn calculate_length(s: String) -> usize {
//     s.len()
// }
//
// fn add_one_incorrect(mut numbers: Vec<i32>) -> Vec<i32> {
//     numbers.push(1);
//     numbers
// }

// use chrono::{DateTime,Local};

// fn main() {
//     println!("Hello World");
//     let local = Local::now();
//     println!("{}", local.format("%A"));
// }

// fn main() {
//     // Owners can have immutable or mutable references but not both at the same time

//     let mut s = String::from("Hello, World");

//     let r1 = &mut s;

//     let r2 = &mut s; // This will throw an error borrowed mutable also immutable

//     println!("{} and {}", r1, r2);
// }

// #[derive(Debug)]
// struct Test {
//     name: String,
//     x: &Vec<i32>,
// }

// fn exceptional_lifetimes(mut passing_through: Test) -> Test {
//     let x: Vec<i32> = vec![1, 2, 3, 4, 5];
//     passing_through.name = String::from("Hello, World");
//     passing_through.x = &x;
//     passing_through
// }

// fn main() {
//     let _x = vec![1, 2, 3, 4, 5];
//     let test = Test {
//         name: String::from("Hello, World"),
//         x: &_x,
//     };

//     let result = exceptional_lifetimes(test);
//     println!("{:?}", result);
// }

// Palindrome number

// fn is_palindrome(s: i32) -> bool {
//     if s < 0 || (s % 10 == 0 && s != 0) {
//         return false;
//     } else {
//         let mut x = s;
//         let mut reversed = 0;
//         while x > reversed {
//             reversed = reversed * 10 + x % 10;
//             x /= 10;
//         }

//         return x == reversed || x == reversed / 10;
//     }
// }

// fn main() {
//     let x = 121;
//     let result = is_palindrome(x);
//     println!("{}", result);
// }

// use std::thread;

// fn threading() {
//     let x = 10;
//     let handle = thread::spawn(|| {
//         println!("Hello from a thread, the number is {}", x);
//     });
//     handle.join().unwrap();
// }

// fn main() {
//     threading();
// }

// ! What are traits and how are they different from interfaces?
// *Traits are similar to interfaces in other languages.
// *A trait is a collection of methods defined for an unknown type: Self. They can access other methods implemented in the same trait.

// ! Why doesn't Rust have a garbage collector?
// *Rust doesn't have a garbage collector because it uses a concept called ownership.
// *The ownership system allows Rust to manage memory without a garbage collector. It keeps track of the memory and cleans it up when it's no longer needed.

// ! Name three examples of how lifetimes are created in Rust (explicitly and implicitly)!
// *1. Function parameters
// *2. Struct fields
// *3. Return values

// ! Why is immutability for variables important?
// *Immutability is important because it helps prevent bugs and makes the code easier to reason about.
// *It also helps with concurrency and parallelism by reducing the chances of data.

// ! What does the Sync marker trait do?
// *The Sync marker trait indicates that a type is safe to share between threads.
// *It ensures that the type can be safely accessed from multiple threads without causing data

// Implementing the linked list

// struct Node {
//     value: i32,
//     next: Option<Box<Node>>,
// }

// impl Node {
//     fn new(value: i32)
//     -> Node {
//         Node { value, next: None }
//     }
// }

// struct LinkedList {
//     head: Option<Box<Node>>,
// }

// impl LinkedList {
//     fn new() -> LinkedList {
//         LinkedList { head: None }
//     }

//     fn add(&mut self, value: i32) {
//         let mut new_node: Box<Node> = Box::new(Node::new(value));

//         match self.head.take() {
//             Some(old_head) => {
//                 new_node.next = Some(old_head);
//                 self.head = Some(new_node);
//             }
//             None => {
//                 self.head = Some(new_node);
//             }
//         }
//     }

//     fn remove(&mut self) -> Option<i32> {
//         self.head.take().map(|node| {
//             self.head = node.next;
//             node.value
//         })
//     }

//     fn print(&self) {
//         let mut current = &self.head;
//         while let Some(node) = current {
//             println!("{}", node.value);
//             current = &node.next;
//         }
//     }
// }

// fn main() {
//     let mut list = LinkedList::new();
//     list.add(1);
//     list.add(2);
//     list.add(3);
//     list.print();
//     list.remove();
//     list.print();
// }

// fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) {
//     let mut result: Vec<i32> = Vec::new();
//     let mut i = 0;
//     let mut j = 0;

//     while i < arr1.len() && j < arr2.len() {
//         if arr1[i] == arr2[j] {
//             result.push(arr1[i]);
//             i += 1;
//             j += 1;
//         } else {
//             break;
//         }
//     }
//     result.len();
// }

// fn main() {
//     let arr1 = vec![1, 2, 3, 4, 5];
//     let arr2 = vec![1, 2, 3, 4, 5];
//     let result = longest_common_prefix(arr1, arr2);
//     println!("{:?}", result);
// }

// struct Solution;

// impl Solution {
//     pub fn longest_common_prefix() {
//         // let mut result: Vec<i32> = Vec::new();
//         // let mut i = 0;
//         // let mut j = 0;

//         // while i < arr1.len() && j < arr2.len() {
//         //     if arr1[i] == arr2[j] {
//         //         result.push(arr1[i]);
//         //         i += 1;
//         //         j += 1;
//         //     } else {
//         //         break;
//         //     }
//         // }
//         // result.len();

//         print!("Hello, World");
//     }
// }

// fn main() {
//     // let arr1 = vec![1, 2, 3, 4, 5];
//     // let arr2 = vec![1, 2, 3, 4, 5];
//     let result = Solution::longest_common_prefix();
//     println!("{:?}", result);
// }

// struct Stack {
//     stack: Vec<i32>,
//     size: i32,
// }

// impl Stack {
//     fn new() -> Stack {
//         Stack {
//             stack: Vec::new(),
//             size: 0,
//         }
//     }

//     fn push(&mut self, value: i32) {
//         self.stack.push(value);
//         self.size += 1;
//     }

//     fn pop(&mut self) -> Option<i32> {
//         self.size -= 1;
//         self.stack.pop()
//     }

//     fn peek(&self) -> Option<&i32> {
//         self.stack.last()
//     }

//     fn is_empty(&self) -> bool {
//         self.stack.is_empty()
//     }

//     fn print(&self) {
//         for i in &self.stack {
//             println!("{}", i);
//             println!("Size:{}", self.size);
//         }
//     }
// }

// fn main() {
//     let mut stack = Stack::new();
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     stack.print();
//     stack.pop();
//     stack.print();
//     println!("{:?}", stack.peek());
//     println!("{}", stack.is_empty());
// }

// use rand::Rng;

// fn generate_random_number() -> i32 {
//     let random_number = rand::thread_rng().gen_range(0..3);
//     random_number
// }

// fn main() {
//     println!("Enter your choice: ");

//     let mut choice: String = String::new();

//     std::io::stdin()
//         .read_line(&mut choice)
//         .expect("Failed to read line");

//     let choices: Vec<&str> = vec!["Stone", "Paper", "Scissors"];

//     let random_number: &mut i32 = &mut generate_random_number();

//     println!(
//         "The computer choice is: {}",
//         choices[*random_number as usize]
//     );
//     println!("Your choice is: {}", choice);

//     while choice.trim() == choices[*random_number as usize] {
//         println!("It's a tie, play again");
//         let mut choice: String = String::new();
//         println!("Enter your choice: ");
//         std::io::stdin()
//             .read_line(&mut choice)
//             .expect("Failed to read line");
//         *random_number = generate_random_number();
//     }

//     if choice.trim() == "Stone" && choices[*random_number as usize] == "Scissors" {
//         println!("You win");
//     } else if choice.trim() == "Paper" && choices[*random_number as usize] == "Stone" {
//         println!("You win");
//     } else if choice.trim() == "Scissors" && choices[*random_number as usize] == "Paper" {
//         println!("You win");
//     } else {
//         println!("You lose");
//     }
// }

// use std::mem;

// struct MyStruct {
//     a: u8,
//     b: u8,
//     c: u8,
// }

// fn main() {
//     print!("This is the size:{}", 3 * mem::size_of::<u8>());
//     assert_eq!(mem::size_of::<MyStruct>(), 3 * mem::size_of::<u8>());
//     // assert_eq!(
//     //     mem::size_of::<[MyStruct; 2]>(),
//     //     3 * mem::size_of::<u8>() * 2
//     // );
// }

// trait MyG {
//     fn a(&self) -> u8;
//     fn b(&self) -> u8;
//     fn c(&self) -> u8;
// }

// impl MyG for MyStruct {
//     fn a(&self) -> u8 {
//         self.a
//     }

//     fn b(&self) -> u8 {
//         self.b
//     }

//     fn c(&self) -> u8 {
//         self.c
//     }
// }

// fn main() {
//     let my_struct = MyStruct { a: 1, b: 2, c: 3 };
//     let my_g = MyG::a(&my_struct);
//     println!("{}", my_g);
// }

// struct Node {
//     value: i32,
//     next: Option<Box<Node>>,
// }

// impl Node {
//     fn new(value: i32) -> Node {
//         Node { value, next: None }
//     }
// }

// struct LinkedList {
//     head: Option<Box<Node>>,
// }

// impl LinkedList {
//     fn new() -> LinkedList {
//         LinkedList { head: None }
//     }

//     fn add(&mut self, value: i32) {
//         let mut new_node: Box<Node> = Box::new(Node::new(value));

//         match self.head.take() {
//             Some(old_head) => {
//                 new_node.next = Some(old_head);
//                 self.head = Some(new_node);
//             }
//             None => {
//                 self.head = Some(new_node);
//             }
//         }
//     }

//     fn remove(&mut self) -> Option<i32> {
//         self.head.take().map(|node| {
//             self.head = node.next;
//             node.value
//         })
//     }

//     fn print(&self) {
//         let mut current = &self.head;
//         while let Some(node) = current {
//             println!("{}", node.value);
//             current = &node.next;
//         }
//     }
// }

// struct CustomStack {
//     stack: Vec<i32>,
//     maxSize: i32,
// }

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// impl CustomStack {
//     fn new(maxSize: i32) -> Self {
//         CustomStack {
//             stack: Vec::new(),
//             maxSize: maxSize,
//         }
//     }

//     fn push(&mut self, x: i32) {
//         if self.stack.len() < self.maxSize as usize {
//             self.stack.push(x);
//         }
//     }

//     fn pop(&self) -> i32 {
//         if self.stack.is_empty() {
//             return -1;
//         }
//         return self.stack[self.stack.len() - 1];
//     }

//     fn increment(&mut self, k: i32, val: i32) {
//         for i in 0..k {
//             if i < self.stack.len() as i32 {
//                 self.stack[i as usize] += val;
//             }
//         }
//     }
// }

// struct Ranks;

// impl Ranks {
//     fn new() -> Self {
//         Ranks
//     }

//     fn arrayRankTransform(&self, arr: Vec<i32>) -> Vec<i32> {
//         let mut sorted_arr = arr.clone();
//         sorted_arr.sort();
//         let mut map = std::collections::HashMap::new();
//         let mut rank = 1;
//         for i in 0..sorted_arr.len() {
//             if !map.contains_key(&sorted_arr[i]) {
//                 map.insert(sorted_arr[i], rank);
//                 rank += 1;
//             }
//         }
//         let mut result = Vec::new();
//         for i in 0..arr.len() {
//             result.push(*map.get(&arr[i]).unwrap());
//         }
//         result
//     }
// }

// fn main() {
//     let arr = vec![40, 10, 20, 30];
//     let ranks = Ranks::new();
//     let result = ranks.arrayRankTransform(arr);
//     println!("{:?}", result);
// }
mod helpers;
use helpers::namehelpers::get_full_name;

// fn main() {
//     let fname: String = String::from("Dev");
//     let lname: String = String::from("Parekh");

//     let full_name = get_full_name(&fname, &lname);

//     println!("{}", full_name);
// }

use helpers::txHelpers::TransactionLog;

fn main() {
    let mut log = TransactionLog::new();
    log.append(1);
    log.append(2);
    log.append(3);
    log.append(4);
    log.append(5);
    log.print();
}
