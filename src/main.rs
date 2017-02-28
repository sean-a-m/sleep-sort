extern crate rand;
extern crate num;

use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

//Return a vector of random integers
fn random_list(size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut xs: Vec<u32> = Vec::new();

    //Generate a list of random integers
    for _ in 0..size {
        xs.push(rng.gen_range(0, 1000));
    }
    return xs;
}

fn main() {

  //  let mut fixed_xs: [i32;100] = fixed_random_list();
    let size = 100;
    let mut xs: Vec<u32> = random_list(size);
    let mut results: Vec<u32> = Vec::new();
    let (tx, rx) = mpsc::channel();
    let rx_ref = Arc::new(Mutex::new(rx));

    let recieve_thread = thread::spawn(move || {
        let mut results: Vec<u32> = Vec::new();
        let rxer = rx_ref.lock().unwrap();
        for _ in 0..size {
            results.push(rxer.recv().unwrap())
        }
        return results
    });

    for i in 0..size {
        let tx = tx.clone();
        let x = xs[i].clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(x as u64));
            tx.send(x).unwrap(); 
        });
    }  

   let results = recieve_thread.join().unwrap();

    for x in &results {
        println!("{}",x);
    }
}



