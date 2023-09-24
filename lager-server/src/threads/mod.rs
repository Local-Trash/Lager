use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
    size: usize
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "The size can't be 0");

        let (sender, receiver) = mpsc::channel();

        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));

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

impl std::ops::Drop for ThreadPool {
    fn drop(&mut self) {
        std::mem::drop(self.sender.take());

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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
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