use std::{
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pub init_num: usize,
    pub works: Vec<Worker>,
    pub jsender: Option<mpsc::Sender<Job>>,
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(init_num: usize) -> ThreadPool {
        assert!(init_num > 0);

        let (jsender, jreceiver) = mpsc::channel();
        let jreceiver = Arc::new(Mutex::new(jreceiver));

        let mut workers = Vec::with_capacity(init_num);
        for i in 0..init_num {
            workers.push(Worker::new(i, Arc::clone(&jreceiver)));
        }

        ThreadPool {
            init_num,
            works: workers,
            jsender: Some(jsender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.jsender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.jsender.take());

        for worker in &mut self.works {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
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

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn it_works() {
        let workpool = ThreadPool::new(3);

        for num in 0..=10 {
            workpool.execute(move || {
                println!("num {}", num);
                sleep(Duration::from_secs(5));
            });
        }
    }
}
