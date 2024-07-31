// owning: T
// mutable borrow: &mut T
// borrow: &T

pub struct VecIterMut<'a, T: 'a> {
    // &'mut [T], &'mut [T...]
    slice: &'a mut [T],
}

impl<'a, T: 'a> VecIterMut<'a, T> {
    pub fn new(vec: &'a mut Vec<T>) -> Self {
        Self {
            slice: vec.as_mut_slice(),
        }
    }
}

impl<'a, T: 'a> Iterator for VecIterMut<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }

        let old_slice = std::mem::replace(&mut self.slice, &mut []);
        let (left, right) = old_slice.split_at_mut(1);

        self.slice = right;

        Some(&mut left[0])
    }
}

fn main() {
    let mut vec = vec![2, 7, 11, 15];
    for x in VecIterMut::new(&mut vec) {
        *x += 1;
    }
    dbg!(vec);
}
