use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;

enum Message {
	NewJob(Job),
	Terminate,
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>) {
		(*self)()
	}
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

impl ThreadPool {
	pub fn new(size: usize) -> Self {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		Self { 
			workers,
			sender,
		}
	}

	pub fn execute<F>(&self, f: F)
		where
			F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);

		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		println!("Sending terminate message to all workers.");

		for _ in &mut self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		println!("Shuting down all workers.");

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}

		}
	}
}


struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
		let thread = thread::spawn(move || {
			loop {
				let message = receiver.lock().unwrap()
										.recv()
										.unwrap();

				match message {
					Message::NewJob(job) => {
						println!("Worker {} got a job; executing.", id);
						job.call_box();
					},
					Message::Terminate => {
						println!("Worker {} was told to terminate.", id);
						break;
					},
				}

			}
		});

		Self {
			id,
			thread: Some(thread),
		}
	}
}