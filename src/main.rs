use show_image::{create_window, event, ImageInfo, ImageView};
use ultraviolet::DVec3;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn render_pixel(x: f64, y: f64) -> Vec<f64> {
    let ray = DVec3::new(x, y, 0.0);
    let origin = DVec3::zero();
    //let view = DVec3::new(0.0, 0.0, 1.0);
    let distance = (ray - origin).mag_sq();
    if distance < 0.4 {
        vec![x.abs(), 0.0, 0.0]
    } else {
        vec![0.0, y.abs(), 0.0]
    }
}

fn render_loop(image_info: &ImageInfo) -> Vec<u8> {
    let scale_factor = ((1
        << (8 * (image_info.pixel_format.bytes_per_pixel() / image_info.pixel_format.channels())))
        - 1) as f64;
    let pixel_data = (0..image_info.size.y)
        .flat_map(|y| {
            (0..image_info.size.x).flat_map(move |x| {
                let device_x = 2.0 * ((x as f64 / image_info.size.x as f64) - 0.5);
                let device_y = 2.0 * ((y as f64 / image_info.size.y as f64) - 0.5);
                let pixel = render_pixel(device_x, device_y)
                    .into_iter()
                    .map(|v| v.clamp(0.0, 1.0));

                pixel
                    .into_iter()
                    .map(|v| (v * scale_factor) as u8)
                    .collect::<Vec<u8>>()
            })
        })
        .collect::<Vec<u8>>();

    let num_bytes: usize = image_info.byte_size().try_into().unwrap();
    debug_assert_eq!(pixel_data.len(), num_bytes);

    pixel_data
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_info = ImageInfo::rgb8(WIDTH, HEIGHT);

    let pixel_data = render_loop(&image_info);
    let image = ImageView::new(image_info, &pixel_data[..]);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }

    Ok(())
}
