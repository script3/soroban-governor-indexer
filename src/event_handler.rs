use rs_zephyr_sdk::{
    stellar_xdr::next::{ContractEventV0, Hash, ScVal, Uint32},
    utils, EnvClient,
};

use crate::db;

pub struct EventTypes {
    pub vote_cast: ScVal,
    pub proposal_created: ScVal,
    pub proposal_updated: ScVal,
}

impl EventTypes {
    pub fn new() -> Self {
        Self {
            vote_cast: utils::to_scval_symbol("vote_cast").unwrap(),
            proposal_created: utils::to_scval_symbol("proposal_created").unwrap(),
            proposal_updated: utils::to_scval_symbol("proposal_updated").unwrap(),
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
    ledger_sequence: u32,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => match Uint32::try_from(topic) {
            Ok(num) => num,
            Err(_) => return,
        },
        None => return,
    };
    let voter = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };

    if let ScVal::Vec(data_opt) = &event.data {
        if let Some(data) = data_opt {
            let support = match data.get(0).cloned() {
                Some(topic) => match Uint32::try_from(topic) {
                    Ok(num) => num,
                    Err(_) => return,
                },
                None => return,
            };
            let amount = match event.topics.get(1).cloned() {
                Some(topic) => topic,
                None => return,
            };

            let votes = db::Votes {
                contract: contract_id,
                prop_num: proposal_number,
                user: voter,
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
/// - data - `[title: String, desc: String, calldata: Calldata, sub_calldata: SubCallData]`
pub fn handle_proposal_created(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
    ledger_sequence: u32,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => match Uint32::try_from(topic) {
            Ok(num) => num,
            Err(_) => return,
        },
        None => return,
    };
    let proposer = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };
    if let ScVal::Vec(data_opt) = &event.data {
        if let Some(data) = data_opt {
            let title = match data.get(0).cloned() {
                Some(topic) => topic,
                None => return,
            };
            let desc = match data.get(1).cloned() {
                Some(topic) => topic,
                None => return,
            };
            let calldata = match data.get(2).cloned() {
                Some(topic) => topic,
                None => return,
            };
            let sub_auths = match data.get(3).cloned() {
                Some(topic) => topic,
                None => return,
            };

            let proposal = db::Proposal {
                contract: contract_id,
                prop_num: proposal_number,
                title,
                desc,
                calldata,
                sub_auths,
                proposer,
                status: 0,
                ledger: ledger_sequence,
            };
            db::write_proposal(env, proposal);
        }
    }
}

/// Handle a Proposal Updated event
///
/// Returns None if the event is not a proposal_created event, or the data was malormed
///
/// - topics - `["proposal_updated", proposal_id: u32, status: u32]`
/// - data - `[]`
pub fn handle_proposal_updated(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
    ledger_sequence: u32,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => match Uint32::try_from(topic) {
            Ok(num) => num,
            Err(_) => return,
        },
        None => return,
    };
    let status = match event.topics.get(2).cloned() {
        Some(topic) => match Uint32::try_from(topic) {
            Ok(num) => num,
            Err(_) => return,
        },
        None => return,
    };

    db::update_proposal_status(env, status, contract_id, proposal_number);
}
