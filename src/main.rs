use show_image::{create_window, event, ImageInfo, ImageView};
use ultraviolet::DVec3;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

enum RaySphereIntersection {
    No,
    Yes(f64),
}

fn ray_sphere_intersection(
    ray_origin: &DVec3,
    ray: &DVec3,
    sphere_center: &DVec3,
    sphere_radius: &f64,
) -> RaySphereIntersection {
    let oc = *ray_origin - *sphere_center;
    let oc_dot_ray = ray.dot(oc);
    let discriminant = oc_dot_ray * oc_dot_ray - (oc.mag_sq() - sphere_radius * sphere_radius);
    if discriminant < 0.0 {
        RaySphereIntersection::No
    } else {
        let d = -oc_dot_ray - discriminant.sqrt();
        RaySphereIntersection::Yes(d)
    }
}

fn render_pixel(x: f64, y: f64) -> Vec<f64> {
    let ray_origin = DVec3::new(x, y, 0.0);
    let ray = DVec3::new(0.0, 0.0, 1.0);
    let sphere_center = DVec3::new(0.3, 0.0, 5.0);
    let sphere_radius = 0.5;

    match ray_sphere_intersection(&ray_origin, &ray, &sphere_center, &sphere_radius) {
        RaySphereIntersection::No => {
            vec![0.0, 0.0, 0.1]
        }
        RaySphereIntersection::Yes(d) => {
            let intersection = ray_origin + d * ray;
            let intersection_normal = (intersection - sphere_center).normalized();
            let shade = ray.dot(intersection_normal).abs();

            vec![shade, 0.0, 0.0]
        }
    }
}

fn render_loop(image_info: &ImageInfo) -> Vec<u8> {
    let float_to_channel_factor = ((1
        << (8 * (image_info.pixel_format.bytes_per_pixel() / image_info.pixel_format.channels())))
        - 1) as f64;
    let aspect_ratio = (image_info.size.y as f64) / (image_info.size.x as f64);

    let pixel_data = (0..image_info.size.y)
        .flat_map(|y| {
            (0..image_info.size.x).flat_map(move |x| {
                let device_x = 2.0 * ((x as f64 / image_info.size.x as f64) - 0.5);
                let device_y = 2.0 * aspect_ratio * ((y as f64 / image_info.size.y as f64) - 0.5);
                render_pixel(device_x, device_y)
                    .into_iter()
                    .map(|v| v.clamp(0.0, 1.0))
            })
        })
        .map(|v| (v * float_to_channel_factor) as u8) // [0.0, 1.0] -> [0, 255]
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
