use std::{
    borrow::Borrow,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle}, time::Duration,
};

type Job = Box<dyn FnOnce() + 'static + Sync + Send>;
struct ThreadPool {
    workders: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: i32) -> Self {
        let mut workders = vec![];
        let (sender, receiver): (Sender<Job>, Receiver<Job>) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            workders.push(Worker::new(receiver.clone(), i));
        }
        ThreadPool {
            workders,
            sender: Some(sender),
        }
    }
    pub fn exec<T>(&self, f: T)
    where
        T: FnOnce() + 'static + Sync + Send,
    {
        self.sender
            .as_ref()
            .unwrap()
            .send(Box::new(f))
            .expect("error in exec");
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // drop(self.sender);
        let sender = self.sender.take().unwrap();
        drop(sender);
        for i in self.workders.iter_mut() {
            let thread = i.thread.take().unwrap();
            println!("shutting {:#?}", i.name);
            thread.join().unwrap();
            println!("shutted {:#?}", i.name);
        }
    }
}
struct Worker {
    thread: Option<JoinHandle<()>>,
    name: i32,
}
impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Job>>>, name: i32) -> Self {
        let th = thread::spawn(move || loop {
            let lock = receiver.lock().unwrap().recv();
            if let Ok(job) = lock {

                println!("{:#?} is working!", name);
                thread::sleep(Duration::from_secs(1));
                job()
            } else {
                break;
            }
        });
        println!("worker <{:#?}> is running", name);
        Worker {
            thread: Some(th),
            name,
        }
    }
}
#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    #[test]
    fn it_works() {
        let pool = crate::ThreadPool::new(4);
        for i in 0..10 {
            pool.exec(move || {
                println!("hi {:#?}", i);
            })
        }
        //    thread::sleep(Duration::from_secs(3));
    }
}
