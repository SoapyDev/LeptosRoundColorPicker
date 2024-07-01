use leptos::web_sys::js_sys::Math;
use leptos::{RwSignal, SignalGet,SignalWith};
use std::f32::consts::PI;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub(crate) struct Color {
    pub(crate) r: RwSignal<u8>,
    pub(crate) g: RwSignal<u8>,
    pub(crate) b: RwSignal<u8>,
    pub(crate) x: RwSignal<i32>,
    pub(crate) y: RwSignal<i32>,
    pub(crate) degree: RwSignal<f64>,
}

impl Color {
    pub(crate) fn new(r: u8, g: u8, b: u8, x: i32, y: i32, d: f64) -> Self {
        Self {
            r: RwSignal::new(r),
            g: RwSignal::new(g),
            b: RwSignal::new(b),
            x: RwSignal::new(x),
            y: RwSignal::new(y),
            degree: RwSignal::new(d),
        }
    }

    pub(crate) fn get_style(&self) -> String {
        let red = Self::get_hex_value(self.r.get());
        let green = Self::get_hex_value(self.g.get());
        let blue = Self::get_hex_value(self.b.get());

        format!("background-color: #{}{}{};", red, green, blue)
    }
    fn get_hex_value(value: u8) -> String {
        let hex = format!("{value:X}");
        let size = hex.len();

        if size > 1 {
            hex
        } else {
            format!("0{}", hex)
        }
    }

    pub(crate) fn calculate_color(
        color: RwSignal<Color>,
        mouse_x: f64,
        mouse_y: f64,
        radius: f64,
    ) -> (u8, u8, u8) {
        let colors = color.with(|c|c.get_gradient());
        let mut degree = ((PI / 2.0) as f64 - Math::atan2(mouse_y, mouse_x)) / PI as f64 * 180.0;
        if degree < 0.0 {
            degree += 360.0;
        }
        degree = degree / 360.0 * (colors.len() - 1) as f64;

        let numeral_color_1 = (degree as usize) % colors.len();
        let numeral_color_2 = (numeral_color_1 + 1) % colors.len();

        let color_1 = colors.get(numeral_color_1).unwrap();
        let color_2 = colors.get(numeral_color_2).unwrap();

        let weight_2 = degree - Math::floor(degree);
        let weight_1 = 1.0 - weight_2;

        let new_color = (
            color_1.0 as f64 * weight_1 + color_2.0 as f64 * weight_2,
            color_1.1 as f64 * weight_1 + color_2.1 as f64 * weight_2,
            color_1.2 as f64 * weight_1 + color_2.2 as f64 * weight_2,
        );

        let color_grey = if radius < 0.8 { radius / 0.8 } else { 1.0 };
        let grey_weight = 1.0 - color_grey;

        let red = Math::round(new_color.0 * color_grey + 128.0 * grey_weight);
        let green = Math::round(new_color.1 * color_grey + 128.0 * grey_weight);
        let blue = Math::round(new_color.2 * color_grey + 128.0 * grey_weight);

        (red as u8, green as u8, blue as u8)
    }
    pub(crate) fn get_position(&self) -> String {
        format!("top: {}px; left: {}px;", self.y.get(), self.x.get())
    }
    pub(crate) fn get_degree(&self) -> f64 {
        self.degree.get()
    }

    pub(crate) fn get_gradient(&self) -> Vec<(u8, u8, u8)> {
        vec![
            (0xe4, 0x3f, 0x00),
            (0xfa, 0xe4, 0x10),
            (0x55, 0xcc, 0x3b),
            (0x09, 0xad, 0xff),
            (0x6b, 0x0e, 0xfd),
            (0xe7, 0x0d, 0x86),
            (0xe4, 0x3f, 0x00),
        ]
    }
}
