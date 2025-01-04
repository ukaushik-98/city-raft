use super::{
    state::{PersistentServerState, VolatileServerState},
    LeaderServerState,
};

#[derive(Debug)]
pub struct FollowerServer {
    persistent_server_state: PersistentServerState,
    volatile_server_state: VolatileServerState,
}

impl FollowerServer {
    pub fn new() -> FollowerServer {
        FollowerServer {
            persistent_server_state: PersistentServerState::new(),
            volatile_server_state: VolatileServerState::new(),
        }
    }

    fn vote() {
        println!("Voting...")
    }

    pub fn promote(self) -> LeaderServer {
        LeaderServer {
            server_state: self,
            leader_server_state: LeaderServerState::new(),
        }
    }
}

#[derive(Debug)]
pub struct LeaderServer {
    server_state: FollowerServer,
    leader_server_state: LeaderServerState,
}

#[derive(Debug)]
pub enum ServerType {
    Follower(FollowerServer),
    Leader(LeaderServer),
}
