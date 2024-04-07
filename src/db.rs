use std::sync::{Mutex, MutexGuard};
use postgres::{Client, NoTls};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;

lazy_static! {
   static ref DB_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
        Mutex::new(
            Client::connect("postgresql://admin:admin@localhost:5432/postgres", NoTls).unwrap(),
        )
    });
}


pub fn get_connection() -> MutexGuard<'static, Client> {
   let client = DB_CLIENT.lock().unwrap();
   client
}