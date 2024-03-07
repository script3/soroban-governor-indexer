use rs_zephyr_sdk::{
    stellar_xdr::next::ScVal, Condition, DatabaseDerive, DatabaseInteract, EnvClient,
};

use crate::GovernorError;

#[derive(DatabaseDerive, Clone)]
#[with_name("proposals")]
pub struct Proposal {
    pub contract: [u8; 32], // governor contract address
    pub prop_num: u32,      // proposal number
    pub title: ScVal,       // scval type -> string
    pub desc: ScVal,        // scval type -> string
    pub calldata: ScVal,    // custom scval type -> CallData
    pub sub_auths: ScVal,   // custom scval type -> SubCallData
    pub proposer: ScVal,    // scavl type -> address
    pub status: u32,        // proposal status
    pub ledger: u32,        // created time (sequence)
}

#[derive(DatabaseDerive, Clone)]
#[with_name("votes")]
pub struct Votes {
    pub contract: [u8; 32], // governor contract address
    pub prop_num: u32,      // proposal number
    pub user: ScVal,        // user who voted
    pub support: u32,       // vote type
    pub amount: ScVal,      // votes cast
    pub ledger: u32,        // sequence
}

/// Write a new proposal entry to the database
pub fn write_proposal(env: &EnvClient, proposal: Proposal) {
    env.put(&proposal)
}

/// Update a proposal entry with a new status
///
/// Errors if the proposal could not be found
pub fn update_proposal_status(
    env: &EnvClient,
    new_status: u32,
    contract: [u8; 32],
    prop_num: u32,
) -> Result<(), GovernorError> {
    let proposals = env.read::<Proposal>();

    // TODO: Is there a better way to do this? Can we apply some form of WHERE clause to the read?
    if let Some(mut proposal) = proposals
        .iter()
        .find(|p| p.prop_num == prop_num && p.contract == contract)
    {
        proposal.status = new_status;
        env.put(&proposal);
        Ok(())
    } else {
        Err(GovernorError::ProposalNotFound)
    }
}

/// Write a new proposal entry to the database
pub fn write_votes(env: &EnvClient, votes: Votes) {
    env.put(&votes)
}
