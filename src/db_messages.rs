use postgres::Client;
use time::OffsetDateTime;

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

pub fn insert_into_messages(client: &mut Client, message: String) {
    let result = client.execute(
        "INSERT INTO message (message, data) VALUES ($1, $2)",
        &[&message, &OffsetDateTime::now_utc().to_string()],
    );

    match result {
        Ok(_) => println!("The message {:?}", message),
        Err(error) => println!("Error to print the message '{:?}': {:?}", message, error),
    }
}
