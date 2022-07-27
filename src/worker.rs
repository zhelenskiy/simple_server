use std::thread;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use crate::{Message};

pub(crate) struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock().expect("Other thread panicked").recv().unwrap() {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "worker-{}", self.id)
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("Shutting down worker {}", self.id);
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
