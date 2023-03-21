use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq)]
pub struct MyStructSlice {
    contents: [u8],
}

impl MyStructSlice {
    pub fn inc(&mut self) {
        self.contents.iter_mut().for_each(|f| *f += 1);
    }
}

impl From<Box<MyStructSlice>> for Box<[u8]> {
    fn from(value: Box<MyStructSlice>) -> Self {
        unsafe { Box::from_raw(Box::into_raw(value) as *mut [u8]) }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MyStruct {
    contents: Vec<u8>,
}

impl MyStruct {
    pub fn new(items: impl IntoIterator<Item = u8>) -> Self {
        Self {
            contents: items.into_iter().collect(),
        }
    }

    pub fn into_boxed_struct(self) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(self.contents.into_boxed_slice()) as *mut Self) }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.contents
    }
}

impl Deref for MyStruct {
    type Target = MyStructSlice;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(&self.contents as &[u8]) }
    }
}

impl DerefMut for MyStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(&mut self.contents as &mut [u8]) }
    }
}

impl From<MyStruct> for Vec<u8> {
    fn from(value: MyStruct) -> Self {
        value.into_inner()
    }
}

fn main() {
    let mut x = MyStruct::new([1, 5, 4]);
    println!("{:?}", x);
    x.inc();
    println!("{:?}", x);
}
