use crate::models;
use fltk::*;
use fltk::{
    button::Button,
    enums::*,
    input::Input,
    prelude::*,
    window::{DoubleWindow, Window},
};
use std::cmp;

static WINDOW_WIDTH: i32 = 400;
static HEIGHT_OFFSET_FOR_FOCUSED_NOTE: i32 = 150;

pub fn draw_window() -> DoubleWindow {
    let wind = Window::new(0, 0, WINDOW_WIDTH, 50, "Kiire").center_screen();
    return wind;
}

pub fn draw_note_input(win: &mut DoubleWindow) -> fltk::input::Input {
    let mut input = render_input();
    win.add(&input);
    win.redraw();
    input.take_focus().unwrap();
    return input;
}

pub fn draw_notes(win: &mut DoubleWindow, notes: &Vec<models::MemoNote>, focused_note: i32) {
    win.clear();

    //Duplicate in main, size of parent does not work here for some reason
    let height_by_note_amount = notes.len() as i32 * 50 + HEIGHT_OFFSET_FOR_FOCUSED_NOTE;
    let container_height = cmp::min(height_by_note_amount, 500);

    let mut scroll = render_scroll_container(container_height);
    let mut pack = render_notes_container(container_height);
    let mut fill_container_with_notes = || {
        for (i, note) in notes.iter().enumerate() {
            let is_focused_note = i == focused_note.try_into().unwrap();

            let label = if is_focused_note {
                note.memo.clone().to_string()
            } else {
                truncate_oversize_label(&note.memo.clone().to_string())
            };

            let mut note = render_note(&label, is_focused_note);

            if is_focused_note {
                note.take_focus().unwrap();

                let offset = if i > 3 { 100 } else { 0 };

                let scroll_amount = i as i32 * 50 - offset;
                scroll.scroll_to(0, scroll_amount);
            }

            pack.add(&note);
        }
    };

    fill_container_with_notes();

    scroll.add(&pack);
    win.add(&scroll);
    win.redraw();
}

fn render_input() -> fltk::input::Input {
    return Input::new(0, 0, WINDOW_WIDTH, 50, "");
}

fn render_scroll_container(height: i32) -> group::Scroll {
    let mut scroll = group::Scroll::default().with_size(WINDOW_WIDTH, height);
    scroll.set_scrollbar_size(10);

    return scroll;
}

fn render_notes_container(height: i32) -> group::Pack {
    let mut pack = group::Pack::default()
        .with_size(WINDOW_WIDTH - 30, height)
        .with_pos(10, 10);

    pack.set_spacing(10);
    return pack;
}

fn render_note(label: &String, is_focused_note: bool) -> Button {
    let mut note = Button::new(0, 0, 50, 50, "")
        .with_label(&label)
        .with_align(Align::Wrap);

    if is_focused_note {
        note.set_size(50, 150);
    }

    note.set_label_font(Font::HelveticaBold);
    note.set_frame(FrameType::PlasticUpBox);
    note.set_color(Color::from_rgb(255, 0, 0));
    note.set_label_type(LabelType::Embossed);

    return note;
}

fn truncate_oversize_label(label_to_trunc: &String) -> String {
    let mut label = label_to_trunc.clone();
    let is_oversized_label = label.len() > 50;

    if is_oversized_label {
        label = label[..50].to_string();
        label.push_str("...");
    }

    return label;
}
