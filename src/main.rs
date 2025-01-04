use commands::execute_command;

pub mod commands;
pub mod election;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    execute_command();
}
