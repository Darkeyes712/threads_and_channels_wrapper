use std::sync::mpsc::{self, Receiver, Sender}; // <-- Make sure Receiver is imported here.
use std::thread;

pub struct ProducerConsumer<T: Send + 'static> {
    tx: Sender<T>, // This will be usefull later!
    producer_handle: Option<thread::JoinHandle<()>>,
    consumer_handle: Option<thread::JoinHandle<()>>,
}

pub trait Produce<T: Send> {
    fn produce(&self, tx: Sender<T>);
}

pub trait Consume<T: Send> {
    fn consume(&self, rx: Receiver<T>);
}

impl<T: Send + 'static> ProducerConsumer<T> {
    pub fn join_threads(self) {
        if let Some(handle) = self.producer_handle {
            handle.join().unwrap();
        }
        if let Some(handle) = self.consumer_handle {
            handle.join().unwrap();
        }
    }
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
