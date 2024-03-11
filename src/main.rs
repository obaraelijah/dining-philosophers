use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
use parking_lot::Mutex;

struct Fork;

struct Philosopher {
    name: String,
    indent: usize,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn print(&self, s: &str) {
        let indent = " ".repeat(self.indent);
        println!("{}{}: {}", indent, &self.name, s);
    }

    fn think(&self) {
        self.print("is thinking...");
        random_sleep(10, 30);
        if let Err(err) = self.thoughts.send(format!("Eureka! {} has a new idea!", &self.name)) {
            eprintln!("Failed to send thought: {:?}", err);
        }
    }
    

    fn eat(&self) {
        let max_retries = 3;
        let mut retries = 0;

        while retries < max_retries {
            let _locks = loop {
                self.print("trying to get both forks...");
                let r1 = self.left_fork.try_lock_for(Duration::from_millis(100));
                if r1.is_some() {
                    self.print("got left fork!");
                }
                random_sleep(1, 100);
                let r2 = self.right_fork.try_lock_for(Duration::from_millis(100));
                if r2.is_some() {
                    self.print("got right fork!");
                }
                if r1.is_some() && r2.is_some() {
                    self.print("got both forks!");
                    break (r1.unwrap(), r2.unwrap());
                } else {
                    if r1.is_some() {
                        self.print("can't get right fork, returning left fork");
                        drop(r1);
                        thread::sleep(Duration::from_millis(10));
                    }
                    if r2.is_some() {
                        self.print("can't get left fork, returning right fork");
                        drop(r2);
                        thread::sleep(Duration::from_millis(10));
                    }
                }
                random_sleep(1, 1000);
            };

            self.print("is eating...");
            random_sleep(10, 30);
            self.print("is returning forks...");

            break;
        }

        if retries == max_retries {
            self.print("giving up on eating for now...");
        }
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    let forks: Vec<_> = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect();

        let (tx, rx) = mpsc::channel();    
    let philosophers: Vec<_> = PHILOSOPHERS.iter()
        .enumerate()
        .map(|(i, name)| {
            let left_fork = forks[i].clone();
            let right_fork = forks[(i + 1) % PHILOSOPHERS.len()].clone();
            Philosopher {
                name: name.to_string(),
                indent: i * 30,
                left_fork,
                right_fork,
                thoughts: tx.clone(),
            }
        })
        .collect();

    let handles: Vec<_> = philosophers.into_iter()
        .map(|p| {
            thread::spawn(move || {
                for _ in 0..100 {
                    p.think();
                    p.eat();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    tx.send("Done".to_string()).unwrap();

    println!("Number of forks: {}", forks.len() - 1);

    if std::env::args().any(|arg| arg == "--thoughts") {
        // Output their thoughts
        use std::io::Write;
        loop {
            let thought = rx.recv().unwrap();
            if thought == "Done" {
                break;
            }
            std::io::stdout().write_all(thought.as_bytes()).unwrap();
        }
    }
}

fn random_sleep(a: u64, b: u64) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(a..b);
    thread::sleep(Duration::from_millis(sleep_time));
}