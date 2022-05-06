pub trait IteratorExt: Iterator {
    fn window_count(self, count: u32) -> WindowCount<Self> where Self: Sized {
        WindowCount {iter: self, count}
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator{}

pub struct WindowCount<I> {
    pub(crate) iter: I,
    count: u32,
}

impl<I: Iterator> Iterator for WindowCount<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = (0..self.count)
            .filter_map(|_| self.iter.next())
            .collect::<Vec<_>>();
        
        if data.is_empty() {
            None
        } else {
            Some(data)
        }
    }
}

fn main() {
    let data = vec![1,2,3,4,5];
    let result = data.iter().window_count(2).collect::<Vec<Vec<_>>>();
    println!("{:?}", result);
}