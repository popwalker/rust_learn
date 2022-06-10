use futures::prelude::*;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll}, io::BufRead,
};
use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncRead, BufReader, Lines},
};

/// LineStream内部使用tokio::io::Lines
#[pin_project]
struct LineStream<R> {
    #[pin]
    lines: Lines<BufReader<R>>
}

impl<R: AsyncRead> LineStream<R> {
    /// 从BufReader创建一个LineStream
    pub fn new(reader: BufReader<R>) -> Self {
        Self {
            lines: reader.lines(),
        }
    }
}

/// 为LineStream实现Stream trait
impl<R: AsyncRead> Stream for LineStream<R> {
    type Item = std::io::Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project()
            .lines
            .poll_next_line(cx)
            .map(Result::transpose)
    }
}
#[tokio::main]
async fn main() -> std::io::Result<()>{
    let file = fs::File::open("Cargo.toml").await?;
    let reader = BufReader::new(file);
    let mut st = LineStream::new(reader);
    while let Some(Ok(line)) = st.next().await {
        println!("Got: {}", line);
    }

    Ok(())
}