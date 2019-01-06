mod runner;
mod settings;
mod user;
mod ui;
mod storage;
mod verificator;

use runner::Runner;

fn main() {
    let app : Runner = Runner::new();
    app.run();
}

