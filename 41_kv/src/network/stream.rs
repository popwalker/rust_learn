use bytes::BytesMut;
use futures::{ready, Sink, Stream, FutureExt};
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite};
use crate::{FrameCoder, KvError, read_frame};

/// 处理KV server prost frame的stream
pub struct ProstStream<S, In, Out> where {
    // inner stream
    stream: S,
    // 写缓存
    wbuf: BytesMut,
    // 写入了多少字节
    written: usize,
    // 读缓存
    rbuf: BytesMut,

    // 类型占位符
    _in: PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<S, In, Out> ProstStream<S, In, Out> 
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    // 创建一个ProstStream
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            written: 0,
            wbuf: BytesMut::new(),
            rbuf: BytesMut::new(),
            _in: PhantomData::default(),
            _out: PhantomData::default(),
        }
    }
}

// 一般来说，如果我们的stream是Unpin，最好实现一下
impl<S, Req, Res> Unpin for ProstStream<S, Req, Res> where S: Unpin {}

impl<S, In, Out> Stream for ProstStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Unpin+ Send,
    In: Unpin + Send + FrameCoder,
    Out: Unpin + Send,
{
    /// 当调用next()时，得到Result<In, KvError>
    type Item = Result<In, KvError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 上一次调用结束后，rbuf应该为空
        assert!(self.rbuf.len() == 0);

        // 从rbuf中分离出rest(摆脱对self的引用)
        let mut rest = self.rbuf.split_off(0);

        // 使用read_frame来获取数据
        let fut = read_frame(&mut self.stream, &mut rest);
        ready!(Box::pin(fut).poll_unpin(cx))?;
        
        // 拿到一个frame的数据，把buffer合并回去
        self.rbuf.unsplit(rest);

        //调用decode_frame获取解析后的数据
        Poll::Ready(Some(In::decode_frame(&mut self.rbuf)))
    }
}

// 当调用send()时，会把Out发出去
impl<S, In, Out> Sink<Out> for ProstStream<S, In, Out> 
where
    S: AsyncRead + AsyncWrite + Unpin,
    In: Unpin + Send,
    Out: Unpin + Send + FrameCoder,
{
    /// 如果发送出错，会返回KvError
    type Error = KvError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: Out) -> Result<(), Self::Error> {
        let this = self.get_mut();
        item.encode_frame(&mut this.wbuf)?;
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.get_mut();

        // 循环写入stream中
        while this.written != this.wbuf.len() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, &this.wbuf[this.written..]))?;
            this.written += n;
        }

        // 清除wbuf
        this.wbuf.clear();
        this.written = 0;

        // 调用stream的poll_flush确保写入
        ready!(Pin::new(&mut this.stream).poll_flush(cx)?);
        Poll::Ready(Ok(()))
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // 调用stream的poll_flush确保写入
        ready!(self.as_mut().poll_flush(cx))?;
        // 调用stream的poll_shutdown确保stream关闭
        ready!(Pin::new(&mut self.stream).poll_shutdown(cx))?;
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils::DummyStream, CommandRequest};
    use anyhow::Result;
    use futures::prelude::*;

    #[tokio::test]
    async fn prost_stream_should_work() -> Result<()>{
        let buf = BytesMut::new();
        let stream = DummyStream { buf };
        let mut stream = ProstStream::<_, CommandRequest, CommandRequest>::new(stream);
        let cmd = CommandRequest::new_hdel("t1", "k1");
        stream.send(cmd.clone()).await?;
        if let Some(Ok(s)) = stream.next().await {
            assert_eq!(s, cmd);
        } else {
            assert!(false);
        }
        Ok(())
    }
}