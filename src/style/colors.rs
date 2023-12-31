use iced_native::{Background, Color};
use palette::{Blend};
use palette::encoding::Linear;
use crate::style::colors::basics::*;
use palette::rgb::{Rgb, Rgba};
use palette::encoding::Srgb;
use crate::style::alpha::Alpha;

pub type Srgba<T = f32> = Rgba<palette::encoding::Srgb, T>;

pub fn get_colors(dark_theme: bool) -> &'static IcedwaitaColors {
    if dark_theme {
        &DARK
    } else {
        &LIGHT
    }
}

#[inline]
pub fn mix(c1: &Color, c2: &Color, ratio: f32) -> Color {
    // let inverse_ratio = 1. - ratio;
    // let raw1 = Srgba::from(*c1).into_linear();
    // let raw2 = Srgba::from(*c2).into_linear();
    //
    // println!("raw1: {} {} {}", raw1.red, raw1.green, raw1.blue);
    // println!("raw2: {} {} {}", raw2.red, raw2.green, raw2.blue);
    //     // r: f32::sqrt((c1.r * c1.r * ratio) + (c2.r * c2.r * inverse_ratio)),
    //     // g: f32::sqrt((c1.g * c1.g * ratio) + (c2.g * c2.g * inverse_ratio)),
    //     // b: f32::sqrt((c1.b * c1.b * ratio) + (c2.b * c2.b * inverse_ratio)),
    //     // a: (c1.a * ratio) + (c2.a * inverse_ratio),
    // let result : palette::Alpha<palette::rgb::Rgb<Linear<Srgb>, f32>, f32> = Rgba::new(
    //     raw1.red * ratio * raw1.alpha + raw2.red * inverse_ratio * raw2.alpha,
    //     raw1.green * ratio * raw1.alpha + raw2.green * inverse_ratio  * raw2.alpha,
    //     raw1.blue * ratio * raw1.alpha + raw2.blue * inverse_ratio  * raw2.alpha,
    //     raw1.alpha * ratio + raw2.alpha * inverse_ratio,
    // );
    // let result_non_linear = Srgba::from_linear(result);
    // let color = Color::from(result_non_linear);
    // println!("{:?}", color);
    // color
    let w = 2. * ratio - 1.; //mix function from sass
    let a = c1.a - c2.a;
    let w1= ((if w * a == -1. { w } else {(w + a)/(1. + w*a)}) + 1.)/2.0;
    let w2 = 1. - w1;

    Color {
        r: w1 * c1.r + w2 * c2.r,
        g: w1 * c1.g + w2 * c2.g,
        b: w1 * c1.b + w2 * c2.b,
        a: c1.a * ratio + c2.a *(1.-ratio)
    }
}

#[cfg(test)]
mod tests {
    use iced_native::Color;
    use crate::style::colors::mix;

    #[test]
    fn mix_alpha_color() {
        let c1 = Color::from_rgba8(242, 236, 228, 0.5);
        let c2 = Color::from_rgba8(107, 113, 127, 1.0);

        let mix_c = mix(&c1, &c2, 0.5).into_rgba8();
        assert_eq!(mix_c[0], 141);
        assert_eq!(mix_c[1], 144);
        assert_eq!(mix_c[2], 152);
    }

    #[test]
    fn mix_with_0_75_ratio() {
        let c1 = Color::from_rgba8(0, 51, 102, 1.0);
        let c2 = Color::from_rgba8(210, 225, 221, 1.0);

        let mix_c = mix(&c1, &c2, 0.75).into_rgba8();
        assert_eq!(mix_c[0], 53);
        assert_eq!(mix_c[1], 95);
        assert_eq!(mix_c[2], 132);
    }

    #[test]
    fn mix_with_0_25_ratio() {
        let c1 = Color::from_rgba8(0, 51, 102, 1.0);
        let c2 = Color::from_rgba8(210, 225, 221, 1.0);

        let mix_c = mix(&c1, &c2, 0.25).into_rgba8();
        assert_eq!(mix_c[0], 158);
        assert_eq!(mix_c[1], 182);
        assert_eq!(mix_c[2], 191);
    }

    #[test]
    fn mix_with_0_5_ratio() {
        let c1 = Color::from_rgba8(0, 51, 102, 1.0);
        let c2 = Color::from_rgba8(210, 225, 221, 1.0);

        let mix_c = mix(&c1, &c2, 0.5).into_rgba8();
        assert_eq!(mix_c[0], 105);
        assert_eq!(mix_c[1], 138);
        assert_eq!(mix_c[2], 162);
    }
}

#[inline]
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha,
        ..color
    }
}

