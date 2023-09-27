use std::{thread, sync::{mpsc, Arc, Mutex}};
use std::sync::mpsc::{Receiver, Sender};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
    size: usize
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new() -> ThreadPool {
        let (sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();

        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(10);

        for id in 0..10 {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
            size: 10
        }
    }

    /// Creates a `ThreadPool` with amount of threads used.
    /// Panics if size is 0
    fn new_with_size(size: usize) -> ThreadPool {
        assert!(size > 0, "Panicked at `new_with_size`. Size can't be zero");

        let (sender, receiver) = mpsc::channel();

        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
            size
        }
    }

    fn execute<F>(&self, fun: F)
    where
        F: FnOnce() + Send + 'static 
    {
        let job = Box::new(fun);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            job();
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}