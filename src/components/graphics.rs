use macroquad::prelude::*;

macro_rules! load_texture {
    ($path:expr) => {
        Texture2D::from_file_with_format(
            include_bytes!($path),
            None,
        );
    };
}

#[derive(Debug)]
pub enum GraphicsObject {
    Color(Color),
    Image(Texture2D),
}

// fn load_images(paths: Vec<String>) {
//     for path in &paths {
//         let image = match path.split(".").last() {
//             Some("png") => load_png!(&path, ImageFormat::Png).unwrap(),
//         };
//     }
// }
