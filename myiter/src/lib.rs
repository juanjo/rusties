struct MyVecIterator<'a, T> {
    vec: &'a Vec<T>,
    current_index: usize,
}

pub struct MyMap<I, F> {
    iter: I,
    func: F,
}

impl<B, I: MyIterator, F: FnMut(I::Item) -> B> MyIterator for MyMap<I, F> {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        Some((self.func)(item))
    }
}

pub trait MyIterator: Sized {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    fn map<F, B>(self, f: F) -> MyMap<Self, F>
    where
        F: FnMut(Self::Item) -> B,
    {
        MyMap {
            iter: self,
            func: f,
        }
    }
}

impl<'a, T> MyIterator for MyVecIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_index;
        let current_item = self.vec.get(current);
        self.current_index += 1;

        return current_item;
    }
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let my_vec = vec![1, 2, 3];

        let mut my_iterator = MyVecIterator {
            vec: &my_vec,
            current_index: 0,
        };

        let mut iter = my_iterator.map(|x| x * 2);

        while let Some(item) = iter.next() {
            println!("{}", item)
        }
    }
}
