use crate::commands::ServerCommands;

#[derive(Debug)]
pub struct PersistentServerState {
    current_term: usize,
    voted_for: usize,
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
    commit_index: usize,
    last_applied: usize,
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
    next_index: usize,
    match_index: usize,
}

impl LeaderServerState {
    pub fn new() -> Self {
        LeaderServerState {
            next_index: 0,
            match_index: 0,
        }
    }
}
