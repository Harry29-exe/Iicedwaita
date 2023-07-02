pub struct IcedwaitaTheme {
    pub is_dark_theme: bool,
}

mod components;
pub use components::Container;
pub use components::Button;


pub mod size {

    pub enum Text {
        Sm,
        Md,
        Lg,
        Xl,
        Xl2,
    }

    pub enum Widget {
        Sm,
        Md,
        Lg,
    }

    pub mod padding {
        pub const SM: f32 = 16.;
        pub const MD: f32 = 32.;
    }

}

pub mod border {
    pub const NONE: f32 = 0.;
    pub const MD: f32 = 2.;

    pub mod radius {
        pub const NONE: f32 = 0.;
        pub const SM: f32 = 2.;
        pub const MD: f32 = 6.;
        pub const LG: f32 = 9.;
        pub const XL: f32 = 12.;
    }
}

pub mod colors;