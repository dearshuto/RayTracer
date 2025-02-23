use std::sync::Arc;

use clap::Parser;
use sjrt::IBuffer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short = 's', long = "sampling-count", default_value_t = 16)]
    sampling_count: u16,

    /// Number of times to greet
    #[clap(short = 'x', long = "width", default_value_t = 128)]
    width: i32,

    #[clap(short = 'y', long = "height", default_value_t = 128)]
    height: i32,

    #[clap(long = "thread-count-x", default_value_t = 1)]
    thread_count_x: u8,

    #[clap(long = "thread-count-y", default_value_t = 1)]
    thread_count_y: u8,

    #[clap(short = 'd', long = "depth-max", default_value_t = 50)]
    depth_max: u16,

    #[clap(long = "enable-nee")]
    is_nee_enabled: bool,

    #[clap(short = 'o', long = "output", default_value = "test.png")]
    output_file_path: std::path::PathBuf,

    #[clap(short = 'p', long = "port", default_value_t = -1)]
    port: i32,

    #[clap(short = 'i', long = "ip-address", default_value = "")]
    ip_address: String,

    #[clap(long = "scene-file", default_value = "_")]
    scene_file: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut buffer = sjrt::util::ImageBuffer::new(args.width, args.height);
    let scene = if args.scene_file.exists() {
        let scene = sjrt_res::Loader::xml()
            .deserialize(std::io::BufReader::new(
                std::fs::File::open(args.scene_file).unwrap(),
            ))
            .unwrap();
        sjrt::util::RapierScene::new_from_scene(&scene)
    } else {
        sjrt::util::RapierScene::new()
    };

    if args.port != -1 {
        let client = sjrt_net::RenderingClient {
            width: args.width,
            height: args.height,
            sampling_count: args.sampling_count,
            thread_count_x: args.thread_count_x as i32,
            thread_count_y: args.thread_count_y as i32,
        };
        let addr = format!("{}:{}", args.ip_address, args.port)
            .parse()
            .unwrap();
        println!("{}", addr);
        let result = client.run(addr).await?;
        let image_view = result.get_ref();
        println!(
            "width range : {}-{}",
            image_view.width_start, image_view.width_end
        );
        println!(
            "height range: {}-{}",
            image_view.height_start, image_view.height_end
        );

        for y in image_view.height_start..image_view.height_end {
            for x in image_view.width_start..image_view.width_end {
                let width = image_view.width_end - image_view.width_start;

                let index = (x + y * width) as usize;
                let pixel = image_view.pixels[index];
                let red = ((pixel >> 24) & 0xFF) as u8;
                let green = ((pixel >> 16) & 0xFF) as u8;
                let blue = ((pixel >> 8) & 0xFF) as u8;
                buffer.set_color(x, y, red, green, blue);
            }
        }
    } else {
        let width = args.width as u32;
        let height = args.height as u32;
        let renderer = sjrt::PathTracerEx::default()
            .with_depth(args.depth_max as u32)
            .with_sampling_count(args.sampling_count as u32);
        let rays = sjrt::Camera::builder()
            .with_field_of_view(std::f32::consts::PI / 5.5)
            .with_position(&nalgebra::Vector3::new(4.8, 4.73, -8.0))
            .with_look_at(&nalgebra::Vector3::new(4.8, 4.73, 0.0))
            .build()
            .calculate_ray_direction_range(width, height, 0..width, 0..height);

        let start = std::time::Instant::now();

        // スレッド数が指定されたら並列実行
        if args.thread_count_x == 1 && args.thread_count_y == 1 {
            sjrt::Executor::default().execute(&mut buffer, rays.into_iter(), scene, renderer);
        } else {
            sjrt::Executor::default()
                .execute_async(
                    &mut buffer,
                    rays.into_iter(),
                    Arc::new(scene),
                    Arc::new(renderer),
                )
                .await;
        }

        let end = start.elapsed();

        println!(
            "{} sec, {}",
            end.as_secs(),
            end.subsec_nanos() as f32 / 1_000_000f32
        );
    }

    buffer.save(args.output_file_path);

    Ok(())
}
