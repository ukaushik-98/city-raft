use super::{
    state::{PersistentServerState, VolatileServerState},
    VolatileLeaderServerState,
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

    pub fn into_candidate(self) -> CandidateServer {
        CandidateServer { server_state: self }
    }
}

#[derive(Debug)]
pub struct CandidateServer {
    server_state: FollowerServer,
}

impl CandidateServer {
    pub fn into_leader(self) -> LeaderServer {
        LeaderServer {
            server_state: self.server_state,
            leader_server_state: VolatileLeaderServerState::new(),
        }
    }
}

// The LeaderServer struct is essentially a wrapper around the existing server_state.
// The only way to create a LeaderServer is by promoting a FollowerServer.
#[derive(Debug)]
pub struct LeaderServer {
    server_state: FollowerServer,
    leader_server_state: VolatileLeaderServerState,
}

impl LeaderServer {
    fn into_follower(self) -> FollowerServer {
        self.server_state
    }
}

///At any given time each server is in one of three states:
/// - leader
/// - follower
/// - candidate
#[derive(Debug)]
pub enum ServerType {
    Follower(FollowerServer),
    Candidate(CandidateServer),
    Leader(LeaderServer),
}
