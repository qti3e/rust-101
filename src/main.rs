struct VecIter<'a, T> {
    v: &'a Vec<T>,
    index: usize,
}

impl<'a, T> VecIter<'a, T> {
    pub fn new<'b: 'a>(v: &'b Vec<T>) -> Self {
        Self { v, index: 0 }
    }
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.v.len() {
            let n = self.index;
            self.index += 1;
            Some(&self.v[n])
        } else {
            None
        }
    }
}

fn main() {
    let numbers = vec![2, 7, 11, 15];
    for n in VecIter::new(&numbers) {
        println!("element: {n}");
    }
}
