#[derive(Debug)]
struct MyStruct {
    contents: [u8],
}

impl MyStruct {
    pub fn new(i: impl IntoIterator<Item = u8>) -> Box<MyStruct> {
        let x = i.into_iter().collect::<Vec<_>>().into_boxed_slice();
        unsafe { Box::from_raw(Box::into_raw(x) as *mut MyStruct) }
    }

    pub fn inc(&mut self) {
        self.contents.iter_mut().for_each(|f| *f += 1);
    }
}

impl Default for Box<MyStruct> {
    fn default() -> Self {
        let x: Box<[u8]> = vec![].into_boxed_slice();
        unsafe { Box::from_raw(Box::into_raw(x) as *mut MyStruct) }
    }
}

impl From<Box<MyStruct>> for Box<[u8]> {
    fn from(value: Box<MyStruct>) -> Self {
        unsafe { Box::from_raw(Box::into_raw(value) as *mut [u8]) }
    }
}

fn main() {
    let mut x = MyStruct::new([1, 5, 4]);
    println!("{:?}", x);
    x.inc();
    println!("{:?}", x);
    println!("{:?}", Into::<Box<[u8]>>::into(x).into_vec());
}
