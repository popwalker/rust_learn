use actix::prelude::*;
use anyhow::{Result, Ok};


// actor可以处理消息
#[derive(Debug, Message, Clone, PartialEq)]
#[rtype(result = "OutMsg")]
enum InMsg {
    Add((usize, usize)),
    Concat((String, String)),
}
#[derive(Debug, MessageResponse, Clone, PartialEq)]
enum OutMsg {
    Num(usize),
    Str(String),
}

struct DummyActor;

impl Actor for DummyActor {
    type Context = Context<Self>;
}

// 实现处理InMsg的Handler trait
impl Handler<InMsg> for DummyActor {
    // 返回的消息
    type Result = OutMsg;

    fn handle(&mut self, msg: InMsg, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InMsg::Add((a, b)) => OutMsg::Num(a + b),
            InMsg::Concat((mut s1, s2)) => {
                s1.push_str(&s2);
                OutMsg::Str(s1)
            }
        }
    }
}

#[actix::main]
async fn main() -> Result<()> {
    let addr = DummyActor.start();
    let res = addr.send(InMsg::Add((21, 21))).await?;
    let res1 = addr
        .send(InMsg::Concat(("hello, ".into(), "world".into())))
        .await?;
    println!("res: {:?}, res1: {:?}", res, res1);
    Ok(())
}