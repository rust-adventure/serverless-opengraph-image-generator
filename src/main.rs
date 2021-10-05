use http::StatusCode;
use image::{
    png::PngEncoder, ColorType, DynamicImage, ImageBuffer,
    ImageEncoder, ImageFormat, ImageOutputFormat, Pixel,
    Rgba, RgbaImage,
};
use lambda_runtime::{handler_fn, Context, Error};
use og_image_writer::{style, writer::OGImageWriter};
use serde_json::{json, Value};

const width: u32 = 1024;
const height: u32 = 512;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: Value,
    _: Context,
) -> Result<Value, Error> {
    dbg!(event);
    let encoded_data = gen_image("Rust Adventure Dynamic Image Serverless Function Test")?;

    Ok(json!({
        "headers": {
            "Content-Type": "image/png",
            "Content-Length": encoded_data.len().to_string()
        },
        "statusCode": StatusCode::OK.as_u16(),
        "body": base64::encode(encoded_data),
        "isBase64Encoded": true
    }))
}

fn gen_image(title: &str) -> Result<Vec<u8>, Error> {
    let mut writer =
        OGImageWriter::new(style::WindowStyle {
            width,
            height,
            background_color: Some(style::Rgba([
                255, 255, 255, 255,
            ])),
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })
        .unwrap();

    let font_bold = Vec::from(include_bytes!(
        "../AlfaSlabOne-Regular.ttf"
    ) as &[u8]);
    let font_medium =
        Vec::from(include_bytes!("../FiraMono-Medium.ttf")
            as &[u8]);

    let mut top_container =
        OGImageWriter::new(style::WindowStyle {
            width,
            height: height - 50,
            background_color: Some(style::Rgba([
                223, 246, 245, 255,
            ])),
            align_items: style::AlignItems::Start,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })?;

    top_container.set_text(
        title,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 100.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 113, 55, 255]),
            text_align: style::TextAlign::Start,
            ..style::Style::default()
        },
        font_bold.clone(),
    )?;

    writer.set_container(
        &mut top_container,
        style::Style {
            margin: style::Margin(0, 0, 0, 0),
            text_align: style::TextAlign::Center,
            position: style::Position::Absolute,
            ..style::Style::default()
        },
    )?;

    let mut container =
        OGImageWriter::new(style::WindowStyle {
            width: width - 300,
            height: 50,
            background_color: Some(style::Rgba([
                255, 113, 55, 255,
            ])),
            align_items: style::AlignItems::Start,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })?;

    container.set_text(
        "Dynamic OpenGraph Image",
        style::Style {
            margin: style::Margin(10, 0, 10, 15),
            line_height: 1.5,
            font_size: 30.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Center,
            text_overflow: style::TextOverflow::Ellipsis,
            max_height: Some(50),
            position: style::Position::Absolute,
            ..style::Style::default()
        },
        font_medium.clone(),
    )?;

    writer.set_container(
        &mut container,
        style::Style {
            margin: style::Margin(
                height as i32 - 50,
                0,
                0,
                0,
            ),
            text_align: style::TextAlign::Center,
            position: style::Position::Absolute,
            ..style::Style::default()
        },
    )?;
    let mut bottom_right_container =
        OGImageWriter::new(style::WindowStyle {
            width: 300,
            height: 50,
            background_color: Some(style::Rgba([
                255, 113, 55, 255,
            ])),
            align_items: style::AlignItems::Start,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })?;
    bottom_right_container.set_text(
        "Rust Adventure",
        style::Style {
            margin: style::Margin(5, 0, 5, 0),
            line_height: 1.5,
            font_size: 40.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::End,
            text_overflow: style::TextOverflow::Ellipsis,
            max_height: Some(50),
            position: style::Position::Absolute,
            ..style::Style::default()
        },
        font_bold.clone(),
    )?;
    writer.set_container(
        &mut bottom_right_container,
        style::Style {
            margin: style::Margin(
                height as i32 - 50,
                0,
                0,
                width as i32 - 300,
            ),
            text_align: style::TextAlign::Center,
            position: style::Position::Absolute,
            ..style::Style::default()
        },
    )?;
    writer.paint()?;

    let img = writer.image()?;
    let mut buf = vec![];
    let dyn_img = DynamicImage::ImageRgba8(img);
    dyn_img
        .write_to(&mut buf, ImageOutputFormat::Png)
        .unwrap();
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn generates_image() {
        let image = gen_image().unwrap();
        std::fs::write("./test-file.png", image).unwrap()
    }
}
