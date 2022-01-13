use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

// Engine trait: 未来可以添加更多的engine，主流程只需要替换engine
pub trait Engine {
    // 对engine按照specs进行一系列有序处理
    fn apply(&mut self, specs: &[Spec]);
    // 从engine中生成目标图片，这里是self，不是self的引用
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}