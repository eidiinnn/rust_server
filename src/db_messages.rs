use postgres::Client;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct MessageStruct {
    message: String,
    date: String,
}

pub fn create_message_table(client: &mut Client) {
    println!("creating the table Person...");
    let result = client.batch_execute(
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
        }
        Err(error) => {
            println!("Error to create the table Message: {:?}", error)
        }
    }
}

pub fn get_last_message(client: &mut Client) -> Result<MessageStruct, postgres::Error> {
    match client.query("SELECT * FROM message ORDER BY id DESC LIMIT 1", &[]) {
        Ok(result) => {
            let message = result[0].get(1);
            let date = result[0].get(2);
            Result::Ok(MessageStruct { message, date })
        }
        Err(error) => Result::Err(error),
    }
}

pub fn insert_into_messages(client: &mut Client, message: String) -> Result<(), postgres::Error> {
    let result = client.execute(
        "INSERT INTO message (message, data) VALUES ($1, $2)",
        &[&message, &OffsetDateTime::now_utc().to_string()],
    );

    match result {
        Ok(_) => {
            println!("The message {:?}", message);
            return Ok(());
        }
        Err(error) => {
            println!("Error to print the message '{:?}': {:?}", message, error);
            return Err(error);
        }
    }
}
