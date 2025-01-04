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

    pub fn into_leader(self) -> LeaderServer {
        LeaderServer {
            server_state: self,
            leader_server_state: LeaderServerState::new(),
        }
    }
}

// The LeaderServer struct is essentially a wrapper around the existing server_state.
// The only way to create a LeaderServer is by promoting a FollowerServer.
#[derive(Debug)]
pub struct LeaderServer {
    server_state: FollowerServer,
    leader_server_state: LeaderServerState,
}

impl LeaderServer {
    fn into_follower(self) -> FollowerServer {
        self.server_state
    }
}

#[derive(Debug)]
pub enum ServerType {
    Follower(FollowerServer),
    Leader(LeaderServer),
}
