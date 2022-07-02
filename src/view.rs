use fltk::{app::{self, event_key}, frame::Frame, button::Button ,input::Input, enums::*, prelude::*, window::{Window, DoubleWindow}, button::RoundButton};
use fltk::*;
use crate::models;

static WINDOW_WIDTH: i32 = 400;

pub fn draw_window() -> DoubleWindow {
    let wind = Window::new(0, 0, WINDOW_WIDTH, 50, "Kiire").center_screen();
    return wind;
}

pub fn draw_note_input(win: & mut DoubleWindow) -> fltk::input::Input {
    let mut input = render_input();
    win.add(&input);
    win.redraw();
    input.take_focus();

    return input;
}

pub fn render_input() -> fltk::input::Input {
    return Input::new(0, 0, WINDOW_WIDTH, 50, "");
}

pub fn draw_notes(win: & mut DoubleWindow, notes: &Vec<models::MemoNote>, focused_note: i32) {
                win.clear();
                let mut image_frame = Frame::default().with_size(WINDOW_WIDTH,800);
                let flamingo = image::JpegImage::load("flamingo.jpg").unwrap();
                image_frame.set_image(Some(flamingo));

                win.add(&image_frame);


                let scroll = group::Scroll::default().size_of_parent();
                let mut pack = group::Pack::default()
                    .with_size(WINDOW_WIDTH - 20, 350)
                    .with_pos(10, 10);

                pack.set_spacing(10);
                pack.end();
                scroll.end();


                let mut pack = pack.clone();

                for (i, note) in notes.iter().enumerate() {
                let label = note.memo.clone().to_string();

                let mut item = Button::new(0, 0, 50, 50, "").with_label(&label);

                item.set_label_font(Font::HelveticaBold);
                item.set_frame(FrameType::PlasticUpBox);
                item.set_color(Color::from_rgb(255, 0, 0));
                item.set_label_type(LabelType::Embossed);

                if i == focused_note.try_into().unwrap() {
                    item.take_focus();
                }

                pack.add(&item);
                }

                win.add(&pack);
                win.redraw();
}


