extern crate rusqlite;
use rusqlite::{params, Connection, Result};
use std::io;

enum Command {
    AddBook,
    ShowInventory,
    EditBook,
    RemoveBook,
    Exit,
    Invalid,
}

fn add_book(conn: &Connection, title: &str, author: &str, year: u32, genre: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO inventory (title, author, year, genre) VALUES (?1, ?2, ?3, ?4)",
        params![title, author, year, genre],
    )?;
    println!("Book added to inventory!");
    Ok(())
}

fn show_inventory(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, author, year, genre FROM inventory")?;
    let book_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, u32>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;

    for book in book_iter {
        let (id, title, author, year, genre) = book?;
        println!("ID: {}, Title: {}, Author: {}, Year: {}, Genre: {}", id, title, author, year, genre);
    }

    Ok(())
}

fn edit_book(conn: &Connection, id: i32, new_title: &str, new_author: &str, new_year: u32, new_genre: &str) -> Result<()> {
    conn.execute(
        "UPDATE inventory SET title = ?2, author = ?3, year = ?4, genre = ?5 WHERE id = ?1",
        params![id, new_title, new_author, new_year, new_genre],
    )?;
    println!("Book information updated!");
    Ok(())
}

fn remove_book(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM inventory WHERE id = ?", params![id])?;
    println!("Book removed from inventory!");
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("books_inventory.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS inventory (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            year INTEGER NOT NULL,
            genre TEXT NOT NULL
        )",
        [],
    )?;

    loop {
        println!("Book Inventory CLI:");
        println!("1. Add a new book");
        println!("2. Show book inventory");
        println!("3. Edit book information");
        println!("4. Remove a book from inventory");
        println!("5. Exit the program");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let cmd = match input.trim().parse::<u32>() {
            Ok(1) => Command::AddBook,
            Ok(2) => Command::ShowInventory,
            Ok(3) => Command::EditBook,
            Ok(4) => Command::RemoveBook,
            Ok(5) => Command::Exit,
            _ => Command::Invalid,
        };

        match cmd {
            Command::AddBook => {
                println!("Enter book title: ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read title");
                let title = title.trim();
        
                println!("Enter author's name: ");
                let mut author = String::new();
                io::stdin().read_line(&mut author).expect("Failed to read author");
                let author = author.trim();
        
                println!("Enter year of publication: ");
                let mut year_str = String::new();
                io::stdin().read_line(&mut year_str).expect("Failed to read year of publication");
                let year: u32 = year_str.trim().parse().expect("Please enter a valid number for the year");
        
                println!("Enter genre of the book: ");
                let mut genre = String::new();
                io::stdin().read_line(&mut genre).expect("Failed to read genre");
                let genre = genre.trim();
        
                add_book(&conn, title, author, year, genre)?;
            },
            Command::ShowInventory => {
                show_inventory(&conn)?;
            },
            Command::EditBook => {
                println!("Enter the ID of the book to edit: ");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read ID");
                let id: i32 = id_input.trim().parse().expect("Please enter a valid number for ID");
        
                println!("Enter new title: ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read title");
                let title = title.trim();
        
                println!("Enter new author's name: ");
                let mut author = String::new();
                io::stdin().read_line(&mut author).expect("Failed to read author");
                let author = author.trim();
        
                println!("Enter new year of publication: ");
                let mut year_str = String::new();
                io::stdin().read_line(&mut year_str).expect("Failed to read year of publication");
                let year: u32 = year_str.trim().parse().expect("Please enter a valid number for the year");
        
                println!("Enter new genre: ");
                let mut genre = String::new();
                io::stdin().read_line(&mut genre).expect("Failed to read genre");
                let genre = genre.trim();
        
                edit_book(&conn, id, title, author, year, genre)?;
            },
            Command::RemoveBook => {
                println!("Enter the ID of the book to remove: ");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read ID");
                let id: i32 = id_input.trim().parse().expect("Please enter a valid number for ID");
        
                remove_book(&conn, id)?;
            },
            Command::Exit => {
                println!("Exiting the program.");
                break;
            },
            Command::Invalid => {
                println!("Invalid command, please try again.");
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::NO_PARAMS;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE inventory (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                year INTEGER NOT NULL,
                genre TEXT NOT NULL
            )",
            NO_PARAMS,
        ).unwrap();
        conn
    }

    #[test]
    fn test_add_book() {
        let conn = setup_db();
        add_book(&conn, "The Rust Programming Language", "Steve Klabnik and Carol Nichols", 2018, "Programming").unwrap();

        let mut stmt = conn.prepare("SELECT title, author, year, genre FROM inventory WHERE title = ?").unwrap();
        let book_iter = stmt.query_map(params!["The Rust Programming Language"], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u32>(2)?,
                row.get::<_, String>(3)?,
            ))
        }).unwrap();

        for book in book_iter {
            let (title, author, year, genre) = book.unwrap();
            assert_eq!(title, "The Rust Programming Language");
            assert_eq!(author, "Steve Klabnik and Carol Nichols");
            assert_eq!(year, 2018);
            assert_eq!(genre, "Programming");
        }
    }

    #[test]
    fn test_edit_book() {
        let conn = setup_db();
        add_book(&conn, "The Rust Book", "Unknown", 2021, "Education").unwrap();

        let book_id = conn.last_insert_rowid();
        edit_book(&conn, book_id as i32, "The Rust Programming Language", "Steve Klabnik and Carol Nichols", 2018, "Programming").unwrap();

        let mut stmt = conn.prepare("SELECT title, author, year, genre FROM inventory WHERE id = ?").unwrap();
        let book_iter = stmt.query_map(params![book_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u32>(2)?,
                row.get::<_, String>(3)?,
            ))
        }).unwrap();

        for book in book_iter {
            let (title, author, year, genre) = book.unwrap();
            assert_eq!(title, "The Rust Programming Language");
            assert_eq!(author, "Steve Klabnik and Carol Nichols");
            assert_eq!(year, 2018);
            assert_eq!(genre, "Programming");
        }
    }

    #[test]
    fn test_remove_book() {
        let conn = setup_db();
        add_book(&conn, "Rust for Beginners", "Anonymous", 2020, "Learning").unwrap();

        let book_id = conn.last_insert_rowid();
        remove_book(&conn, book_id as i32).unwrap();

        let mut stmt = conn.prepare("SELECT title FROM inventory WHERE id = ?").unwrap();
        let result = stmt.query_row(params![book_id], |row| row.get::<_, String>(0));

        assert!(result.is_err(), "Book should have been removed from inventory");
    }

    #[test]
    fn test_show_inventory() {
        let conn = setup_db();
        add_book(&conn, "Rust in Action", "Tim McNamara", 2020, "Technical").unwrap();
        add_book(&conn, "Programming Rust", "Jim Blandy and Jason Orendorff", 2017, "Technical").unwrap();

        let mut stmt = conn.prepare("SELECT title FROM inventory").unwrap();
        let book_iter = stmt.query_map(NO_PARAMS, |row| {
            Ok(row.get::<_, String>(0)?)
        }).unwrap();

        let mut titles = vec![];
        for title_result in book_iter {
            titles.push(title_result.unwrap());
        }
        assert_eq!(titles.len(), 2);
        assert!(titles.contains(&"Rust in Action".to_string()));
        assert!(titles.contains(&"Programming Rust".to_string()));
    }
}
