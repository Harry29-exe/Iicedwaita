use iced::{Element, Settings, Sandbox, Alignment, Theme};
use iced::alignment::Vertical;
use iced::Length::Fill;
use iced::widget::{container, row, text, column, Row};
use iced_native::{Length, Padding};
use iced_native::Length::FillPortion;
use iced_native::widget::button;
use crate::AppMessage::NOOP;


fn main() {
    Application::run(Settings::default());
}

struct Application {
    is_dark: bool
}

#[derive(Clone, Debug)]
enum AppMessage {
    SetDarkMode(bool),
    NOOP,
}

impl iced::Sandbox for Application {
    type Message = AppMessage;

    fn new() -> Self {
        Self{
            is_dark: true
        }
    }

    fn title(&self) -> String {
        "Hello World".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::SetDarkMode(dark_mode) => self.is_dark = dark_mode,
            AppMessage::NOOP => {},
        };
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let sterring_panel = self.sterring_panel();

        let hello_world = self.hello_world_line();

        let buttons_row = self.button_row();

        let col = column![sterring_panel, hello_world, buttons_row]
            .width(Fill);

        container(col)
            .width(Fill)
            .height(Fill)
            .style(icedwaita::style::Container::Default)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

impl Application {

    fn sterring_panel<'a>(&self) -> Row<'a, AppMessage> {
        let spacer_l = container("")
            .width(Fill);
        let spacer_r = container("")
            .width(Fill);
        let text = text("Theme mode:")
            .size(24)
            .vertical_alignment(Vertical::Center);
        let dark_mode_btn = button("Dark")
            .on_press(AppMessage::SetDarkMode(true))
            .style(icedwaita::style::Button::DefaultExt{checked: self.is_dark}.into())
            .padding([8., 20.]);

        let ligth_mode_btn = button("Light")
            .on_press(AppMessage::SetDarkMode(false))
            .style(icedwaita::style::Button::DefaultExt {checked: !self.is_dark}.into())
            .padding([8., 20.]);

        row![
            spacer_l,
            text,
            dark_mode_btn,
            ligth_mode_btn,
            spacer_r
        ]
            .padding([5., 0.])
            .width(Length::Fill)
            .align_items(Alignment::Center)
    }

    fn hello_world_line<'a>(&self) -> Row<'a, AppMessage> {
        let spacer_l = container("")
            .width(FillPortion(4));
        let spacer_r = container("")
            .width(FillPortion(4));
        let hello = container(text("Hello"))
            .style(icedwaita::style::Container::WithBorder)
            .padding(icedwaita::style::size::padding::MD);

        let spacer_m = container(text(""))
            .width(FillPortion(1));
        let world = container(text("World"))
            .style(icedwaita::style::Container::Default)
            .padding(icedwaita::style::size::padding::MD);

        row![
            spacer_l,
            hello,
            spacer_m,
            world,
            spacer_r
        ]
            .padding([5., 0.])
            .width(Fill)
            .align_items(Alignment::Center)
    }

    fn button_row<'a>(&self) -> Row<'a, AppMessage> {
        let default_btn = button("Default")
            .style(icedwaita::style::Button::Default.into())
            .padding(Padding::new(15.))
            .on_press(NOOP);

        let default_btn_wrapper = container(default_btn)
            .padding(Padding::from([5., 10.]));

        let primary_btn = button("Primary")
            .style(icedwaita::style::Button::Primary.into())
            .padding(Padding::new(15.))
            .on_press(NOOP);

        let primary_btn_wrapper = container(primary_btn)
            .padding(Padding::from([5., 10.]));

        row![default_btn_wrapper, primary_btn_wrapper]
            .padding(Padding::from([5., 0.]))
    }

}