use crate::commands::ServerCommands;

pub struct PersistentServerState {
    current_term: i32,
    voted_for: i32,
    log: Vec<ServerCommands>,
}

impl PersistentServerState {
    fn new() -> Self {
        PersistentServerState {
            current_term: 0,
            voted_for: 0,
            log: Vec::new(),
        }
    }
}

pub struct VolatitleServerState {
    commit_index: i32,
    last_applied: i32,
}

impl VolatitleServerState {
    fn new() -> Self {
        VolatitleServerState {
            commit_index: 0,
            last_applied: 0,
        }
    }
}

pub struct LeaderServerState {
    next_index: i32,
    match_index: i32,
}
