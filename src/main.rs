use fltk::{
    app::{self, event_key},
    enums::*,
    prelude::*,
};
use std::cmp;

mod models;
mod ops;
mod view;

static WINDOW_WIDTH: i32 = 400;
static NOTE_HEIGHT: i32 = 50;
static BIG_NOTE_HEIGHT: i32 = NOTE_HEIGHT * 3;

fn main() {
    let mut showing_todos = false;
    let mut focused_todo: i32 = 0;

    let connection = match ops::create_db() {
        Ok(connection) => connection,
        Err(_) => {
            panic!("failed to connect")
        }
    };

    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut wind = view::draw_window();
    let mut input = view::draw_note_input(&mut wind);

    let mut d_was_pressed = false;

    let mut notes = ops::get_notes(&connection).unwrap();

    wind.end();
    wind.show();

    wind.handle(move |win, event| {
        match event {
            Event::KeyUp => {
                if event_key() == Key::Enter {
                    ops::save_note(input.value(), &connection).unwrap();
                    notes = ops::get_notes(&connection).unwrap();

                    input.set_value("");
                }
                if event_key() == Key::Tab {
                    if showing_todos == true {
                        win.resize(100, 100, WINDOW_WIDTH, 50);
                        win.clear();
                        showing_todos = false;
                        input = view::draw_note_input(win);
                        return true;
                    }

                    let dynamic_height = notes.len() as i32 * NOTE_HEIGHT + BIG_NOTE_HEIGHT;
                    let window_size = cmp::min(dynamic_height, 500);

                    win.resize(100, 100, WINDOW_WIDTH, window_size);
                    view::draw_notes(win, &notes, focused_todo);
                    showing_todos = true;
                }

                if event_key() == Key::from_char('j') {
                    if showing_todos && focused_todo + 1 <= (notes.len() - 1).try_into().unwrap() {
                        focused_todo = focused_todo + 1;
                        view::draw_notes(win, &notes, focused_todo);
                    }
                }

                if event_key() == Key::from_char('k') {
                    if showing_todos && focused_todo - 1 >= 0 {
                        focused_todo = focused_todo - 1;
                        view::draw_notes(win, &notes, focused_todo);
                    }
                }

                if event_key() == Key::from_char('d') {
                    //TODO: automatically reset d_was_pressed if not pressed again in second or two
                    if showing_todos {
                        if !d_was_pressed {
                            d_was_pressed = true;
                        } else {
                            d_was_pressed = false;
                            ops::delete_note(notes[focused_todo as usize].id, &connection).unwrap();
                            notes.remove(focused_todo as usize);
                            focused_todo = minus_one_or_zero(focused_todo);
                            view::draw_notes(win, &notes, focused_todo);
                        }
                    }
                }

                return true;
            }
            _ => false,
        }
    });

    app.run().unwrap();
}

fn minus_one_or_zero(current: i32) -> i32 {
    if current > 0 {
        return current - 1;
    } else {
        return 0;
    }
}
