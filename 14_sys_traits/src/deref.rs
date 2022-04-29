use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main(){
    let mut buf = Buffer::new([1,3,2,4]);
    buf.sort();
    println!("buf: {:?}", buf);

    let mut v: Vec<i32> = vec![1,3,2,4];
    v.sort();
    println!("v: {:?}", v);
}