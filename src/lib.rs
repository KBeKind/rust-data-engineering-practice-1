use rand::{seq::SliceRandom, rngs::ThreadRng};
use rand::thread_rng;
use std::collections::{VecDeque, LinkedList};


pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {

    let mut fruits: Vec<String> = vec!["banana".to_string(), "apple".to_string(), "orange".to_string(), "grapes".to_string()];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()

}