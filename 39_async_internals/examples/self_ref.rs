
#[derive(Debug)]
struct SelfReference {
    name: String,
    name_ptr: *const String,
}


impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference { name: name.into(), name_ptr: std::ptr::null() }
    }

    pub fn init(&mut self) {
        self.name_ptr = &self.name as *const String;
    }

    pub fn print_name(&self) {
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
    let data = move_creates_issue();
    println!("data: {:?}", data);

    // data.print_name();
    println!("\\n");
    mem_swap_creates_issue();
}

fn mem_swap_creates_issue() {
    let mut data1 = SelfReference::new("Tyr");
    data1.init();

    let mut data2 = SelfReference::new("lindsey");
    data2.init();

    data1.print_name();
    data2.print_name();

    std::mem::swap(&mut data1, &mut data2);
    data1.print_name();
    data2.print_name();
}

fn move_creates_issue() -> SelfReference {
    let mut data = SelfReference::new("Tyr");
    data.init();

    // 不move， 一切正常
    data.print_name();

    let data = move_it(data);

    data.print_name();
    data
}

fn move_it(data: SelfReference) -> SelfReference {
    data
}