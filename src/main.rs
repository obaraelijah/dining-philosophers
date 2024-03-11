use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

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
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        let _locks = loop {
            self.print("trying to get both forks...");
            let r1 = self.left_fork.try_lock();
            if r1.is_ok() {
                self.print("got left fork!");
            }
            random_sleep(1, 100);
            let r2 = self.right_fork.try_lock();
            if r2.is_ok() {
                self.print("got right fork!");
            }
            if r1.is_ok()  && r2.is_ok() {
                self.print("got both forks!");
                break (r1.unwrap(), r2.unwrap());
            } else {
                if r1.is_ok() {
                    self.print("can't get right fork, returning left fork");
                }
                if r2.is_ok() {
                    self.print("can't get left fork, returning right fork");
                }
                // releasing  locks that were acquired
                drop(r1);
                drop(r2);
            }
            random_sleep(1, 1000)
        };
        self.print("is eating...");
        random_sleep(10, 30);
        self.print("is returning forks...")
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];


fn main() {
    // Create forks

    // Create philosophers

    // Make each of them think and eat 100 times

    // Output their thoughts
}

// Utility function to sleep for a random amount of time
fn random_sleep(a: u64, b: u64) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(a..=b);
    thread::sleep(Duration::from_millis(sleep_time));
}