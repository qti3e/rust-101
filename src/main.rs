// owning: T
// mutable borrow: &mut T
// borrow: &T

pub struct VecIterMut<'a, T: 'a> {
    index: usize,
    vec: &'a mut Vec<T>,
}

impl<'a, T: 'a> VecIterMut<'a, T> {
    pub fn new(vec: &'a mut Vec<T>) -> Self {
        Self { index: 0, vec }
    }
}

impl<'a, T: 'a> Iterator for VecIterMut<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;
        // why does this not work?
        // imagine the prev line was commented out.
        Some(&mut self.vec[index]) // &mut 2
    }
}

fn main() {
    let mut vec = vec![2, 7, 11, 15];
    let mut iter = VecIterMut::new(&mut vec);
    let x1 = iter.next().unwrap();
    let x2 = iter.next().unwrap();
    dbg!(x1, x2); // &mut 2, &mut 2 // -> now we have 2 mutable access to the same value
                  // this violates the borrow checker.

    for x in VecIterMut::new(&mut vec) {
        // x == &mut 2
        // x == &mut 2
        *x += 1;
    }

    dbg!(vec);
}
