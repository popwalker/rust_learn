fn main(){
    let name = "tom".to_string();
    std::thread::spawn(move || {
        println!("hello {}", name);
    });
}