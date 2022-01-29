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
}

fn main() {
    let args = Args::parse();

    let mut buffer = sjrt::image::ImageBuffer::new(args.width, args.height);
    let path_tracer = sjrt::PathTracer::new(args.sampling_count);
    let scene = sjrt::RapierScene::new();
    sjrt::System::new().execute(&scene, &mut buffer, &path_tracer);

    buffer.save("test.png");
}
