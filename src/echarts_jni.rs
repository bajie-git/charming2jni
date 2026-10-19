use super::EchartsError;
use charming::{Chart, ImageFormat, ImageRenderer};
use jni::objects::*;
use jni::sys::jint;
use jni::{EnvUnowned, jni_mangle};
use std::path::Path;

/// 跟进路径和原始json渲染图表
// #[unsafe(no_mangle)]
#[jni_mangle("top.magicpotato.Echarts")]
pub fn save<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    width: jint,
    height: jint,
    path: JString<'local>,
    data: JString<'local>,
) {
    let outcome = unowned_env.with_env(|_env| -> Result<_, EchartsError> {
        let data: String = data.to_string();
        let path: String = path.to_string();
        let mut renderer = ImageRenderer::new(width as u32, height as u32);

        // 反序列化为 Chart
        let chart = serde_json::from_str::<Chart>(&data)
            .map_err(|e| EchartsError::Unknown(e.to_string()))?;

        let path = Path::new(&path);
        if path.extension().expect("无法获取文件扩展名") == "svg" {
            renderer
                .save(&chart, path)
                .map_err(|e| EchartsError::Unknown(e.to_string()))?;
        } else {
            let extension = parse_extension_by_path(path)?;
            renderer
                .save_format(extension, &chart, path)
                .map_err(|e| EchartsError::Unknown(e.to_string()))?;
        }

        Ok(())
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

/// 跟进json渲染数据  返回byte数组
// #[unsafe(no_mangle)]
#[jni_mangle("top.magicpotato.Echarts")]
pub fn render<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    width: jint,
    height: jint,
    extension: JString<'local>,
    data: JString<'local>,
) -> JByteArray<'local> {
    let outcome = unowned_env.with_env(|env| -> Result<_, EchartsError> {
        let data: String = data.to_string();
        let extension: String = extension.to_string();
        let mut renderer = ImageRenderer::new(width as u32, height as u32);

        // 反序列化为 Chart
        let chart = serde_json::from_str::<Chart>(&data)
            .map_err(|e| EchartsError::Unknown(e.to_string()))?;

        if extension == "svg" {
            let x = renderer
                .render(&chart)
                .map_err(|e| EchartsError::Unknown(e.to_string()))?;
            Ok(env.byte_array_from_slice(x.as_bytes())?)
        } else {
            let extension = parse_extension(&extension)?;
            let x = renderer
                .render_format(extension, &chart)
                .map_err(|e| EchartsError::Unknown(e.to_string()))?;
            Ok(env.byte_array_from_slice(x.as_slice())?)
        }
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

/// 通过文件名字解析扩展名
fn parse_extension_by_path(path: &Path) -> Result<ImageFormat, EchartsError> {
    ImageFormat::from_path(path).map_err(|_| EchartsError::Format)
}

fn parse_extension(ext: &str) -> Result<ImageFormat, EchartsError> {
    ImageFormat::from_extension(ext).ok_or(EchartsError::Format)
}
