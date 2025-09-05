use crate::db::{Database, DatabaseLike, User};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

static UNDERLYING_DB: OnceCell<Mutex<HashMap<i32, User>>> = OnceCell::new();

impl DatabaseLike for Database {

    async fn connect() -> Result<(), String> {
        match UNDERLYING_DB.set(Mutex::new(HashMap::new())) {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    async fn add_user(&mut self, id: i32, name: String) -> Result<(), String> {
        Self::connect().await?;

        let user = User { id, name };

        // set
        UNDERLYING_DB.get().unwrap().lock().unwrap().insert(id, user);

        // and then get

        match UNDERLYING_DB.get().unwrap().lock().unwrap().get(&id) {
            None => Err("unable to add user to in-memory db".to_string()),
            Some(_) => Ok(())
        }
    }

    async fn get_user(&self, id: i32) -> Result<User, String> {
        Self::connect().await?;

        match UNDERLYING_DB.get().unwrap().lock().unwrap().get(&id) {
            None => Err("unable to find user in in-memory db".to_string()),
            Some(user) => Ok(user.clone()),
        }
    }
}



