use iced::Sandbox;

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
    InputChanged(String),
}

struct MainWindow {
    buffer: sjrt::image::ImageBuffer,
    sampling_count: String,
    width_string: String,
    height_string: String,
    button_state: iced::button::State,
    save_button_state: iced::button::State,
    width_state: iced::text_input::State,
    height_state: iced::text_input::State,
    sampling_count_state: iced::text_input::State,
}

impl iced::Sandbox for MainWindow {
    type Message = MainWindowMessage;

    fn new() -> Self {
        let buffer = sjrt::image::ImageBuffer::new(512, 512);

        Self {
            buffer,
            sampling_count: String::from("32"),
            width_string: String::from("512"),
            height_string: String::from("512"),
            button_state: iced::button::State::new(),
            save_button_state: iced::button::State::new(),
            width_state: std::default::Default::default(),
            height_state: std::default::Default::default(),
            sampling_count_state: std::default::Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("sjrteditor")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MainWindowMessage::Run => {
                let buffer = sjrt::image::ImageBuffer::new(
                    self.width_string.parse().unwrap(),
                    self.height_string.parse().unwrap(),
                );
                self.buffer = buffer;

                let sampling_count: u16 = self.sampling_count.parse().unwrap();
                let scene = sjrt::RapierScene::new();
                let renderer =
                    sjrt::PathTracer::new(sampling_count, 1 /*depth*/, false /*nee*/);
                let system = sjrt::System::new();
                system.execute(&scene, &mut self.buffer, &renderer);
            }
            MainWindowMessage::Save => self.buffer.save("test.png"),
            MainWindowMessage::WidthChanged(new_width) => self.width_string = new_width,
            MainWindowMessage::HeightChanged(new_height) => self.height_string = new_height,
            MainWindowMessage::InputChanged(new_value) => {
                self.sampling_count = new_value;
            }
        }
    }

    fn view(&mut self) -> iced::Element<MainWindowMessage> {
        let mut pixels = Vec::new();
        for y in 0..self.buffer.get_height() {
            for x in 0..self.buffer.get_width() {
                let red = self.buffer.get_red(x, y);
                let blue = self.buffer.get_blue(x, y);
                let green = self.buffer.get_green(x, y);
                let alpha = std::u8::MAX;
                pixels.push(red);
                pixels.push(green);
                pixels.push(blue);
                pixels.push(alpha);
            }
        }

        let handle = iced::image::Handle::from_pixels(
            self.buffer.get_width() as u32,
            self.buffer.get_height() as u32,
            pixels,
        );
        let image = iced::Image::new(handle)
            .width(iced::Length::Units(self.buffer.get_width() as u16))
            .height(iced::Length::Units(self.buffer.get_height() as u16));

        let contents: iced::Element<_> = iced::Row::new()
            .align_items(iced::Align::Start)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(image)
            .push(
                iced::Column::new()
                    .spacing(20)
                    .padding(20)
                    .push(iced::TextInput::new(
                        &mut self.width_state,
                        "Width",
                        &self.width_string,
                        MainWindowMessage::WidthChanged,
                    ))
                    .push(iced::TextInput::new(
                        &mut self.height_state,
                        "Height",
                        &self.height_string,
                        MainWindowMessage::HeightChanged,
                    ))
                    .push(iced::TextInput::new(
                        &mut self.sampling_count_state,
                        "",
                        &self.sampling_count,
                        MainWindowMessage::InputChanged,
                    ))
                    .push(
                        iced::Row::new()
                            .spacing(10)
                            .push(
                                iced::Button::new(&mut self.button_state, iced::Text::new("Run"))
                                    .on_press(MainWindowMessage::Run),
                            )
                            .push(
                                iced::Button::new(
                                    &mut self.save_button_state,
                                    iced::Text::new("Save"),
                                )
                                .on_press(MainWindowMessage::Save),
                            ),
                    ),
            )
            .into();

        iced::Container::new(contents)
            .height(iced::Length::Fill)
            .center_y()
            .into()
    }
}
