use city_raft::{
    election::elect,
    rpc::execute_command,
    server::{FollowerServer, ServerType},
};

fn start_up(n: usize) -> Vec<ServerType> {
    println!("Booting up instances...");
    let mut servers: Vec<ServerType> = Vec::new();
    (0..n).into_iter().fold(&mut servers, |acc, _| {
        acc.push(ServerType::Follower(FollowerServer::new()));
        acc
    });
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
            // temp to get a leader. fleshing out election logic next.
            let new_leader = follower_server.into_candidate().into_leader();
            leader_servers.push(ServerType::Leader(new_leader));
        }
        _ => panic!("There should be no leaders or candidates currently"),
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
