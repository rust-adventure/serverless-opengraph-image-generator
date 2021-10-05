use http::StatusCode;
use image::{
    png::PngEncoder, ColorType, DynamicImage, ImageBuffer,
    ImageEncoder, ImageFormat, ImageOutputFormat, Pixel,
    Rgba, RgbaImage,
};
use lambda_runtime::{handler_fn, Context, Error};
use og_image_writer::{style, writer::OGImageWriter};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    _: Value,
    _: Context,
) -> Result<Value, Error> {
    let encoded_data = gen_image()?;

    Ok(json!({
        "headers": {
            "Content-Type": "image/png",
            "Content-Length": encoded_data.len().to_string()
        },
        "statusCode": StatusCode::OK.as_u16(),
        "body": encoded_data,
        // "isBase64Encoded": true
    }))
}

fn gen_image() -> Result<Vec<u8>, Error> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer =
        OGImageWriter::new(style::WindowStyle {
            width: 1024,
            height: 512,
            background_color: Some(style::Rgba([
                70, 40, 90, 255,
            ])),
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })
        .unwrap();

    let font =
        Vec::from(include_bytes!("../FiraSans-Bold.ttf")
            as &[u8]);

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 100.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Start,
            ..style::Style::default()
        },
        font,
    )?;
    writer.paint()?;

    let img = writer.image()?;
    // let data: Vec<u8> = writer.into_vec()?;
    // let img = image::load_from_memory(&data).unwrap();
    // let rgba_image: RgbaImage =
    //     ImageBuffer::from_vec(1024, 512, data).unwrap();
    // let img = image::load_from_memory(&rgba_image).unwrap();
    // let img: RgbaImage =
    //     ImageBuffer::from_vec(1024, 512, data).unwrap();
    let mut buf = vec![];
    // let encoder = PngEncoder::new(buf);
    let dyn_img = DynamicImage::ImageRgba8(img);
    dyn_img
        .write_to(&mut buf, ImageOutputFormat::Png)
        .unwrap();

    // encoder.encode(img, 1024, 512, ColorType::Rgba8);
    // let encoded =
    //     img.write_image(buf, 1024, 512, ColorType::Rgba8);

    // let encoded_data = base64::encode(&buf);
    // println!("base64 string: {:?}", encoded_data);
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
