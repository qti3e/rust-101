struct FibIter {
    prev: u64,
    next: u64,
}

impl FibIter {
    pub fn new() -> Self {
        // 1 1 2 3 5
        FibIter { prev: 0, next: 1 }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev + self.next;
        self.prev = self.next;
        self.next = next;
        Some(self.prev)
    }
}

fn main() {
    for (i, n) in FibIter::new().take(5).enumerate() {
        println!("Fib({}) = {n}", i + 1);
    }
    println!("exited...");
}
