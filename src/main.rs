use clap::{App, Arg};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::table;
use dotenvy::dotenv;
use std::env;

mod schema {
    diesel::table! {
        users (id) {
            id -> Integer,
            name -> Text,
            email -> Text,
        }
    }
}

use self::schema::users;

fn main() {
    dotenv().ok();

    let matches = App::new("rust-diesel-cli")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("A simple CLI program with Diesel and PostgreSQL")
        .arg(Arg::with_name("database")
            .short('d')
            .long("database")
            .value_name("DBNAME")
            .help("Sets the database name")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("user")
            .short('u')
            .long("user")
            .value_name("USERNAME")
            .help("Sets the database user")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("password")
            .short('p')
            .long("password")
            .value_name("PASSWORD")
            .help("Sets the database password")
            .takes_value(true)
            .required(true))
        .get_matches();

    let dbname = matches.value_of("database").unwrap();
    let user = matches.value_of("user").unwrap();
    let password = matches.value_of("password").unwrap();

    let database_url = format!("postgres://{}:{}@localhost/{}", user, password, dbname);
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let results = users::table.load::<(i32, String, String)>(&mut connection);
    match results {
        Ok(rows) => {
            for row in rows {
                println!("{:?}", row);
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
