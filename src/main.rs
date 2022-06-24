use fltk::{app::{self, event_key}, input::Input, enums::*, prelude::*, window::Window, text::{TextAttr, TextDisplay}, button::RoundButton};
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


fn main() {

    let connection = match create_db() {
        Ok(connection) => connection,
        Err(_) => {panic!("failed to connect")},
    };

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 100, "Hello from rust");
    let mut input = Input::new(0, 0, 400 , 100, "");

    wind.end();
    wind.show();
    wind.handle(move |win, event| {
        match event {
            Event::KeyUp => {
                if event_key() == Key::Enter {
                    save_note(input.value(), &connection).unwrap();
                    input.set_value("");
                }
                if event_key() == Key::Tab {

                    win.clear();

    let mut scroll = group::Scroll::default().with_size(600, 350);
    let mut pack = group::Pack::default()
        .with_size(580, 350)
        .center_of(&scroll);
    pack.end();
    scroll.end();


                    let notes = get_notes(&connection).unwrap();
                    let mut pack = pack.clone();

                    for memo in notes {
                    let joo = memo.memo.clone().to_string();
                    let item = RoundButton::new(0, 0, 50, 20, "").with_label(&joo);
                    pack.add(&item);
                    }
                    win.add(&pack);
                    win.redraw();
                }
                true


            },
            //TODO: hjkl movement, todo valinta
            _ => false,
        }
    });

    app.run().unwrap();
}
