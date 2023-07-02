use iced::{Element, Settings, Sandbox, Alignment};
use iced::Length::Fill;
use iced::widget::{container, row, text, column, Row};
use iced_native::widget::button;


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
        let hello_world = self.hello_world_line();

        let default_btn = button("Default")
            .style(icedwaita::style::Button::Default.into());
        let primary_btn = button("Primary")
            .style(icedwaita::style::Button::Primary.into());

        let buttons_row = row![default_btn, primary_btn];

        column![hello_world, buttons_row]
            .into()
    }



}

impl Application {
    fn hello_world_line<'a>(&self) -> Row<'a, ()> {
        let hello = container(text("Hello"))
            .style(icedwaita::style::Container::WithBorder)
            .padding(icedwaita::style::size::padding::MD);

        let spaccing = container(text(""))
            .width(Fill);
        let world = container(text("World"))
            .style(icedwaita::style::Container::Default)
            .padding(icedwaita::style::size::padding::MD);

        let hello_world = row![
            hello,
            spaccing,
            world
        ]
            .width(Fill)
            .align_items(Alignment::Center);

        hello_world.into()
    }
}