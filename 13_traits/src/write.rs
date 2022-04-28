use std::fmt;
use std::io::Write;

// 自定义BufBuilder结构体，实现Write trait

struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

// 实现Debuf trait 打印字符串
impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // 把buf添加到BufBuilder尾部
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        // 由于是内存操作，不需要flush
        Ok(())
    }
}

fn main () {
    let mut buf = BufBuilder::new();
    buf.write_all(b"Hello World!").unwrap();
    println!("{:?}", buf);
}