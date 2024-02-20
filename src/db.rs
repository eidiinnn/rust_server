use postgres::{Client, NoTls};
use time::OffsetDateTime;

pub struct DB {
    client: Client,
}

impl DB {
    pub fn new() -> DB {
        let client = Client::connect("postgresql://admin:admin@localhost:5432/postgres", NoTls);

        match client {
            Ok(client) => {
                println!("Has connection with Postgres!");
                DB { client }
            }
            Err(error) => {
                println!("error to connect to database: {:?}", error);
                panic!("impossible to connect on Postgres database");
            }
        }
    }

    pub fn create_message_table(&mut self) {
        println!("creating the table Person...");
        let result = self.client.batch_execute(
            "
         CREATE TABLE IF NOT EXISTS message (
            id  SERIAL PRIMARY KEY,
            message TEXT NOT NULL,
            data TEXT NOT NULL
         )
      ",
        );

        match result {
            Ok(()) => {
                println!("Successful Message Table create!");
                self.insert_into_messages(String::from("test"))
            }
            Err(error) => {
                println!("Error to create the table Message: {:?}", error)
            }
        }
    }

    pub fn insert_into_messages(&mut self, message: String,) {
        let result = self.client.execute(
            "INSERT INTO message (message, data) VALUES ($1, $2)",
            &[&message, &OffsetDateTime::now_utc().to_string()],
        );

       match result {
        Ok(_) => println!("the message {:?}", message),
        Err(error) => println!("error to print the message '{:?}': {:?}", message, error)
       }
    }
}
