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
    let mut forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    // clone the first fork to the last position is done to ensure that the last philosopher can access the first fork. 
    forks.push(forks[0].clone());

    let (tx, rx) = mpsc::channel();
    // Create philosophers
    let philosophers: Vec<_> = PHILOSOPHERS
        .iter()
        .zip(forks.windows(2))
        .enumerate()
        .map(|(i, (name, pair))| {
            let philosopher = Philosopher {
                name: name.to_string(),
                indent: i * 30,
                left_fork: pair[0].clone(),
                right_fork: pair[1].clone(),
                thoughts: tx.clone(),
            };
            philosopher
        })
        .collect();

    // Make each of them think and eat 100 times
    thread::scope(|s| {
        for p in philosophers {
            s.spawn(move || {
                for _ in 0..100 {
                    // make the philosopher think
                    p.think();
                    // make the philosopher eat
                    p.eat();
                }
            });
        }
    });
    tx.send("Done".to_string()).unwrap();

    println!("Number of forks: {}", forks.len() - 1);
    // Output their thoughts
}

// Utility function to sleep for a random amount of time
fn random_sleep(a: u64, b: u64) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(a..=b);
    thread::sleep(Duration::from_millis(sleep_time));
}