use iced::widget::container::Appearance;
use iced_native::{Background, theme, widget};
use crate::style;
use crate::style::{colors, IcedwaitaTheme};

#[derive(Copy, Clone, Default)]
pub enum Container {
    #[default]
    Default,
    WithBorder,
}

fn appearance(style: &Container, dark_theme: bool) -> Appearance {
    let colors = colors::get_colors(dark_theme);
    match style {
        Container::Default => Appearance {
            text_color: Some(colors.text_color),
            background: Some(Background::Color(colors.window_bg_color)),
            border_color: colors.border_color,
            border_width: style::border::NONE,
            border_radius: style::border::radius::NONE,
            ..Appearance::default()
        },
        Container::WithBorder => Appearance{
            text_color: Some(colors.text_color),
            background: Some(Background::Color(colors.window_bg_color)),
            border_color: colors.border_color,
            border_width: style::border::MD,
            border_radius: style::border::radius::MD,
            ..Appearance::default()
        },
    }
}

// for use with IcedwaitaTheme
impl iced_native::widget::container::StyleSheet for IcedwaitaTheme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        appearance(style, self.is_dark_theme)
    }
}

// for use with iced_native::Theme
impl iced_native::widget::container::StyleSheet for Container {
    type Style = iced_native::Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            iced_native::Theme::Dark | iced_native::Theme::Custom(..) => appearance(self, true),
            iced_native::Theme::Light => appearance(self, false)
        }
    }
}

impl From<Container> for iced_native::theme::Container {
    fn from(value: Container) -> Self {
        iced_native::theme::Container::Custom(Box::new(value))
    }
}