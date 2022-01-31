use clap::Parser;

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

    #[clap(short = 'j', long = "thread-count", default_value_t = 1)]
    thread_count: i32,

    #[clap(short = 'd', long = "depth-max", default_value_t = 50)]
    depth_max: u16,

    #[clap(short = 'o', long = "output")]
    output_file_path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut buffer = sjrt::image::ImageBuffer::new(args.width, args.height);
    let path_tracer = sjrt::PathTracer::new(args.sampling_count, args.depth_max);
    let scene = sjrt::RapierScene::new();

    if args.thread_count == 1 {
        let start = std::time::Instant::now();
        sjrt::System::new().execute(&scene, &mut buffer, &path_tracer);
        let end = start.elapsed();

        println!("{} sec, {}", end.as_secs(), end.subsec_nanos() / 1_000_000);
    }
    else {
        let start = std::time::Instant::now();
        sjrt::ParallelizeSystem::new().execute(std::sync::Arc::new(scene), &mut buffer, std::sync::Arc::new(path_tracer));
        let end = start.elapsed();

        println!("{} sec, {}", end.as_secs(), end.subsec_nanos() / 1_000_000);
    }

    buffer.save(args.output_file_path);
}
