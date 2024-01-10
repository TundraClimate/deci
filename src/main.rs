mod app;

use app::App;
use clap::Parser;

fn main() {
    App::parse();
}