pub struct IcedwaitaColors {
    pub text_color: Color,
    pub accent_bg_color: Color,
    pub accent_fg_color: Color,
    pub accent_color: Color,
    pub destructive_bg_color: Color,
    pub destructive_fg_color: Color,
    pub destructive_color: Color,
    pub success_bg_color: Color,
    pub success_fg_color: Color,
    pub success_color: Color,
    pub warning_bg_color: Color,
    pub warning_fg_color: Color,
    pub warning_color: Color,
    pub error_bg_color: Color,
    pub error_fg_color: Color,
    pub error_color: Color,
    pub window_bg_color: Color,
    pub window_fg_color: Color,
    pub border_color: Color,
    pub view_bg_color: Color,
    pub view_fg_color: Color,
    pub header_bar_bg_color: Color,
    pub header_bar_fg_color: Color,
    pub header_bar_border_color: Color,
    pub header_bar_backdrop_color: Color,
    pub header_bar_shade_color: Color,
    pub card_bg_color: Color,
    pub card_fg_color: Color,
    pub card_shade_color: Color,
    pub dialog_bg_color: Color,
    pub dialog_fg_color: Color,
    pub popover_bg_color: Color,
    pub popover_fg_color: Color,
    pub thumbnail_bg_color: Color,
    pub thumbnail_fg_color: Color,
    pub shade_color: Color,
    pub scrollbar_outline_color: Color,
    pub opaque_el_mix_color: Color,
}

impl IcedwaitaColors {
    pub fn opaque_mixcol(&self) -> Color {
        // println!("{}", (self.window_fg_color.r * 0.15 + self.window_bg_color.r * 0.85));
        // Color {
        //     r: self.window_bg_color.r * 0.15 + self.window_fg_color.r * 0.85,
        //     g: self.window_bg_color.g * 0.15 + self.window_fg_color.g * 0.85,
        //     b: self.window_bg_color.b * 0.15 + self.window_fg_color.b * 0.85,
        //     a: self.window_bg_color.a * 0.15 + self.warning_fg_color.a * 0.85,
        // }

        // let mut fg = self.window_fg_color.clone();
        // fg.a = 1.0;
        // c.a = 1.0;
        // fg.a = 1.0;
        let mut mixc = mix(&self.window_bg_color, &self.window_fg_color, 0.15);
        // let mut finalc = mix(&basics::DARK_5, &mixc, 1. - 0.85);
        // println!("{:?}", mixc);
        // mixc.a = 1.0;
        mixc
    }
}

pub const DARK: IcedwaitaColors = IcedwaitaColors {
    text_color: WHITE,
    accent_bg_color: BLUE_3,
    accent_fg_color: WHITE,
    accent_color: Color::from_rgb(0.47058823529411764, 0.6823529411764706, 0.9294117647058824),
    destructive_bg_color: RED_4,
    destructive_fg_color: WHITE,
    destructive_color: Color::from_rgb(1.0, 0.4823529411764706, 0.38823529411764707),
    success_bg_color: GREEN_5,
    success_fg_color: WHITE,
    success_color: GREEN_1,
    warning_bg_color: Color::from_rgb(0.803921568627451, 0.5764705882352941, 0.03529411764705882),
    warning_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    warning_color: YELLOW_2,
    error_bg_color: RED_4,
    error_fg_color: WHITE,
    error_color: Color::from_rgb(1.0, 0.4823529411764706, 0.38823529411764707),
    window_bg_color: Color::from_rgb(0.1411764705882353, 0.1411764705882353, 0.1411764705882353),
    window_fg_color: WHITE,
    border_color: Color::from_rgba(1., 1., 1., 0.15), //window_fg_color with 15% alpha
    view_bg_color: Color::from_rgb(0.11764705882352941, 0.11764705882352941, 0.11764705882352941),
    view_fg_color: WHITE,
    header_bar_bg_color: Color::from_rgb(0.18823529411764706, 0.18823529411764706, 0.18823529411764706),
    header_bar_fg_color: WHITE,
    header_bar_border_color: WHITE,
    header_bar_backdrop_color: Color::from_rgb(0.1411764705882353, 0.1411764705882353, 0.1411764705882353), //window_bg_color
    header_bar_shade_color: Color::from_rgba(0., 0., 0., 0.36),
    card_bg_color: Color::from_rgba(1., 1., 1., 0.08),
    card_fg_color: WHITE,
    card_shade_color: Color::from_rgba(0., 0., 0., 0.36),
    dialog_bg_color: Color::from_rgb(0.2196078431372549, 0.2196078431372549, 0.2196078431372549),
    dialog_fg_color: WHITE,
    popover_bg_color: Color::from_rgb(0.2196078431372549, 0.2196078431372549, 0.2196078431372549),
    popover_fg_color: WHITE,
    thumbnail_bg_color: Color::from_rgb(0.2196078431372549, 0.2196078431372549, 0.2196078431372549),
    thumbnail_fg_color: WHITE,
    shade_color: Color::from_rgba(0., 0., 0., 0.36),
    scrollbar_outline_color: Color::from_rgba(0., 0., 0., 0.5),
    opaque_el_mix_color: Color::from_rgba(0.52, 0.52, 0.52, 1.),
};

