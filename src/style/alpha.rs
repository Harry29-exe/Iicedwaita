use std::cmp::min;
use iced::Color;
use iced_core::Background;
use iced_core::keyboard::KeyCode::A;
use crate::style::colors;
use crate::style::colors::mix;

pub const HOVER: Alpha = Alpha{a: 0.07};//0.15 | 0.07
pub const ACTIVE: Alpha = Alpha{a: 0.16}; //0.3 | 0.16
pub const CHECKED: Alpha = Alpha{a: 0.10}; //0.3
pub const CHECKED_HOVER: Alpha = Alpha{a: 0.13};
pub const CHECKED_ACTIVE: Alpha = Alpha{a: 0.19};

pub const HOVER_BTN: Alpha = Alpha{a: 0.07};
pub const ACTIVE_BTN: Alpha = Alpha{a: 0.16};
pub const SELECTED_BTN: Alpha = Alpha{a: 0.10};
pub const SELECTED_HOVER_BTN: Alpha = Alpha{a: 0.13};
pub const SELECTED_ACTIVE_BTN: Alpha = Alpha{a: 0.19};

// pub const HOVER_BTN: Alpha = Alpha { a: 0.15 };
// pub const ACTIVE_BTN: Alpha = Alpha { a: 0.30 };
// pub const SELECTED_BTN: Alpha = Alpha { a: 0.30 };
// pub const SELECTED_HOVER_BTN: Alpha = Alpha{a: 0.35};
// pub const SELECTED_ACTIVE_BTN: Alpha = Alpha { a: 0.4 };

#[inline]
pub fn for_hover_btn(checked: bool) -> Alpha {
    if checked {
        SELECTED_HOVER_BTN
    } else {
        HOVER_BTN
    }
}

#[inline]
pub fn for_active_btn(checked: bool) -> Alpha {
    if checked {
        SELECTED_ACTIVE_BTN
    } else {
        ACTIVE_BTN
    }
}

#[inline]
pub fn for_hover(checked: bool) -> Alpha {
    if checked {
        CHECKED_HOVER
    } else {
        HOVER
    }
}

#[inline]
pub fn for_active(checked: bool) -> Alpha {
    if checked {
        CHECKED_ACTIVE
    } else {
        ACTIVE
    }
}

#[inline]
pub fn from_state(is_checked: bool, is_hovered: bool, is_active: bool) -> Alpha {
    match (is_checked, is_hovered, is_active) {
        (true, false, false) => CHECKED,
        (true, .., true) => CHECKED_ACTIVE,
        (true, true, false) => CHECKED_HOVER,
        (false, .., true) => ACTIVE,
        (false, true, false) => HOVER,
        (false, false, false) => Alpha{a: 0.}
    }
}

#[derive(Copy, Clone)]
pub struct Alpha {
    pub a: f32,
}

impl Alpha {
    pub fn color(&self, c: &Color) -> Color {
        Color {
            a: c.a * self.a,
            ..(*c)
        }
    }

    pub fn to_some_bg(&self, c: &Color) -> Option<iced_native::Background> {
        Option::Some(Background::Color(
            mix(&Color{a: self.a, ..(*c)},
                &Color::from_rgba(0., 0., 0., 0.65),
                0.85)
        ))
        // Some(iced_native::Background::from(
        //     Color {
        //         a: self.a,
        //         r: f32::max(c.r - 0.08, 0.),
        //         g: f32::max(c.g - 0.08, 0.),
        //         b: f32::max(c.b - 0.08, 0.),
        //     }
        // ))
    }

    pub fn mix(&self, primary: &Color, additional: &Color) -> Color {
        colors::mix(primary, additional, 1. - self.a)
    }

    pub fn mix_into_bg(&self, primary: &Color, additional: &Color) -> iced_native::Background {
        iced_native::Background::from(
            colors::mix(primary, additional, 1. - self.a)
        )
    }

    pub fn mix_into_some_bg(&self, primary: &Color, additional: &Color) -> Option<iced_native::Background> {
        Some(iced_native::Background::from(
            colors::mix(primary, additional, 1. - self.a)
        ))
    }

    #[inline]
    pub fn inverse(&self) -> Alpha {
        Alpha{a: 1. - self.a}
    }
}
