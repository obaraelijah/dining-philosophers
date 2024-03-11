# Dining Philosophers

I played around with the "Dining Philosophers" concurrency exercise from the Comprehensive Rust course.

## Key Observations

There are two key observations:
1. **Race Conditions:** Introducing a delay between a philosopher attempting to grab the left and right forks can lead to deadlock situations, making it easier to test different solutions..
2. **Dependency on Both Forks:** The fundamental issue is that each philosopher needs both forks to eat, but the forks are independently shared among philosophers.

Based on the insight above, the basic solution I came up with  involves philosophers attempting to pick up both forks simultaneously. If a philosopher cannot pick up both forks, they put back any forks picked up and wait before trying again later. This approach avoids the deadlock scenario where a philosopher holds one fork while waiting for another.

```rust
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
```
## Running the Program
```
cargo run

```
### Further Improvements
While the current solution prevents deadlocks, it introduces latency into the system due to the wait delay for philosophers to "try again." In the future, it would be interesting to experiment with holding onto one of the two forks for a short period of time, in case the other shows up during that window. If the timeout is reached, the picked-up fork goes back to the table with a wait, as above. This approach could potentially allow philosophers to eat faster by waiting while holding one of the locks.