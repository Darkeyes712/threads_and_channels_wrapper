use async_trait::async_trait;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task;

/// Represents an asynchronous producer-consumer structure.
///
/// This structure manages both producer and consumer asynchronous tasks using Tokio.
/// It also provides a method to wait for both tasks to complete their execution.
pub struct AsyncProducerConsumer<T: Send + 'static> {
    /// Sender end of the channel. Useful for sending more messages in the future.
    tx: Sender<T>,
    /// Handle to the producer task, allowing for its management after creation.
    producer_handle: Option<task::JoinHandle<()>>,
    /// Handle to the consumer task, allowing for its management after creation.
    consumer_handle: Option<task::JoinHandle<()>>,
}

/// An asynchronous trait that describes the behavior of a producer.
///
/// The trait defines an asynchronous method `produce` which accepts a Sender
/// and is responsible for producing items and sending them through the channel.
#[async_trait]
pub trait AsyncProduce<T: Send> {
    /// Produce items asynchronously and send them through the provided sender.
    ///
    /// # Arguments
    ///
    /// * `tx` - The sender through which items are to be sent.
    async fn produce(&self, tx: Sender<T>);
}

/// An asynchronous trait that describes the behavior of a consumer.
///
/// The trait defines an asynchronous method `consume` which accepts a Receiver
/// and is responsible for consuming items received through the channel.
#[async_trait]
pub trait AsyncConsume<T: Send> {
    /// Consume items asynchronously received through the provided receiver.
    ///
    /// # Arguments
    ///
    /// * `rx` - The receiver through which items are to be received.
    async fn consume(&self, rx: Receiver<T>);
}

impl<T: Send + 'static> AsyncProducerConsumer<T> {
    /// Waits for both the producer and consumer tasks to finish their execution.
    ///
    /// This method will block until both tasks have completed. If either task panics,
    /// this method will propagate the panic.
    pub async fn join_tasks(self) {
        if let Some(handle) = self.producer_handle {
            handle.await.unwrap();
        }
        if let Some(handle) = self.consumer_handle {
            handle.await.unwrap();
        }
    }

    /// Creates a new instance of `AsyncProducerConsumer` initializing and spawning
    /// the producer and consumer tasks.
    ///
    /// # Arguments
    ///
    /// * `producer` - The producer implementing the `AsyncProduce` trait.
    /// * `consumer` - The consumer implementing the `AsyncConsume` trait.
    ///
    /// # Returns
    ///
    /// A new instance of `AsyncProducerConsumer`.
    pub fn new(
        producer: impl AsyncProduce<T> + Send + 'static + std::marker::Sync,
        consumer: impl AsyncConsume<T> + Send + 'static + std::marker::Sync,
    ) -> Self {
        let (tx, rx) = mpsc::channel(500); // Using a buffer size of 100 for simplicity. Adjust as needed.

        let producer_tx = tx.clone();
        let producer_handle = task::spawn(async move {
            producer.produce(producer_tx).await;
        });

        let consumer_handle = task::spawn(async move {
            consumer.consume(rx).await;
        });

        AsyncProducerConsumer {
            tx,
            producer_handle: Some(producer_handle),
            consumer_handle: Some(consumer_handle),
        }
    }
}
