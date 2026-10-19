mod echarts_jni;

pub use echarts_jni::render;
pub use echarts_jni::save;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EchartsError {
    #[error("文件格式错误")]
    Format,
    #[error("请提交issue: {0}")]
    Unknown(String),
    #[error("JNI错误")]
    Jni(#[from] jni::errors::Error),
}
