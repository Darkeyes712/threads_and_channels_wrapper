use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// A generic struct to manage the communication between a producer and consumer using channels.
pub struct ProducerConsumer<T: Send + 'static> {
    /// Sender end of the channel. Useful for sending more messages in the future.
    tx: Sender<T>,
    /// Handle to the producer thread, allowing for its management after creation.
    producer_handle: Option<thread::JoinHandle<()>>,
    /// Handle to the consumer thread, allowing for its management after creation.
    consumer_handle: Option<thread::JoinHandle<()>>,
}

/// A trait defining the behavior of a producer.
///
/// Types that implement this trait should be able to produce items of type `T`
/// and send them to a given `Sender`.
pub trait Produce<T: Send> {
    fn produce(&self, tx: Sender<T>);
}

/// A trait defining the behavior of a consumer.
///
/// Types that implement this trait should be able to consume items of type `T`
/// from a given `Receiver`.
pub trait Consume<T: Send> {
    fn consume(&self, rx: Receiver<T>);
}

impl<T: Send + 'static> ProducerConsumer<T> {
    /// Waits for both the producer and consumer threads to finish their execution.
    pub fn join_threads(self) {
        if let Some(handle) = self.producer_handle {
            handle.join().unwrap();
        }
        if let Some(handle) = self.consumer_handle {
            handle.join().unwrap();
        }
    }

    /// Creates a new ProducerConsumer setup. Starts threads for both producer and consumer.
    ///
    /// # Arguments
    ///
    /// * `producer` - An object implementing the `Produce` trait.
    /// * `consumer` - An object implementing the `Consume` trait.
    pub fn new(
        producer: impl Produce<T> + Send + 'static,
        consumer: impl Consume<T> + Send + 'static,
    ) -> Self {
        let (tx, rx) = mpsc::channel();

        let producer_tx = tx.clone();
        let producer_handle = thread::spawn(move || {
            producer.produce(producer_tx);
        });

        let consumer_handle = thread::spawn(move || {
            consumer.consume(rx);
        });

        ProducerConsumer {
            tx,
            producer_handle: Some(producer_handle),
            consumer_handle: Some(consumer_handle),
        }
    }
}
