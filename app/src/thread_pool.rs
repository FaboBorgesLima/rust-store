use std::{
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::new();

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("shutting down thread pool");

        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutting down worker thread {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => job(),
                Err(_) => {
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
