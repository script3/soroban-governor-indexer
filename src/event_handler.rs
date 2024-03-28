use rs_zephyr_sdk::{
    stellar_xdr::next::{ContractEventV0, Hash, ScVal},
    utils, EnvClient,
};

use crate::db;

pub struct EventTypes {
    pub vote_cast: ScVal,
    pub proposal_created: ScVal,
    pub proposal_canceled: ScVal,
    pub proposal_voting_closed: ScVal,
    pub proposal_executed: ScVal,
    pub proposal_expired: ScVal,
}

impl EventTypes {
    pub fn new() -> Self {
        Self {
            vote_cast: utils::to_scval_symbol("vote_cast").unwrap(),
            proposal_created: utils::to_scval_symbol("proposal_created").unwrap(),
            proposal_canceled: utils::to_scval_symbol("proposal_canceled").unwrap(),
            proposal_voting_closed: utils::to_scval_symbol("proposal_voting_closed").unwrap(),
            proposal_executed: utils::to_scval_symbol("proposal_executed").unwrap(),
            proposal_expired: utils::to_scval_symbol("proposal_expired").unwrap(),
        }
    }
}

/// Handle a Vote Cast event
///
/// Returns None if the event is not a vote_cast event, or the data was malormed
///
/// Event:
/// - topics - `["vote_cast", proposal_id: u32, voter: Address]`
/// - data - `[support: u32, amount: i128]`
pub fn handle_vote_cast(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
    ledger_sequence: ScVal,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let voter = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };

    if let ScVal::Vec(data_opt) = &event.data {
        if let Some(data) = data_opt {
            let support = match data.get(0).cloned() {
                Some(data) => data,
                None => return,
            };
            let amount = match data.get(1).cloned() {
                Some(data) => data,
                None => return,
            };

            let votes = db::Votes {
                contract: contract_id,
                prop_num: proposal_number,
                voter,
                support,
                amount,
                ledger: ledger_sequence,
            };
            db::write_votes(env, votes);
        }
    }
}

/// Handle a Proposal Created event
///
/// Returns None if the event is not a proposal_created event, or the data was malormed
///
/// - topics - `["proposal_created", proposal_id: u32, proposer: Address]`
/// - data - `[title: String, desc: String, action: ProposalAction, vote_start: u32, vote_end: u32]`
pub fn handle_proposal_created(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let proposer = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };
    if let ScVal::Vec(data_opt) = &event.data {
        if let Some(data) = data_opt {
            let title = match data.get(0).cloned() {
                Some(data) => data,
                None => return,
            };
            let descr = match data.get(1).cloned() {
                Some(data) => data,
                None => return,
            };
            let action = match data.get(2).cloned() {
                Some(data) => data,
                None => return,
            };
            let vote_start = match data.get(3).cloned() {
                Some(data) => data,
                None => return,
            };
            let vote_end = match data.get(4).cloned() {
                Some(data) => data,
                None => return,
            };

            let proposal = db::Proposal {
                contract: contract_id,
                prop_num: proposal_number,
                title,
                descr,
                action,
                creator: proposer,
                status: ScVal::U32(0),
                v_start: vote_start,
                v_end: vote_end,
                eta: ScVal::U32(0),
                votes: ScVal::Void,
            };
            db::write_proposal(env, proposal);
        }
    }
}

/// Handle a Proposal Status Update event
///
/// Returns None if the event is not a proposal status update event
///
/// - topics - `["proposal_canceled" or "proposal_executed" or "proposal_expired", proposal_id: u32]`
/// - data - `[]`
pub fn handle_proposal_updated(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
    status: ScVal,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => topic,
        None => return,
    };

    let _ = db::update_proposal_status(env, status, contract_id, proposal_number);
}


/// Handle proposal voting closed
///
/// Returns None if the event is not a proposal status update event
///
/// - topics - `["proposal_voting_closed", proposal_id: u32, status: u32, eta: u32]`
/// - data - `final_votes: VoteCount`
pub fn handle_proposal_voting_closed(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let status = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let eta = match event.topics.get(3).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let votes = event.data.clone();

    let _ = db::update_proposal_voting_closed(env, status, eta, votes, contract_id, proposal_number);
}
