use fltk::{app::{self, event_key}, frame::Frame, button::Button ,input::Input, enums::*, prelude::*, window::{Window, DoubleWindow}, button::RoundButton};
use fltk::*;

mod ops;
mod models;

static WINDOW_WIDTH: i32 = 400;

fn main() {
    let mut showing_todos = false;
    let mut focused_todo: i32 = 0;

    let connection = match ops::create_db() {
        Ok(connection) => connection,
        Err(_) => {panic!("failed to connect")},
    };

    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut wind = Window::new(0, 0, WINDOW_WIDTH, 50, "Kiire").center_screen();

    let mut input = draw_note_input(& mut wind);

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
                        input = draw_note_input(win);
                        return true;
                    }

                    win.resize(100, 100, WINDOW_WIDTH, 800);
                    draw_notes(win, &notes, focused_todo);
                    showing_todos = true;
                }

                if event_key() == Key::from_char('j') {
                    if showing_todos && focused_todo + 1 <= (notes.len() - 1).try_into().unwrap() {
                        focused_todo = focused_todo + 1;
                        draw_notes(win, &notes, focused_todo);
                    }
                }

                if event_key() == Key::from_char('k') {
                    if showing_todos && focused_todo - 1 >= 0 {
                        focused_todo = focused_todo - 1;
                        draw_notes(win, &notes, focused_todo);
                    }
                }

                if event_key() == Key::from_char('d') {
                    //TODO: automatically reset d_was_pressed if not pressed again in second or two
                    if showing_todos {
                        if !d_was_pressed {
                            d_was_pressed = true;
                        } else {
                            d_was_pressed = false;
                            ops::delete_note(notes[focused_todo as usize].id, &connection);
                            notes.remove(focused_todo as usize);
                            focused_todo = 0;
                            draw_notes(win, &notes, focused_todo);
                        }
                    }
                }



                return true


            },
            //TODO: hjkl movement, todo valinta
            _ => false,
        }
    });

    fn draw_note_input(win: & mut DoubleWindow) -> fltk::input::Input {
        let mut input = render_input();
        win.add(&input);
        win.redraw();
        input.take_focus();

        return input;
    }

    fn render_input() -> fltk::input::Input {
        return Input::new(0, 0, WINDOW_WIDTH, 50, "");
    }

    fn draw_notes(win: & mut DoubleWindow, notes: &Vec<models::MemoNote>, focused_note: i32) {
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

    app.run().unwrap();
}

