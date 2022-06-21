#[macro_export]
macro_rules! my_vec {
    // 没有带任何参数的my_vec， 我们创建一个空的vec
    () => {
        std::vec::Vec::new()
    };

    // 处理my_vec![1,2,3,4]
    ($($el:expr), *) => ({
        let mut v = std::vec::Vec::new();
        $(v.push($el);)*
        v
    });

    // 处理my_vec![0; 10];
    ($el:expr; $n:expr) =>  {
        std::vec::from_elem($el, $n)
    }
}

// #[macro_export]
// macro_rules! try_with {
//     ($ctx:ident, $exp:expr) => {
//         match $exp {
//             Ok(v) => v,
//             Err(e) => {
//                 return pipeline::PlugResult::Err {
//                     ctx: $ctx,
//                     err: pipeline::pipelineError::Internal(e.to_string())
//                 }
//             }
//         }
//     };
// }


fn main() {
    let mut v = my_vec![];
    v.push(1);
    // 调用时可以使用[],(),{}
    let _v = my_vec!(1,2,3,4);
    let _v = my_vec![1,2,3,4];
    let v = my_vec!{1,2,3,4};
    println!("{:?}", v);
    println!("{:?}", v);
    let v = my_vec![1;10];
    println!("{:?}",v);
}

