use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long = "sampling-count", default_value_t = 16)]
    sampling_count: u16,
}

pub fn main()
{
    let scene_xml = r##"
<?xml version="1.0" encoding="UTF-8"?>
<scene name="main_body">
  <sky>
    <lower_color>0.0</lower_color>
    <lower_color>0.0</lower_color>
    <lower_color>0.0</lower_color>

    <upper_color>0.7</upper_color>
    <upper_color>0.75</upper_color>
    <upper_color>0.95</upper_color>
  </sky>
  <sphere radius = "1.5"/>
  <sphere radius = "0.75">
    <transform>
      <translation>0.0</translation>
      <translation>5.0</translation>
      <translation>0.0</translation>

      <rotation>0.0</rotation>
      <rotation>0.0</rotation>
      <rotation>0.0</rotation>

      <scale>1.0</scale>
      <scale>1.0</scale>
      <scale>1.0</scale>
    </transform>
    <material>
      <albedo>1</albedo>
      <albedo>0.5</albedo>
      <albedo>0.5</albedo>
      <emission>350</emission>
      <emission>350</emission>
      <emission>350</emission>
    </material>
  </sphere>
</scene>
  "##;

    let args = Args::parse();

    let scene_data = sjrt::scene::Loader::load_from_text(&scene_xml);
    let scene = sjrt::RapierScene::new_from_scene(&scene_data);
    let renderer = sjrt::PathTracer::new(args.sampling_count, 4, false);
    let mut buffer = sjrt::image::ImageBuffer::new(480, 480);
    let system = sjrt::System::new();
    system.execute(&scene, &mut buffer, &renderer);

    buffer.save("example.png");
}
