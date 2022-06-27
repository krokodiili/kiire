use fltk::{app::{self, event_key}, input::Input, enums::*, prelude::*, window::{Window, DoubleWindow}, text::{TextAttr, TextDisplay}, button::RoundButton};
use fltk::{enums::*, prelude::*, *};
use std::io::prelude::*;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use rusqlite::{params, Connection};

struct MemoNote {
    id: i64,
    memo: String
}

fn create_db() -> rusqlite::Result<Connection> {
    let connection = Connection::open("./db.db3")?;

    match connection.execute(
            "CREATE TABLE memos (
                id              INTEGER PRIMARY KEY,
                memo            TEXT NOT NULL
            )",
            [],
        ) {
        Ok(_) => {},
        Err(_) => println!("table already created"),
    };

    return Ok(connection);
}

fn get_notes(connection: &Connection) -> rusqlite::Result<Vec<MemoNote>> {
    let mut statement = connection.prepare("SELECT * from memos")?;
    let mut memos = Vec::new();

    let memo_rows = statement.query_map([], |row| {
        Ok(MemoNote {
            id: row.get(0)?,
            memo: row.get(1)?,
        })
    })?;

    for memo in memo_rows {
        memos.push(memo.unwrap());
    }

    return Ok(memos);

}

fn save_note(note: String, connection: &Connection) -> rusqlite::Result<()>{

    let result = connection.execute(
        "INSERT INTO memos (memo) VALUES (?1)",
        params![note],
    )?;

    return Ok(());
}


fn delete_note(note_id: i64, connection: &Connection) -> rusqlite::Result<()>{

    let result = connection.execute(
        "delete from memos where id = (?1)",
        params![note_id],
    )?;

    return Ok(());
}



fn main() {
    let mut showing_todos = false;
    let mut focused_todo: i32 = 0;

    let connection = match create_db() {
        Ok(connection) => connection,
        Err(_) => {panic!("failed to connect")},
    };

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 100, "Hello from rust");
    let mut input = Input::new(0, 0, 400 , 100, "");
    let mut d_was_pressed = false;

    let mut notes = get_notes(&connection).unwrap();

    wind.end();
    wind.show();

    wind.handle(move |win, event| {
        match event {
            Event::KeyUp => {
                if event_key() == Key::Enter {
                    save_note(input.value(), &connection).unwrap();
                    notes = get_notes(&connection).unwrap();

                    input.set_value("");
                }
                if event_key() == Key::Tab {
                    if showing_todos == true {

                        win.clear();
                        showing_todos = false;
                    input = draw_note_input(win);

                                                return true;
                    }

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
                            delete_note(notes[focused_todo as usize].id, &connection);
                            notes.remove(focused_todo as usize);
                            focused_todo = 0;
                            draw_notes(win, &notes, focused_todo);
                            println!("delete");
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
        let mut input = Input::new(0, 0, 400 , 100, "");
        win.add(&input);
        win.redraw();
        input.take_focus();

        return input;
    }

    fn draw_notes(win: & mut DoubleWindow, notes: &Vec<MemoNote>, focused_note: i32) {
                    win.clear();
                    let mut scroll = group::Scroll::default().with_size(600, 350);
                    let mut pack = group::Pack::default()
                        .with_size(580, 350)
                        .center_of(&scroll);
                    pack.end();
                    scroll.end();


                    let mut pack = pack.clone();

                    for (i, note) in notes.iter().enumerate() {
                    let label = note.memo.clone().to_string();
                    let mut item = RoundButton::new(0, 0, 50, 20, "").with_label(&label);

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
