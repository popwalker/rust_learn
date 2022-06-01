use std::thread;
use anyhow::Result;
use blake3::Hasher;
use futures::{SinkExt, StreamExt};
use rayon::prelude::*;
use tokio::{
    net::TcpListener,
    sync::{mpsc,oneshot},
};
use tokio_util::codec::{Framed, LinesCodec};


pub const PREFIX_ZERO: &[u8] = &[0,0,0];

#[tokio::main]
async fn main() -> Result<()>{
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);

    // 创建tokio task 和thread之间的channel
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    // 使用thread处理计算密集型任务
    thread::spawn(move || {
        // 读取从tokio task过来的msg， 注意这里用的是blocking_recv，而非await
        while let Some((line, reply)) = receiver.blocking_recv() {
            // 计算pow
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("hash:{}, once:{}", hash, nonce),
                None => "Not found".to_string(),
            };

            // 吧计算结果从onshot channel里发回
            if let Err(e) = reply.send(result) {
                println!("Failed to send: {}", e);
            }
        }
    });

    // 使用tokio task处理IO密集型任务
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        let sender1 = sender.clone();
        tokio::spawn(async move {
            // 使用LinesCodec把Tcp数据切成一行行字符串处理
            let framed = Framed::new(stream, LinesCodec::new());
            let (mut w, mut r) = framed.split();
            for line in r.next().await {
                let (reply, reply_receiver) = oneshot::channel();
                sender1.send((line?, reply))?;

                // 接受pow计算完成后的hash和nonce
                if let Ok(v) = reply_receiver.await {
                    w.send(format!("Pow caculated:{}", v)).await?;
                }
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}

// 使用rayon并发计算u32空间下所有nonce，直到找到有头N个0的哈希
pub fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}


//计算携带nonce后的哈希
fn blake3_hash(mut hasher: blake3::Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}

// 计算数据的哈希
fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}