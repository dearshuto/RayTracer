#[derive(Debug, Default, sjrt_macro::Immutable)]
struct HelloWorld {
    value: u32,
    name: String,
}

fn main() {
    let data = HelloWorld::default();
    println!("{:?}", data);

    let data = data.with_value(10);
    println!("{:?}", data);

    let data = data.update_value(|name| name + 5);
    println!("{:?}", data);

    println!("{:?}", data.with_name("Hello World!".to_string()));
}
