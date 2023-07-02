use iced::Vector;
use iced_native::Background;
use iced_native::widget::button as iced_button;
use iced_native::widget::button::Appearance;
use crate::style;
use crate::style::colors::{get_colors, IcedwaitaColors};
use crate::style::IcedwaitaTheme;

#[derive(Copy, Clone, Default)]
pub enum Button {
    #[default]
    Default,
    Primary
}

fn appearance(style: &Button, is_dark: bool) -> iced_button::Appearance {
    let pallete = get_colors(is_dark);
    match style {
        Button::Default => default_appearance(pallete),
        Button::Primary => iced_button::Appearance {
            background: Option::Some(Background::Color(pallete.accent_bg_color)),
            ..default_appearance(pallete)
        }
    }
}

#[inline]
fn default_appearance(pallete: &IcedwaitaColors) -> iced_button::Appearance {
    Appearance {
        shadow_offset: Vector::default(),
        background: Option::Some(Background::Color(pallete.view_bg_color)),
        border_radius: style::border::radius::MD,
        border_width: style::border::NONE,
        border_color: pallete.border_color,
        text_color: pallete.text_color,
    }
}

impl iced_button::StyleSheet for IcedwaitaTheme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        todo!()
    }
}
