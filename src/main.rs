use commands::execute_command;
use election::elect;
use server::{FollowerServer, ServerType};

pub mod commands;
pub mod election;
pub mod server;

fn start_up(n: u32) -> Vec<ServerType> {
    println!("Booting up instances...");
    let mut servers: Vec<ServerType> = Vec::new();
    for _ in 0..n {
        servers.push(ServerType::Follower(FollowerServer::new()));
    }
    servers
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let n: u32 = 5;
    let mut servers = start_up(n);
    let curr_leader_index = TryFrom::try_from(elect(n)).unwrap();
    let curr_leader = servers.remove(curr_leader_index); // take ownership of the server

    match curr_leader {
        ServerType::Follower(follower_server) => {
            let new_leader = follower_server.promote();
            servers.push(ServerType::Leader(new_leader));
        }
        _ => panic!("There should be no leaders currently"),
    }

    println!("servers; {:?}", servers);

    execute_command();
}
