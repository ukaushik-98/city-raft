use commands::execute_command;
use election::elect;
use server::{FollowerServer, ServerType};

pub mod commands;
pub mod election;
pub mod server;

fn start_up(n: usize) -> Vec<ServerType> {
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
    let n: usize = 5;
    let mut follower_servers = start_up(n);
    let mut leader_servers = Vec::new();
    let curr_leader_index = elect(n);
    let curr_leader = follower_servers.remove(curr_leader_index); // take ownership of the server

    match curr_leader {
        ServerType::Follower(follower_server) => {
            let new_leader = follower_server.into_leader();
            leader_servers.push(ServerType::Leader(new_leader));
        }
        _ => panic!("There should be no leaders currently"),
    }

    println!("follower servers; {:?}", follower_servers);

    println!("leader servers: {:?}", leader_servers);

    execute_command();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_up() {
        let n = 5;
        let servers = start_up(n);
        assert_eq!(servers.len(), 5);
        servers
            .iter()
            .for_each(|s| assert!(matches!(s, ServerType::Follower { .. })));
    }
}
