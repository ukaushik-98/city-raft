use crate::rpc::RPC;

/// Persistent state on all servers:
/// Updated on stable storage before responding to RPCs
#[derive(Debug)]
pub struct PersistentServerState {
    /// latest term server has seen (initialized to 0
    /// on first boot, increases monotonically)
    current_term: usize,
    /// candidateId that received vote in current
    /// term (or null if none)
    voted_for: usize,
    // log entries; each entry contains command
    /// for state machine, and term when entry
    /// was received by leader (first index is 1)
    log: Vec<RPC>,
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

/// Volatile server state on ALL servers
#[derive(Debug)]
pub struct VolatileServerState {
    /// index of highest log entry known to be
    /// committed (initialized to 0, increases
    /// monotonically)
    commit_index: usize,
    /// index of highest log entry applied to state
    /// machine (initialized to 0, increases
    /// monotonically)
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

/// Volatile state on all leaders:
/// Reinitialized after election
#[derive(Debug)]
pub struct VolatileLeaderServerState {
    /// for each server, index of the next log entry
    /// to send to that server (initialized to leader
    /// last log index + 1)
    next_index: usize,
    /// for each server, index of highest log entry
    /// known to be replicated on server
    /// (initialized to 0, increases monotonically)
    match_index: usize,
}

impl VolatileLeaderServerState {
    pub fn new() -> Self {
        VolatileLeaderServerState {
            next_index: 0,
            match_index: 0,
        }
    }
}