pub const LIGHT: IcedwaitaColors = IcedwaitaColors {
    text_color: DARK_5,
    accent_bg_color: BLUE_3,
    accent_fg_color: WHITE,
    accent_color: BLUE_4,
    destructive_bg_color: RED_3,
    destructive_fg_color: WHITE,
    destructive_color: RED_4,
    success_bg_color: GREEN_4,
    success_fg_color: WHITE,
    success_color: Color::from_rgb(0.10588235294117647, 0.5215686274509804, 0.3254901960784314),
    warning_bg_color: YELLOW_5,
    warning_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    warning_color: Color::from_rgb(0.611764705882353, 0.43137254901960786, 0.011764705882352941),
    error_bg_color: RED_3,
    error_fg_color: WHITE,
    error_color: RED_4,
    window_bg_color: Color::from_rgb(0.980_392_16, 0.980_392_16, 0.980_392_16),
    window_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    border_color: Color::from_rgba(0., 0., 0., 0.15), //window_fg_color with 15% alpha
    view_bg_color: Color::from_rgb(1.0, 1.0, 1.0),
    view_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    header_bar_bg_color: Color::from_rgb(0.921_568_63, 0.921_568_63, 0.921_568_63),
    header_bar_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    header_bar_border_color: Color::from_rgba(0., 0., 0., 0.8),
    header_bar_backdrop_color: Color::from_rgb(0.9803921568627451, 0.9803921568627451, 0.9803921568627451), //window_bg_color
    header_bar_shade_color: Color::from_rgba(0., 0., 0., 0.07),
    card_bg_color: Color::from_rgb(1.0, 1.0, 1.0),
    card_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    card_shade_color: Color::from_rgba(0., 0., 0., 0.07),
    dialog_bg_color: Color::from_rgb(0.9803921568627451, 0.9803921568627451, 0.9803921568627451),
    dialog_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    popover_bg_color: Color::from_rgb(1.0, 1.0, 1.0),
    popover_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    thumbnail_bg_color: Color::from_rgb(1.0, 1.0, 1.0),
    thumbnail_fg_color: Color::from_rgba(0., 0., 0., 0.8),
    shade_color: Color::from_rgba(0., 0., 0., 0.07),
    scrollbar_outline_color: WHITE,
    opaque_el_mix_color: Color::from_rgba(0.52, 0.52, 0.52, 1.0),
};

pub mod basics {
    use iced::Color;

