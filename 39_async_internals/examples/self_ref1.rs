use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化后指向name
    name_ptr: *const String,
    //PhantomPinned占位符
    _marker: PhantomPinned,
}


impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference { 
            name: name.into(), 
            name_ptr: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let name_ptr = &self.name as *const String;
        // SAFETY： 这里并不会把任何数据从&mut SelfReference中移走
        let this = unsafe { self.get_unchecked_mut() };
        this.name_ptr = name_ptr;
    }

    pub fn print_name(self: Pin<&Self>) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ptr,
            self.name,
            unsafe{ &*self.name_ptr },
        );
    }
}

fn main() {
    move_creates_issue();
}

fn move_creates_issue() {
    let mut data = SelfReference::new("Tyr");
    let mut data = unsafe{ Pin::new_unchecked(&mut data) };
    SelfReference::init(data.as_mut());

    // 不move， 一切正常
    data.as_ref().print_name();

    move_pinned(data.as_mut());
    println!("{:?} ({:?})", data, &data);
}

fn move_pinned(data: Pin<&mut SelfReference>) {
    println!("{:?} ({:?})", data, &data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) -> SelfReference {
    data
}