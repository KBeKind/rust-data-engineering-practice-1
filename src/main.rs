use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rust_proj_8::create_fruit_salad;
use std::collections::{LinkedList, VecDeque};

use std::collections::HashMap;

use petgraph::graph::{self, Node, NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt::{self, write};

use std::collections::HashSet;

use std::collections::BTreeSet;

fn generate_fruit() -> &'static str {
    let fruits: [&str; 8] = [
        "banana", "apple", "orange", "grapes", "cherry", "fig", "berry", "bean",
    ];
    let mut rng = rand::thread_rng();
    fruits.choose(&mut rng).unwrap()
}

// #[derive(Parser)]
// #[clap(version = "3.0", author = "me", about = "number of fruits to include in salad")]
// struct Opts {
//     #[clap(short, long)]
//     number: usize,
// }

#[derive(Debug)]
struct Fighter {
    name: String,
}
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

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
    }

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
    }

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

    println!(
        "the frequencty of each number in the vector is {:?}",
        result
    );

    let mut languages: HashMap<String, i32> = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 newest 100 oldest): ");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }

    let mut graph = UnGraph::new_undirected();

    let fighers = [
        Fighter::new("John"),
        Fighter::new("Jane"),
        Fighter::new("Bob"),
        Fighter::new("Alice"),
        Fighter::new("Mike"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighers
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1);
    add_edge(&mut graph, &fighter_nodes, 0, 2);
    add_edge(&mut graph, &fighter_nodes, 1, 3);
    add_edge(&mut graph, &fighter_nodes, 2, 3);
    add_edge(&mut graph, &fighter_nodes, 2, 4);
    add_edge(&mut graph, &fighter_nodes, 0, 4);
    add_edge(&mut graph, &fighter_nodes, 3, 4);
    add_edge(&mut graph, &fighter_nodes, 2, 4);

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighers[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        match name.as_str() {
            "John" => println!("John is the best {}, {}", name, closeness),
            "Jane" => println!("Jane is the best {}, {}", name, closeness),
            "Bob" => println!("Bob is the best {}, {}", name, closeness),
            "Alice" => println!("Alice is the best {}, {}", name, closeness),
            "Mike" => println!("Mike is the best {}, {}", name, closeness),
            _ => println!("No one is the best {}, {}", name, closeness),
        }
        println!("----------------------");
    }

    let mut fruit_set = HashSet::new();
    println!("Gen 100 random fruits...");
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }
    println!("Num of unique fruits gen: {}", fruit_set.len());

    let fruits_list: Vec<&str> = vec![
        "banana", "apple", "orange", "grapes", "cherry", "fig", "berry", "bean", "mushroom",
    ];

    let amounts = [1, 3, 5, 7, 9];

    let mut rng = thread_rng();

    for amount in amounts.iter() {
        let mut fruits_set = BTreeSet::new();
        let mut shuffled_fruits = fruits_list.clone();
        shuffled_fruits.shuffle(&mut rng);

        for a_fruit in shuffled_fruits {
            fruits_set.insert(a_fruit);
            if fruits_set.len() >= *amount {
                break;
            }
        }
        println!("{}: {:?}", amount, fruits_set);
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
