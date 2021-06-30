use iced::{Color, Text};
mod theme;
use lazy_static::lazy_static;

use self::theme::RadioBtn;


static mut USE_DARK_THEME: bool = false;

pub fn using_dark_theme() -> bool {
    unsafe { USE_DARK_THEME }
}
pub fn set_dark_theme() {
    unsafe { USE_DARK_THEME = true }
}
pub fn set_light_theme() {
    unsafe { USE_DARK_THEME = false }
}

const PADDING: u16 = 5;
const ELEMENT_TEXT_SIZE: u16 = 20;
const SPACING: u16 = 5;

lazy_static! {
    pub static ref BLACK: Color = Color::from_rgb8(0, 0, 0);
    pub static ref WHITE: Color = Color::from_rgb8(255, 255, 255);
    pub static ref LIGHT_GREY: Color = Color::from_rgb8(164, 170, 174);
    pub static ref DARK_GREY: Color = Color::from_rgb8(130, 138, 143);
    pub static ref RED: Color = Color::from_rgb8(139, 0, 0);
    pub static ref GREEN: Color = Color::from_rgb8(0, 139, 0);
    pub static ref BLUE: Color = Color::from_rgb8(1,36,100);
}

pub fn create_button<'a, T: Clone>(state: &'a mut iced::button::State, text: &str) -> iced::button::Button<'a, T> {
    iced::Button::new(state, Text::new(text).size(ELEMENT_TEXT_SIZE)).padding(PADDING).style(theme::OpenStarButton::new(*BLUE))
}

pub fn main_text<'a, S: Into<String>>(text: S) -> iced::Text {
    iced::Text::new(text).size(ELEMENT_TEXT_SIZE)
}

pub fn heading_text<'a, S: Into<String>>(text: S) -> iced::Text {
    main_text(text).size((ELEMENT_TEXT_SIZE as f32 *1.5) as u16)
}

pub fn subheading_text<'a, S: Into<String>>(text: S) -> iced::Text {
    main_text(text).size((ELEMENT_TEXT_SIZE as f32 *1.25) as u16)
}

pub fn row<'a, T>() -> iced::Row<'a, T> {
    iced::Row::new().spacing(SPACING).align_items(iced::Align::Center)
}

pub fn column<'a, T>() -> iced::Column<'a, T> {
    iced::Column::new().spacing(SPACING)
}

pub fn radio_btn<Msg: Clone, V, F>(
    value: V,
    label: impl Into<String>,
    selected: Option<V>,
    f: F,
) -> iced::radio::Radio<Msg>
where
    V: Eq + Copy,
    F: 'static + Fn(V) -> Msg,
{
    iced::radio::Radio::new(value, label, selected, f).style(RadioBtn{}).text_size(ELEMENT_TEXT_SIZE).spacing(SPACING)
}