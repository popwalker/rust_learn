use anyhow::Result;
use std:: {
    collections::VecDeque,
    sync::{atomic::AtomicUsize, Arc, Condvar, Mutex},
};

// 发送者
pub struct Sender<T> {
    shared: Arc<Shared<T>>
}

// 接收者
pub struct Receiver<T> {
    shared: Arc<Shared<T>>
}

/// 发送者和接收者之间共享一个VecDeque, 用Mutex互斥，用Condvar通知
/// 同时，记录有多少个sender和receiver
pub struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    avaiable: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<()> {
        todo!()
    }

    pub fn total_receivers(&self) -> usize {
        todo!()
    } 

    pub fn total_queued_items(&self) -> usize {
        todo!()
    }

}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        todo!()
    }

    pub fn total_senders(&self) -> usize {
        todo!()
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// 克隆 sender
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

/// Drop sender
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        todo!()
    }
}

impl<T> Drop for Receiver<T>  {
    fn drop(&mut self) {
        todo!()
    }
}

/// 创建一个unbounded channel
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};
    use super::*;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world!".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world!");
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });
        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        let mut result = [r.recv().unwrap(),r.recv().unwrap(),r.recv().unwrap()];
        result.sort();
        assert_eq!(result, [1,2,3]);
    }

    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();

        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读取到数据，确保它喝发送的数据一致
                assert_eq!(idx, i);
            }
            // 读取不到，应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        // 1ms足够让生产者发完100个 消息，消费者消费完100个消息并阻塞
        thread::sleep(Duration::from_millis(1));

        // 再次发送数据唤醒消费者
        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        // 留点时间让receiver处理
        thread::sleep(Duration::from_millis(1));

        // 如果receiver被正常唤醒处理，那么队列里的数据会被读取完
        assert_eq!(s1.total_queued_items(), 0);
    }

    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        // sender即用即抛
        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
                // sender在此被丢弃
            })
            .join()
            .unwrap();
        }

        // 虽然没有sender了，接收者依然可以接受已经在队列里的数据
        for _ in 0..total {
            r.recv().unwrap();
        }

        // 然而，读取更多数据时会出错
        assert!(r.recv().is_err());
    }

    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };
        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }
}