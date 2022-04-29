use std::fmt::{self, Display};

// struct 可以derive Default，但我们需要所有字段都实现Default
#[derive(Clone, Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

// enum不能derive Default
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    pub fn new(name: &str) -> Self {
        // 用..Default::default()为剩余字段使用缺省值
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} developer",
            self.name, self.age, self.lang,
        )
    }
}

fn main (){
    let dev1 = Developer::default();
    let dev2: Developer = Default::default();
    let dev3 = Developer::new("Tyr");
    println!("dev1:{}\ndev2:{}\ndev3:{}", dev1, dev2, dev3);
}