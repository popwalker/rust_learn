use std::fs;


fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Convertint html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes());
    println!("converted markdown has been saved in {}", output);
}
