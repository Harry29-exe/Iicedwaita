use iced::Vector;
use iced_native::Background;
use iced_native::widget::button as iced_button;
use iced_native::widget::button::Appearance;
use crate::style;
use crate::style::colors::{get_colors, IcedwaitaColors, mix, with_alpha};
use crate::style::{colors, IcedwaitaTheme};
use crate::style::alpha;

#[derive(Copy, Clone, Default)]
pub enum Button {
    #[default]
    Default,
    DefaultExt{checked: bool},
    Primary,
    PrimaryExt{checked: bool},
}

fn appearance(style: &Button, is_dark: bool) -> iced_button::Appearance {
    let pallete = get_colors(is_dark);
    match style {
        Button::Default => default_appearance(pallete),
        Button::DefaultExt{checked} => iced_button::Appearance {
            background: match checked {
                true => alpha::SELECTED_BTN.to_some_bg(&pallete.opaque_mixcol()),
                false => None
            },
            ..default_appearance(pallete)
        },
        Button::Primary => iced_button::Appearance {
            background: Some(pallete.accent_bg_color.into()),
            text_color: pallete.accent_fg_color,
            ..default_appearance(pallete)
        },
        Button::PrimaryExt{checked} => iced_button::Appearance {
            background: match checked {
                true => alpha::CHECKED.mix_into_some_bg(
                    &pallete.accent_bg_color,
                    &pallete.accent_fg_color),
                false => None
            },
            text_color: pallete.accent_fg_color,
            ..default_appearance(pallete)
        }
    }
}

fn appearance_hower(style: &Button, is_dark: bool) -> iced_button::Appearance {
    let pallete = get_colors(is_dark);
    match style {
        Button::Default => iced_button::Appearance {
            background: alpha::HOVER_BTN.to_some_bg(
                &pallete.opaque_mixcol()),
            ..default_appearance(pallete)
        },
        Button::DefaultExt{checked} => iced_button::Appearance {
            background: alpha::for_hover_btn(*checked).to_some_bg(&pallete.opaque_mixcol()),
            ..default_appearance(pallete)
        },
        Button::Primary => iced_button::Appearance {
            background: alpha::HOVER.mix_into_some_bg(
                &pallete.accent_bg_color,
                &pallete.accent_fg_color),
            text_color: pallete.accent_fg_color,
            ..default_appearance(pallete)
        },
        Button::PrimaryExt {checked} => iced_button::Appearance {
            background: alpha::for_hover(*checked).mix_into_some_bg(
                &pallete.accent_bg_color,
                &pallete.opaque_mixcol()),
            text_color: pallete.accent_fg_color,
            ..default_appearance(pallete)
        }
    }
}

fn appearance_pressed(style: &Button, is_dark: bool) -> iced_button::Appearance {
    let pallete = get_colors(is_dark);
    match style {
        Button::Default => iced_button::Appearance {
            background: alpha::ACTIVE_BTN.to_some_bg(&pallete.opaque_mixcol()),
            ..default_appearance(pallete)
        },
        Button::DefaultExt{checked} => iced_button::Appearance {
            background: alpha::for_active_btn(*checked).to_some_bg(&pallete.opaque_mixcol()),
            ..default_appearance(pallete)
        },
        Button::Primary => iced_button::Appearance {
            background: alpha::ACTIVE.mix_into_some_bg(
                &pallete.accent_bg_color,
                &colors::basics::DARK_5),
            text_color: pallete.accent_fg_color,
            ..default_appearance(pallete)
        },
        Button::PrimaryExt {checked} => iced_button::Appearance {
            background: alpha::for_active(*checked).mix_into_some_bg(
                &pallete.accent_bg_color,
                &colors::basics::DARK_5),
            text_color: pallete.accent_fg_color,
            ..default_appearance(pallete)
        }
    }
}

#[inline]
fn default_appearance(pallete: &IcedwaitaColors) -> iced_button::Appearance {
    Appearance {
        shadow_offset: Vector::default(),
        background: Option::None,
        border_radius: style::border::radius::MD,
        border_width: style::border::NONE,
        border_color: pallete.border_color,
        text_color: pallete.text_color,
    }
}

impl iced_button::StyleSheet for IcedwaitaTheme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        appearance(style, self.is_dark_theme)
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        appearance_hower(style, self.is_dark_theme)
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        appearance_pressed(style, self.is_dark_theme)
    }
}

impl iced_button::StyleSheet for Button {
    type Style = iced_native::Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            iced_native::Theme::Custom(..) | iced_native::Theme::Dark => appearance(self, true),
            iced_native::Theme::Light => appearance(self, false)
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            iced_native::Theme::Light => appearance_hower(self, false),
            _ => appearance_hower(self, true)
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        match style {
            iced_native::Theme::Light => appearance_pressed(self, false),
            _ => appearance_pressed(self, true)
        }
    }
}

impl From<Button> for iced_native::theme::Button {
    fn from(value: Button) -> Self {
        iced_native::theme::Button::Custom(Box::new(value))
    }
}
