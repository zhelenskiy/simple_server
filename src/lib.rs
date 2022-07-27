use std::sync::{Arc, mpsc, Mutex};
use worker::Worker;
use crate::Message::NewJob;

mod worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    queue: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(threads: usize) -> ThreadPool {
        assert!(threads > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        ThreadPool {
            workers: (0..threads).map(|id| Worker::new(id, receiver.clone())).collect(),
            queue: sender,
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        self.queue.send(NewJob(Box::new(f))).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        repeat(self.workers.len(), || self.queue.send(Message::Terminate).unwrap());

        println!("Shutting down all workers.");
    }
}

#[inline]
fn repeat<F>(n: usize, mut action: F) where F: FnMut() {
    for _ in 0..n {
        action()
    }
}
