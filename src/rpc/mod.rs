use crate::logs::LogEntries;

#[derive(Debug)]
pub enum RPC {
    HeartBeatRPC(HeartBeatRPC),
    HeartBeatRPCResponse(HeartBeatRPCResponse),
    Append(AppendRPC),
    AppendRPCResponse(AppendRPCResponse),
    RequestVote(RequestVoteRPC),
    RequestVoteRPCResponse(RequestVoteRPCResponse),
}

/// Heartbeat struct used by leaders
/// leader -> follower
#[derive(Debug)]
struct HeartBeatRPC {
    term: usize,
    candidate_id: usize,
    last_log_index: usize,
    last_log_term: usize,
}

/// Response to heartbeat
/// follower -> leader
#[derive(Debug)]
struct HeartBeatRPCResponse {
    term: usize,
    success: bool,
}

/// Vote Request RPC used by candidates
/// candidate -> follower
#[derive(Debug)]
struct RequestVoteRPC {
    /// candidate's term
    term: usize,
    /// candidate requesting vote
    candidate_id: usize,
    /// index of candidate's last log entry
    last_log_index: usize,
    /// term of cnadidate's last log entry
    last_log_term: usize,
}

/// Response to RequestVoteRPC
/// follower -> candidate
#[derive(Debug)]
struct RequestVoteRPCResponse {
    /// currentTerm, for candidate to update itself
    term: usize,
    /// true means candidate received vote
    vote_granted: bool,
}

/// Invoked by leader to replicate log entries
/// Matches struct for HeartBeat
#[derive(Debug)]
pub struct AppendRPC {
    /// leader's term
    term: usize,
    /// so follower can redirect clients
    leader_id: usize,
    /// index of log entry immediately preceding
    /// new ones
    prev_log_index: usize,
    /// term of prevLogIndex entry
    prev_log_term: usize,
    /// log entries to store (empty for heartbeat;
    /// may send more than one for efficiency)
    entries: Vec<LogEntries>,
    /// leader's commitIndex
    leader_commit: usize,
}

/// Response to AppendRPC
/// follwer -> leader
#[derive(Debug)]
struct AppendRPCResponse {
    /// currentTerm, for leader to update itsel
    term: usize,
    /// true if follower contained entry matching
    /// prevLogIndex and prevLogTerm
    success: bool,
}

pub fn execute_command() {
    println!("executing command...");
}
