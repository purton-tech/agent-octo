use clorinde::{deadpool_postgres, tokio_postgres};
use std::str::FromStr;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clorinde::queries::auth::get_users;
    #[tokio::test]
    async fn load_users() {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = create_pool(&db_url);
        let client = pool.get().await.unwrap();

        // The `all` method returns queried rows collected into a `Vec`
        let users = get_users().bind(&client).all().await.unwrap();
        dbg!(users);
    }
}