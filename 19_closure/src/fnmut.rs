fn main(){
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");

    let mut c = || {
        name.push_str(" tom");
        println!("c:{}", name);
    };

    let mut c1 = move || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    call_once(c);
    call_once(c1);
}

fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_once(c: impl FnOnce()) {
    c();
}