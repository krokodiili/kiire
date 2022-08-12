use fltk::{prelude::*, button::Button ,input::Input, enums::*, window::{Window, DoubleWindow}};
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
    input.take_focus().unwrap();
    return input;
}

pub fn render_input() -> fltk::input::Input {
    return Input::new(0, 0, WINDOW_WIDTH, 50, "");
}

pub fn draw_notes(win: & mut DoubleWindow, notes: &Vec<models::MemoNote>, focused_note: i32) {
                win.clear();

                let height_by_note_amount = notes.len() as i32 * 50 + 100;

                let mut scroll = group::Scroll::default()
                    .with_size(WINDOW_WIDTH, height_by_note_amount);

                    scroll.set_scrollbar_size(10);

                let mut pack = group::Pack::default()
                    .with_size(WINDOW_WIDTH - 30, height_by_note_amount)
                    .with_pos(10,10);

                pack.set_spacing(10);
                pack.end();
                scroll.end();


                let mut pack = pack.clone();

                for (i, note) in notes.iter().enumerate() {
                let mut label = note.memo.clone().to_string();

                let is_focused_note =  i == focused_note.try_into().unwrap();
                let is_oversized_label = label.len() > 50;

                if is_oversized_label && !is_focused_note {
                    label = label[..50].to_string();
                    label.push_str("...");
                }


                let mut item = Button::new(0, 0, 50, 50, "").with_label(&label).with_align(Align::Wrap);

                if is_oversized_label && is_focused_note {
                    item.set_size(50, 150);
                }

                item.set_label_font(Font::HelveticaBold);
                item.set_frame(FrameType::PlasticUpBox);
                item.set_color(Color::from_rgb(255, 0, 0));
                item.set_label_type(LabelType::Embossed);

                if is_focused_note {
                    item.take_focus();

                    let offset = if  i > 3 {
                        100
                    } else {
                        0
                    };

                    let scroll_amount = i as i32 * 50 - offset;
                    scroll.scroll_to(0, scroll_amount);
                }

                pack.add(&item);

                }


                scroll.add(&pack);
                win.add(&scroll);
                win.redraw();

}


