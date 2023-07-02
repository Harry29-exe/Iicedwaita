use iced::{Element, Settings, Sandbox};
use iced::widget::{container, row, text, column};


fn main() {
    Application::run(Settings::default());
}


struct Application {
}

impl iced::Sandbox for Application {
    type Message = ();

    fn new() -> Self {
        Self{

        }
    }

    fn title(&self) -> String {
        "Hello World".to_string()
    }

    fn update(&mut self, message: Self::Message) {

    }

    fn view(&self) -> Element<'_, Self::Message> {
        let hello = container(text("Hello"))
            .style(icedwaita::style::components::Container::WithBorder)
            .padding(icedwaita::style::size::padding::MD);

        let world = container(text("World"))
            .style(icedwaita::style::components::Container::Default)
            .padding(icedwaita::style::size::padding::MD);

        column![
            hello,
            world
        ].into()
    }
}