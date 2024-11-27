use std::sync::Arc;

use iced::{
    widget::{Button, Column, Container, Row, TextInput},
    Alignment, Sandbox,
};

use sjrt::IBuffer;

fn main() {
    let _result = MainWindow::run(iced::Settings::default());
}

#[derive(Debug, Clone)]
enum MainWindowMessage {
    Run,
    Save,
    WidthChanged(String),
    HeightChanged(String),
    DepthChanged(String),
    InputChanged(String),
}

struct MainWindow {
    buffer: sjrt::util::ImageBuffer,
    sampling_count: String,
    width_string: String,
    height_string: String,
    depth_string: String,
    runtime: tokio::runtime::Runtime,
}

impl iced::Sandbox for MainWindow {
    type Message = MainWindowMessage;

    fn new() -> Self {
        let buffer = sjrt::util::ImageBuffer::new(512, 512);
        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();

        Self {
            buffer,
            sampling_count: String::from("32"),
            width_string: String::from("512"),
            height_string: String::from("512"),
            depth_string: String::from("8"),
            runtime,
        }
    }

    fn title(&self) -> String {
        String::from("sjrteditor")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MainWindowMessage::Run => {
                let buffer = self.runtime.block_on(async {
                    let mut buffer = sjrt::util::ImageBuffer::new(
                        self.width_string.parse().unwrap(),
                        self.height_string.parse().unwrap(),
                    );

                    let sampling_count: u16 = self.sampling_count.parse().unwrap();
                    let scene = sjrt::util::RapierScene::new();
                    let renderer = sjrt::PathTracer::new(
                        sampling_count,
                        self.depth_string.parse().unwrap(), /*depth*/
                        false,                              /*nee*/
                    );
                    let system = sjrt::ParallelizeSystem::new_with_thread(16, 16);
                    system
                        .execute(Arc::new(scene), &mut buffer, Arc::new(renderer))
                        .await;
                    buffer
                });
                self.buffer = buffer;
            }
            MainWindowMessage::Save => self.buffer.save("test.png"),
            MainWindowMessage::WidthChanged(new_width) => self.width_string = new_width,
            MainWindowMessage::HeightChanged(new_height) => self.height_string = new_height,
            MainWindowMessage::DepthChanged(new_depth) => self.depth_string = new_depth,
            MainWindowMessage::InputChanged(new_value) => {
                self.sampling_count = new_value;
            }
        }
    }

    fn view(&self) -> iced::Element<MainWindowMessage> {
        let mut pixels = Vec::new();
        for y in 0..self.buffer.get_height() {
            for x in 0..self.buffer.get_width() {
                let red = self.buffer.get_red(x, y);
                let blue = self.buffer.get_blue(x, y);
                let green = self.buffer.get_green(x, y);
                let alpha = u8::MAX;
                pixels.push(red);
                pixels.push(green);
                pixels.push(blue);
                pixels.push(alpha);
            }
        }

        let handle = iced::widget::image::Handle::from_pixels(
            self.buffer.get_width() as u32,
            self.buffer.get_height() as u32,
            pixels,
        );
        let image = iced::widget::Image::new(handle)
            .width(iced::Length::Units(self.buffer.get_width() as u16))
            .height(iced::Length::Units(self.buffer.get_height() as u16));

        let contents: iced::Element<_> = Row::new()
            .align_items(Alignment::Start)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(image)
            .push(
                Column::new()
                    .spacing(20)
                    .padding(20)
                    .push(TextInput::new("Width", &self.width_string, |str| {
                        MainWindowMessage::WidthChanged(str)
                    }))
                    .push(TextInput::new("Height", &self.height_string, |str| {
                        MainWindowMessage::HeightChanged(str)
                    }))
                    .push(TextInput::new("Depth", &self.depth_string, |str| {
                        MainWindowMessage::DepthChanged(str)
                    }))
                    .push(TextInput::new("", &self.sampling_count, |str| {
                        MainWindowMessage::InputChanged(str)
                    }))
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(Button::new("Run").on_press(MainWindowMessage::Run))
                            .push(Button::new("Save").on_press(MainWindowMessage::Save)),
                    ),
            )
            .into();

        Container::new(contents)
            .height(iced::Length::Fill)
            .center_y()
            .into()
    }
}
