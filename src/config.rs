#[cfg(not(debug_assertions))]
pub static DB_PARAM: &str = "postgresql://admin:admin@localhost:5432/postgres";
#[cfg(debug_assertions)]
pub static DB_PARAM: &str = "postgresql://admin:admin@localhost:5432/postgres";
