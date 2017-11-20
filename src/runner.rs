use settings::Settings;
use user::User;

use ui;
use verificator;
use storage;
use storage::StorageBackend;

/**
 * Global application data.
 */
pub struct Runner {
    settings: Settings,
    storage:  Box<StorageBackend>,
}

/**
 * C++-alike class encapsulating application state.
 */
impl Runner {
    /**
     * Initialize the application's message catalogs, loggers,
     * database connections, etc.
     */
    pub fn new() -> Self {
        let stn : Settings = Settings::new("/var/borsh/config.yml".to_string());
        let backend : Box<StorageBackend> = storage::get(&stn);

        let app : Runner = Runner { settings: stn,
            storage: backend };

        return app;
    }

    /**
     * Run the user registration UI:
     *
     * # Description
     *
     * - Start the interface for getting the information from the
     *   user.
     * - Get the structure representing the new user.
     * - Add the user to the database.
     * - Start the interface for user verification.
     * - Get the structure representing updated user state.
     * - Update the user state in the database.
     * - Exit the shell.
     */
    pub fn run(&self) {
        println!("{}", self.settings.tr("BoringHub registration".to_string()));
        let registrant : User = ui::start(&self.settings);
        println!("User added: {}", &registrant.name());
        let checked : bool = verificator::check(&registrant);
        if checked {
            println!("User is verified!");
        } else {
            println!("User is not verified!");
        }
    }
}

