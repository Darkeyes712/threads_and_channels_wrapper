mod threads;
extern crate rand;
use rand::Rng;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use threads::{Consume, Produce, ProducerConsumer};

struct NumberProducer;
struct NumberConsumer;

impl Produce<f64> for NumberProducer {
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
    fn consume(&self, rx: Receiver<f64>) {
        for received in rx.iter() {
            println!("Consumed: {:?}", received);
        }
    }
}

fn main() {
    let pc = ProducerConsumer::new(NumberProducer, NumberConsumer);
    pc.join_threads();
    // Here, you can do other things in the main thread if needed.
}
