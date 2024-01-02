
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{VecDeque, LinkedList};
use clap::Parser;
use rust_proj_8::create_fruit_salad;

use std::collections::HashMap;

// #[derive(Parser)]
// #[clap(version = "3.0", author = "me", about = "number of fruits to include in salad")]
// struct Opts {
//     #[clap(short, long)]
//     number: usize,
// }

fn main() {


    
  // DATA STRUCTURES

  // A VECTOR IS A GROWABLE ARRAY
    let mut fruit: Vec<&str> = vec!["banana", "apple", "orange", "grapes", "cherry", "fig"];

    // SCRAMBLE/SHUFFLE THE FRUIT
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    fruit.shuffle(&mut rng);

    println!("Fruit Salad");
    for (i, item) in fruit.iter().enumerate() {
        
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    };
    
    // VECDEQUE IS A DOUBLE ENDED QUEUE, IT MEANS YOU CAN PUSH AND POP FROM BOTH ENDS


    let mut salad: VecDeque<&str> = VecDeque::new();

    salad.push_back("anotherfruit");
    salad.push_back("morefruit");
    salad.push_back("strawberry");

    // SCRAMBLE/SHUFFLE THE SALAD
    let mut rng2: rand::prelude::ThreadRng = thread_rng();
    let mut salad: Vec<_> = salad.into_iter().collect();
    fruit.shuffle(&mut rng2);

    
    // CONVERT IT BACK TO VecDeque
    let mut salad: VecDeque<_> = salad.into_iter().collect();
    salad.push_front("afruitagain");

    println!("Fruit Salad2");
    for (i, item) in salad.iter().enumerate() {
        
        if i != salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    };


    // LINKED LIST
    // ALLOWS INSERTION OR DELETION FROM THE MIDDLE OF THE LIST
    // NOT USED AS OFTEN

    let mut stuff: LinkedList<&str> = LinkedList::new();
    stuff.push_back("Thing1");
    stuff.push_back("Thing2");
    stuff.push_back("Thing3");


    stuff.push_back("thing4");

    // GET NUMBER OF FRUITS USER REQUESTED
    //let opts: Opts = Opts::parse();

    // CREATE THE SALAD

    // PRINT THE SALAD


    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 3];
    let result: Vec<(i32, u32)> = logic(numbers);

    println!("the frequencty of each number in the vector is {:?}", result);


    let mut languages: HashMap<String, i32> = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 newest 100 oldest): ");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }

}

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    
    // HASHMAP
    // HASHMAP HAS GREAT PERFORMANCE
    let mut frequencies: HashMap<i32, u32> = HashMap::new();
    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result: Vec<(i32, u32)> = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result

}


// MORE WITH HASMAPS
fn init_languages() -> HashMap<String, i32> {
    let mut languages: HashMap<String, i32> = HashMap::new();
    languages.insert(String::from("Rust"), 2013);
    languages.insert(String::from("JavaScript"), 1995);
    languages.insert(String::from("Python"), 1991);
    languages.insert(String::from("Go"), 2009);
    languages.insert(String::from("C"), 1972);
    languages.insert(String::from("C++"), 1983);
    languages.insert(String::from("Java"), 1995);
    languages.insert(String::from("PHP"), 1995);
    languages.insert(String::from("C#"), 2000);
    languages.insert(String::from("Swift"), 2014);
    languages.insert(String::from("Kotlin"), 2010);
    languages.insert(String::from("Scala"), 2003);
    languages.insert(String::from("Ruby"), 1995);
    languages.insert(String::from("Lua"), 1995);
    languages.insert(String::from("Elixir"), 2014);
    languages.insert(String::from("Haskell"), 1990);
    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    // SUBTRACT CREATION YEAR FROM 2024
    for year in years_active.values_mut() {
        *year = 2024 - *year;
    }

    let min_year = years_active.values().min().unwrap_or(&0);
    let max_year = years_active.values().max().unwrap_or(&0);

    let mut weights: HashMap<String, i32> = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1;
        weights.insert(language.to_string(), weight);
    }
    weights
}