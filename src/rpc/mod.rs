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

#[derive(Debug)]
struct HeartBeatRPC {
    term: usize,
    candidate_id: usize,
    last_log_index: usize,
    last_log_term: usize,
}

#[derive(Debug)]
struct HeartBeatRPCResponse {
    term: usize,
    success: bool,
}

#[derive(Debug)]
struct RequestVoteRPC {
    term: usize,
    candidate_id: usize,
    last_log_index: usize,
    last_log_term: usize,
}

#[derive(Debug)]
struct RequestVoteRPCResponse {
    term: usize,
    vote_granted: bool,
}

#[derive(Debug)]
pub struct AppendRPC {
    term: usize,
    leader_id: usize,
    prev_log_index: usize,
    entries: Vec<LogEntries>,
    leader_commit: usize,
}

#[derive(Debug)]
struct AppendRPCResponse {
    term: usize,
    success: bool,
}

pub fn execute_command() {
    println!("executing command...");
}
