use rusqlite::{params, Connection};
use crate::models::MemoNote;

    pub fn create_db() -> rusqlite::Result<Connection> {
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

    pub fn get_notes(connection: &Connection) -> rusqlite::Result<Vec<MemoNote>> {
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

    pub fn save_note(note: String, connection: &Connection) -> rusqlite::Result<()>{

        let result = connection.execute(
            "INSERT INTO memos (memo) VALUES (?1)",
            params![note],
        )?;

        return Ok(());
    }


    pub fn delete_note(note_id: i64, connection: &Connection) -> rusqlite::Result<()>{

        let result = connection.execute(
            "delete from memos where id = (?1)",
            params![note_id],
        )?;

        return Ok(());
    }
