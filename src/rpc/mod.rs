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
    candidateId: usize,
    lastLogIndex: usize,
    lastLogTerm: usize,
}

#[derive(Debug)]
struct HeartBeatRPCResponse {
    term: usize,
    success: bool,
}

#[derive(Debug)]
struct RequestVoteRPC {
    term: usize,
    candidateId: usize,
    lastLogIndex: usize,
    lastLogTerm: usize,
}

#[derive(Debug)]
struct RequestVoteRPCResponse {
    term: usize,
    voteGranted: bool,
}

#[derive(Debug)]
pub struct AppendRPC {
    term: usize,
    leaderId: usize,
    prevLogIndex: usize,
    entries: Vec<LogEntries>,
    leaderCommit: usize,
}

#[derive(Debug)]
struct AppendRPCResponse {
    term: usize,
    success: bool,
}

pub fn execute_command() {
    println!("executing command...");
}
