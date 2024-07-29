mod client;
mod config;
mod macros;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    component: String,
    task: i32
}

fn main() {
    let args = Args::parse();
    let component = client::sgcp::Component::from_str_name(&args.component).unwrap();
    let task = args.task;
    client::make_request(component, task).unwrap();
}