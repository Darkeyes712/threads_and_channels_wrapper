mod threads;
extern crate rand;
use rand::Rng;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use threads::{Consume, Produce, ProducerConsumer};

/// Represents a producer that generates random `f64` values.
struct NumberProducer;

/// Represents a consumer that processes `f64` values.
struct NumberConsumer;

/// Represents a producer that generates sequential `i32` values.
struct NumberProducer2;

/// Represents a consumer that processes `i32` values.
struct NumberConsumer2;

impl Produce<f64> for NumberProducer {
    /// Produces 10 random `f64` values, sends them to a channel, and simulates work between sends.
    ///
    /// # Arguments
    /// * `tx`: A Sender through which produced `f64` values are sent.
    fn produce(&self, tx: Sender<f64>) {
        let mut rng = rand::thread_rng();
        let vec_dat: Vec<f64> = (0..=10).map(|_| rng.gen::<f64>()).collect();

        for i in vec_dat {
            println!("Produced: {}", i);
            tx.send(i).unwrap();
            // Simulating some work
            thread::sleep(Duration::from_millis(500));
        }
    }
}

impl Consume<f64> for NumberConsumer {
    /// Consumes `f64` values from a channel and prints them.
    ///
    /// # Arguments
    /// * `rx`: A Receiver from which `f64` values are received.
    fn consume(&self, rx: Receiver<f64>) {
        for received in rx.iter() {
            println!("Consumed: {:?}", received);
        }
    }
}

impl Produce<i32> for NumberProducer2 {
    /// Produces sequential `i32` values from 0 to 9, sends them to a channel, and simulates work between sends.
    ///
    /// # Arguments
    /// * `tx`: A Sender through which produced `i32` values are sent.
    fn produce(&self, tx: Sender<i32>) {
        for i in 0..=9 {
            println!("Produce_2: {}", i);
            tx.send(i).unwrap();
            // Simulating some work
            thread::sleep(Duration::from_millis(500));
        }
    }
}

impl Consume<i32> for NumberConsumer2 {
    /// Consumes `i32` values from a channel and prints them.
    ///
    /// # Arguments
    /// * `rx`: A Receiver from which `i32` values are received.
    fn consume(&self, rx: Receiver<i32>) {
        for received in rx.iter() {
            println!("Consumed_2: {:?}", received);
        }
    }
}

fn main() {
    let pc = ProducerConsumer::new(NumberProducer, NumberConsumer);
    let _pc_2 = ProducerConsumer::new(NumberProducer2, NumberConsumer2);
    pc.join_threads();
    _pc_2.join_threads();
    // Here, you can do other things in the main thread if needed.
}

// Explore different apporaches to having the data from the threads available from the outside of the thread itself.
// Add another trait/struct combination to the threads.rs file that contains a different approach to the data aquisition.
