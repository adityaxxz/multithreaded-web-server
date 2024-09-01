use std::{sync::{mpsc, Mutex, Arc}, thread};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

//`Job` is type alias for trait object that holds the type of closure `execute` expects.using here coz all types of jobs can be passed to `execute` method.
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
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
 
        let mut workers = Vec::with_capacity(size);

        // for loop which populates the threads
        for id in 1..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {workers, sender}
    }

    /// Used channels to send the closure to one of the workers when execute is called.
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap(); // send will pass(returns a Result type) & all our threads will continue to run as long as the pool exists.

    }
}


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker { 
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            let message = receiver
                                                .lock()//to acquire a mutex 
                                                .unwrap()
                                                .recv()//receive job from channel
                                                .unwrap();

            
            match message {
                Message::NewJob(job) => { 
                    println!("Worker {} got a job: executing.",id);
                    job(); 
                }

                Message::Terminate => {
                    println!("Worker {} was told to terminate",id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread:Some(thread),
        }
    }
}

///? SCENARIO EXPLANATION for the usecase of multi-threaded server
/// Imagine that our server got 4 requests at exact same time, we have 4 workers in our thread pool, one of the workers will acquire a lock to the receiver and then pick up a job to execute.

/// As soon as the worker starts executing the job, the receiver will be unlocked then another worker will acquire the lock, look for a job and start executing 2nd request. this will happen till the 3rd and 4th request.

/// When the 5th request comes in, If all the workers are busy executing the other request, Then the 5th request have to wait. The first worker to get done executing their job will acquire a lock to the receiver, pick up the 5th job to execute and begin executing the job. At this point our multi threaded server is complete.


//* Implementing Drop trait on ThreadPool */ When the pool is dropped, letting threads finish their current request before shutting down the server.

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers...");

        // Looping through all workers & sending termination
        // Workers pick up terminate message when they are done with the job, And we know that if they pick up terminate message then the worker will terminate. As long as the no. of terminate messages are received is equivalent to the no. of workers, then eventually each worker will pick up terminate message & exit.
        for _ in &self.workers { 
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();     
            }
        }
        // calling the join() method ensures that each worker has enough time to receive and process the terminate message.
    }
}


