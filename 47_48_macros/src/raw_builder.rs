use anyhow::Result;
use askama::Template;
use proc_macro::{Ident, TokenStream, TokenTree};
use std::collections::VecDeque;

/// 处理jinjia模板的数据结构，在模板中我们使用了name/ builder_name / fields
#[derive(Template)]
#[template(path="builder.j2", escape = "none")]
pub struct BuilderContext {
    name: String,
    builder_name: String,
    fields: Vec<Fd>,
}

/// 描述struct的每个field
#[derive(Debug, Default)]
struct Fd {
    name: String,
    ty: String,
    optional: bool,
}

impl Fd {
    /// name 和field都是通过冒号Punct切分出来的TokenTree切片
    pub fn new(name: &[TokenTree], ty: &[TokenTree]) -> Self {
        // 把类似Ident("Option"), Punct('<'), Ident("String"), Punct('>')的ty
        // 收集成一个String列表，如vec!["Option", "<", "String", ">"]
        let ty = ty
            .iter()
            .map(|v| match v {
                TokenTree::Ident(n) => n.to_string(),
                TokenTree::Punct(p) => p.as_char().to_string(),
                e => panic!("Expect ident, got {:?}", e),
            })
            .collect::<Vec<_>>();
        // 冒号最后一个TokenTree 是field 的名字
        // 比如execuable: String,
        // 注意这里不应该用name[0], 因为有可能是pub executable: String,
        // 甚至带attributes的field,
        // 比如#[builder(hello = world)] pub executable: String
        match name.last() {
            Some(TokenTree::Ident(name)) => {
                // 如果ty第0项是Optoin，name从第二项取到倒数第一项
                // 取完后上面的例子中的ty会变成["String"], option = true
                let (ty, optional) = if ty[0].as_str() == "Option" {
                    (&ty[2..ty.len() - 1], true)
                } else {
                    (&ty[..], false)
                };
                Self {
                    name: name.to_string(),
                    ty: ty.join(""), // 把ty join成字符串
                    optional,
                }
            }
            e => panic!("Expect ident, got {:?}", e),
        }
    }
}

impl BuilderContext {
    /// 从TokenStram中提取信息，构建BuilderContext
    fn new(input: TokenStream) -> Self {
        let (name, input) = split(input);
        let fields = get_struct_fields(input);
        Self {
            builder_name: format!("{}Builder", name),
            name: name.to_string(),
            fields,
        }
    }

    /// 吧模板渲染成字符串代码
    pub fn render(input: TokenStream) -> Result<String> {
        let template = Self::new(input);
        Ok(template.render()?)
    }
}

/// 把TokenStream分出struct的名字和包含fields的TokenStream
fn split(input: TokenStream) -> (Ident, TokenStream) {
    let mut input = input.into_iter().collect::<VecDeque<_>>();
    // 一直往后找，找到struct停下来
    while let Some(item) = input.pop_front() {
        if let TokenTree::Ident(v) = item {
            if v.to_string() == "struct" {
                break;
            }
        }
    }

    // struct后面，应该就是struct name
    let ident;
    if let Some(TokenTree::Ident(v)) = input.pop_front() {
        ident = v;
    } else {
        panic!("Didn't find struct name");
    }

    // struct 后面可能还有若干TokenTree， 我们不管，医鹿找到第一个Group
    let mut group = None;
    for item in input {
        if let TokenTree::Group(g) = item {
            group = Some(g);
            break;
        }
    }

    (ident, group.expect("Didn't find field group").stream())
}

/// 从包含fields的TokenStream中切出来一个个Fd
fn get_struct_fields(input: TokenStream) -> Vec<Fd> {
    let input = input.into_iter().collect::<Vec<_>>();
    input
        .split(|v| match v {
            // 先用 ',' 切出来一个个包含field所有信息的&[TokenTree]
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        })
        .map(|tokens| {
            tokens
                .split(|v| match v {
                    // 再用 ':' 把&[TokenTree] 切成 [&[TokenTree], &[TokenTree]]
                    // 它们分别对应名字和类型
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false
                })
                .collect::<Vec<_>>()
        })
        // 正常情况下，应该得到[&[TokenTree], &[TokenTree]], 对于切出来长度不为2的统统略过
        .filter(|tokens| tokens.len() == 2)
        // 使用Fd::new 创建出每个Fd
        .map(|tokens| Fd::new(tokens[0], &tokens[1]))
        .collect()

}