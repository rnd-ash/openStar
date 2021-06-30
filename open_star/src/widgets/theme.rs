use iced::{Color, Vector};




pub struct OpenStarButton{
    accent: Color
}

impl OpenStarButton {
    pub fn new(accent: Color) -> Self {
        Self { accent }
    }
}

impl iced::button::StyleSheet for OpenStarButton {

    fn active(&self) -> iced::button::Style {
        match super::using_dark_theme() {
            true => {
                iced::button::Style {
                    shadow_offset: Vector::new(0.0, 0.0),
                    background: self.accent.into(),
                    border_radius: 5f32,
                    border_width: 0f32,
                    border_color: Color::TRANSPARENT,
                    text_color: *super::WHITE,
                }
            },
            false => {
                iced::button::Style {
                    shadow_offset: Default::default(),
                    background: (self.accent).into(),
                    border_radius: 5f32,
                    border_width: 0f32,
                    border_color: Color::TRANSPARENT,
                    text_color: *super::WHITE,
                }
            }
        }
    }
}

pub struct RadioBtn{}

impl iced::radio::StyleSheet for RadioBtn {
    fn active(&self) -> iced::radio::Style {
        iced::radio::Style {
            background: (*super::WHITE).into(),
            dot_color: (*super::BLACK).into(),
            border_width: 1.0,
            border_color: (*super::BLACK).into(),
        }
    }

    fn hovered(&self) -> iced::radio::Style {
        self.active()
    }
}