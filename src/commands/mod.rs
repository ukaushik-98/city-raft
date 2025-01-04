#[derive(Debug)]
pub enum CommandType {
    Insert,
    Delete,
    Update,
}

pub struct ServerCommands {
    commandType: CommandType,
}

pub fn execute_command() {
    println!("executing command...");
}
