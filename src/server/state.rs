use crate::commands::ServerCommands;

#[derive(Debug)]
pub struct PersistentServerState {
    current_term: u32,
    voted_for: u32,
    log: Vec<ServerCommands>,
}

impl PersistentServerState {
    pub fn new() -> Self {
        PersistentServerState {
            current_term: 0,
            voted_for: 0,
            log: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct VolatileServerState {
    commit_index: u32,
    last_applied: u32,
}

impl VolatileServerState {
    pub fn new() -> Self {
        VolatileServerState {
            commit_index: 0,
            last_applied: 0,
        }
    }
}

#[derive(Debug)]
pub struct LeaderServerState {
    next_index: u32,
    match_index: u32,
}

impl LeaderServerState {
    pub fn new() -> Self {
        LeaderServerState {
            next_index: 0,
            match_index: 0,
        }
    }
}
