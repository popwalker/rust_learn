use anyhow::Result;
use futures::prelude::*;
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    let file_sink = writer(File::create("hello.tmp").await?);
    futures::pin_mut!(file_sink);
    if let Err(_) = file_sink.send("hello\\n").await {
        println!("Error on send");
    }

    if let Err(_) = file_sink.send("world!\\n").await {
        println!("Error on send");
    }

    Ok(())
}


// 使用unfold生成一个Sink数据结构
fn writer<'a>(file: File) -> impl Sink<&'a str> {
    sink::unfold(file, |mut file, line: &'a str| async move {
        file.write_all(line.as_bytes()).await?;
        eprint!("Received:{}", line);
        Ok::<_, std::io::Error>(file)
    })
}