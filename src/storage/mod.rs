pub mod storage_type;

/* Various supported backends */
mod sqlite;

use crate::user::User;
use crate::settings::Settings;

use self::storage_type::StorageType;
use self::sqlite::SQLiteStorage;

pub trait StorageBackend {
    /**
     * Change user settings.
     */
    fn update_user(&self, user: &User);
}

impl StorageBackend for SQLiteStorage {
    fn update_user(&self, user: &User) {
        self.update(user);
    }
}

pub fn get(settings: &Settings) -> Box<StorageBackend> {
    match settings.get_storage_type() {
        StorageType::SQLite => Box::new(SQLiteStorage::new(&"/var/borsh/borsh.sqlite3".to_string())),
        _ => Box::new(SQLiteStorage::new(&"/var/borsh/borsh.sqlite3".to_string())),
    }
}

