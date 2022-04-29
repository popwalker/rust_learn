use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{:?}", v.as_ref());
}

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into());
}

fn main () {
    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();

    print([1,1,1,1]);
    print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);
    print(v4);
    print(v6);

    let lang = Language::Rust;
    print_ref("Hello World!");
    print_ref("Hello World!".to_string());
    print_ref(lang);

}