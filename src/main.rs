// ################################################
// ASYNC
// ################################################

mod async_threads;
mod sync_threads;
extern crate rand;
use async_threads::{AsyncConsume, AsyncProduce, AsyncProducerConsumer};
use async_trait::async_trait;

struct IntProducer;
struct FloatProducer;
struct IntConsumer;
struct FloatConsumer;

#[async_trait]
impl AsyncProduce<i32> for IntProducer {
    async fn produce(&self, tx: tokio::sync::mpsc::Sender<i32>) {
        for i in 0..10 {
            if let Err(e) = tx.send(i).await {
                println!("Failed to send {}: {}", i, e);
            }
            println!("{}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        drop(tx);
    }
}

#[async_trait]
impl AsyncProduce<f64> for FloatProducer {
    async fn produce(&self, tx: tokio::sync::mpsc::Sender<f64>) {
        for i in 0..10 {
            tx.send(i as f64 * 0.5).await.unwrap(); // Sending floats for demonstration.
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Sleep for demonstration.
        }
    }
}

#[async_trait]
impl AsyncConsume<i32> for IntConsumer {
    async fn consume(&self, mut rx: tokio::sync::mpsc::Receiver<i32>) {
        while let Some(val) = rx.recv().await {
            println!("Int received: {}", val);
        }
    }
}

#[async_trait]
impl AsyncConsume<f64> for FloatConsumer {
    async fn consume(&self, mut rx: tokio::sync::mpsc::Receiver<f64>) {
        while let Some(val) = rx.recv().await {
            println!("Float received: {}", val);
        }
    }
}

// Async Main
#[tokio::main] // This attribute will set up a Tokio runtime for you
async fn main() {
    let int_channel = AsyncProducerConsumer::new(IntProducer, IntConsumer);
    let float_channel = AsyncProducerConsumer::new(FloatProducer, FloatConsumer);

    int_channel.join_tasks().await; // Join tasks to wait for them to complete concurrently
    float_channel.join_tasks().await;
}

// ################################################
// SYNC
// ################################################

// use rand::{rngs::StdRng, Rng, SeedableRng};
// use std::sync::mpsc::Receiver; // sync threads
// use std::sync::mpsc::Sender; // sync threads
// use std::thread; // sync threads
// use std::time::Duration;
// use sync_threads::{Consume, Produce, ProducerConsumer}; // sync threads

// /// Represents a producer that generates random `f64` values.
// struct NumberProducer;

// /// Represents a consumer that processes `f64` values.
// struct NumberConsumer;

// /// Represents a producer that generates sequential `i32` values.
// struct NumberProducer2;

// /// Represents a consumer that processes `i32` values.
// struct NumberConsumer2;

// impl Produce<f64> for NumberProducer {
//     /// Produces 10 random `f64` values, sends them to a channel, and simulates work between sends.
//     ///
//     /// # Arguments
//     /// * `tx`: A Sender through which produced `f64` values are sent.
//     fn produce(&self, tx: Sender<f64>) {
//         let mut rng = rand::thread_rng();
//         let vec_dat: Vec<f64> = (0..=10).map(|_| rng.gen::<f64>()).collect();

//         for i in vec_dat {
//             println!("Produced: {}", i);
//             tx.send(i).unwrap();
//             // Simulating some work
//             thread::sleep(Duration::from_millis(500));
//         }
//     }
// }
// impl Consume<f64> for NumberConsumer {
//     /// Consumes `f64` values from a channel and prints them.
//     ///
//     /// # Arguments
//     /// * `rx`: A Receiver from which `f64` values are received.
//     fn consume(&self, rx: Receiver<f64>) {
//         for received in rx.iter() {
//             println!("Consumed: {:?}", received);
//         }
//     }
// }

// impl Produce<i32> for NumberProducer2 {
//     /// Produces sequential `i32` values from 0 to 9, sends them to a channel, and simulates work between sends.
//     ///
//     /// # Arguments
//     /// * `tx`: A Sender through which produced `i32` values are sent.
//     fn produce(&self, tx: Sender<i32>) {
//         for i in 0..=9 {
//             println!("Produce_2: {}", i);
//             tx.send(i).unwrap();
//             // Simulating some work
//             thread::sleep(Duration::from_millis(500));
//         }
//     }
// }
// impl Consume<i32> for NumberConsumer2 {
//     /// Consumes `i32` values from a channel and prints them.
//     ///
//     /// # Arguments
//     /// * `rx`: A Receiver from which `i32` values are received.
//     fn consume(&self, rx: Receiver<i32>) {
//         for received in rx.iter() {
//             println!("Consumed_2: {:?}", received);
//         }
//     }
// }

// Sync Main
// fn main() {
//     let pc = ProducerConsumer::new(NumberProducer, NumberConsumer);
//     let _pc_2 = ProducerConsumer::new(NumberProducer2, NumberConsumer2);
//     pc.join_threads();
//     _pc_2.join_threads();
//     // Here, you can do other things in the main thread if needed.
// }