    pub const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
    //"#ffffff"
    pub const BLUE_1: Color = Color::from_rgb(0.6, 0.7568627450980392, 0.9450980392156862);
    //"#99c1f1"
    pub const BLUE_2: Color = Color::from_rgb(0.3843137254901961, 0.6274509803921569, 0.9176470588235294);
    //"#62a0ea"
    pub const BLUE_3: Color = Color::from_rgb(0.20784313725490197, 0.5176470588235295, 0.8941176470588236);
    //"#3584e4"
    pub const BLUE_4: Color = Color::from_rgb(0.10980392156862745, 0.44313725490196076, 0.8470588235294118);
    //"#1c71d8"
    pub const BLUE_5: Color = Color::from_rgb(0.10196078431372549, 0.37254901960784315, 0.7058823529411765);
    //"#1a5fb4"
    pub const GREEN_1: Color = Color::from_rgb(0.5607843137254902, 0.9411764705882353, 0.6431372549019608);
    //"#8ff0a4"
    pub const GREEN_2: Color = Color::from_rgb(0.3411764705882353, 0.8901960784313725, 0.5372549019607843);
    //"#57e389"
    pub const GREEN_3: Color = Color::from_rgb(0.2, 0.8196078431372549, 0.47843137254901963);
    //"#33d17a"
    pub const GREEN_4: Color = Color::from_rgb(0.1803921568627451, 0.7607843137254902, 0.49411764705882355);
    //"#2ec27e"
    pub const GREEN_5: Color = Color::from_rgb(0.14901960784313725, 0.6352941176470588, 0.4117647058823529);
    //"#26a269"
    pub const YELLOW_1: Color = Color::from_rgb(0.9764705882352941, 0.9411764705882353, 0.4196078431372549);
    //"#f9f06b"
    pub const YELLOW_2: Color = Color::from_rgb(0.9725490196078431, 0.8941176470588236, 0.3607843137254902);
    //"#f8e45c"
    pub const YELLOW_3: Color = Color::from_rgb(0.9647058823529412, 0.8274509803921568, 0.17647058823529413);
    //"#f6d32d"
    pub const YELLOW_4: Color = Color::from_rgb(0.9607843137254902, 0.7607843137254902, 0.06666666666666667);
    //"#f5c211"
    pub const YELLOW_5: Color = Color::from_rgb(0.8980392156862745, 0.6470588235294118, 0.0392156862745098);
    //"#e5a50a"
    pub const ORANGE_1: Color = Color::from_rgb(1.0, 0.7450980392156863, 0.43529411764705883);
    //"#ffbe6f"
    pub const ORANGE_2: Color = Color::from_rgb(1.0, 0.6392156862745098, 0.2823529411764706);
    //"#ffa348"
    pub const ORANGE_3: Color = Color::from_rgb(1.0, 0.47058823529411764, 0.0);
    //"#ff7800"
    pub const ORANGE_4: Color = Color::from_rgb(0.9019607843137255, 0.3803921568627451, 0.0);
    //"#e66100"
    pub const ORANGE_5: Color = Color::from_rgb(0.7764705882352941, 0.27450980392156865, 0.0);
    //"#c64600"
    pub const RED_1: Color = Color::from_rgb(0.9647058823529412, 0.3803921568627451, 0.3176470588235294);
    //"#f66151"
    pub const RED_2: Color = Color::from_rgb(0.9294117647058824, 0.2, 0.23137254901960785);
    //"#ed333b"
    pub const RED_3: Color = Color::from_rgb(0.8784313725490196, 0.10588235294117647, 0.1411764705882353);
    //"#e01b24"
    pub const RED_4: Color = Color::from_rgb(0.7529411764705882, 0.10980392156862745, 0.1568627450980392);
    //"#c01c28"
    pub const RED_5: Color = Color::from_rgb(0.6470588235294118, 0.11372549019607843, 0.17647058823529413);
    //"#a51d2d"
    pub const PURPLE_1: Color = Color::from_rgb(0.8627450980392157, 0.5411764705882353, 0.8666666666666667);
    //"#dc8add"
    pub const PURPLE_2: Color = Color::from_rgb(0.7529411764705882, 0.3803921568627451, 0.796078431372549);
    //"#c061cb"
    pub const PURPLE_3: Color = Color::from_rgb(0.5686274509803921, 0.2549019607843137, 0.6745098039215687);
    //"#9141ac"
    pub const PURPLE_4: Color = Color::from_rgb(0.5058823529411764, 0.23921568627450981, 0.611764705882353);
    //"#813d9c"
    pub const PURPLE_5: Color = Color::from_rgb(0.3803921568627451, 0.20784313725490197, 0.5137254901960784);
    //"#613583"
    pub const BROWN_1: Color = Color::from_rgb(0.803921568627451, 0.6705882352941176, 0.5607843137254902);
    //"#cdab8f"
    pub const BROWN_2: Color = Color::from_rgb(0.7098039215686275, 0.5137254901960784, 0.35294117647058826);
    //"#b5835a"
    pub const BROWN_3: Color = Color::from_rgb(0.596078431372549, 0.41568627450980394, 0.26666666666666666);
    //"#986a44"
    pub const BROWN_4: Color = Color::from_rgb(0.5254901960784314, 0.3686274509803922, 0.23529411764705882);
    //"#865e3c"
    pub const BROWN_5: Color = Color::from_rgb(0.38823529411764707, 0.27058823529411763, 0.17254901960784313);
    //"#63452c"
    pub const LIGHT_1: Color = Color::from_rgb(1.0, 1.0, 1.0);
    //"#ffffff"
    pub const LIGHT_2: Color = Color::from_rgb(0.9647058823529412, 0.9607843137254902, 0.9568627450980393);
    //"#f6f5f4"
    pub const LIGHT_3: Color = Color::from_rgb(0.8705882352941177, 0.8666666666666667, 0.8549019607843137);
    //"#deddda"
    pub const LIGHT_4: Color = Color::from_rgb(0.7529411764705882, 0.7490196078431373, 0.7372549019607844);
    //"#c0bfbc"
    pub const LIGHT_5: Color = Color::from_rgb(0.6039215686274509, 0.6, 0.5882352941176471);
    //"#9a9996"
    pub const DARK_1: Color = Color::from_rgb(0.4666666666666667, 0.4627450980392157, 0.4823529411764706);
    //"#77767b"
    pub const DARK_2: Color = Color::from_rgb(0.3686274509803922, 0.3607843137254902, 0.39215686274509803);
    //"#5e5c64"
    pub const DARK_3: Color = Color::from_rgb(0.23921568627450981, 0.2196078431372549, 0.27450980392156865);
    //"#3d3846"
    pub const DARK_4: Color = Color::from_rgb(0.1411764705882353, 0.12156862745098039, 0.19215686274509805);
    //"#241f31"
    pub const DARK_5: Color = Color::from_rgb(0.0, 0.0, 0.0); //"#000000"
}

