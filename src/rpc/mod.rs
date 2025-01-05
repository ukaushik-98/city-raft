use crate::logs::LogEntries;

#[derive(Debug)]
pub enum RPC {
    Append(AppendRPC),
    RequestVote(VoteRPC),
}

#[derive(Debug)]
struct VoteRPC {
    term: usize,
    candidateId: usize,
    lastLogIndex: usize,
    lastLogTerm: usize,
}

#[derive(Debug)]
pub struct AppendRPC {
    term: usize,
    leaderId: usize,
    prevLogIndex: usize,
    entries: Vec<LogEntries>,
    leaderCommit: usize,
}

pub fn execute_command() {
    println!("executing command...");
}
