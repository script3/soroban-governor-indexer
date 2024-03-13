use crate::GovernorError;
use rs_zephyr_sdk::{
    stellar_xdr::next::{Hash, ScVal, WriteXdr, ReadXdr, Limits},
    Condition, DatabaseDerive, DatabaseInteract, EnvClient,
};

#[derive(DatabaseDerive, Clone)]
#[with_name("proposals")]
pub struct Proposal {
    pub contract: Hash,  // governor contract address
    pub prop_num: ScVal, // proposal number
    pub title: ScVal,    // scval type -> string
    pub descr: ScVal,     // scval type -> string
    pub action: ScVal,   // custom scval type -> ProposalAction
    pub creator: ScVal,  // scavl type -> address
    pub status: ScVal,   // proposal status
    pub ledger: ScVal,   // created time (sequence)
}

#[derive(DatabaseDerive, Clone)]
#[with_name("votes")]
pub struct Votes {
    pub contract: Hash,  // governor contract address
    pub prop_num: ScVal, // proposal number
    pub voter: ScVal,     // user who voted
    pub support: ScVal,  // vote type
    pub amount: ScVal,   // votes cast
    pub ledger: ScVal,   // sequence
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
    new_status: ScVal,
    contract: Hash,
    prop_num: ScVal,
) -> Result<(), GovernorError> {
    let proposals = env.read::<Proposal>();

    // Currently all of this is required.
    // Soon Zephyr will be able to just update only a column.

    // TODO: Is there a better way to do this? Can we apply some form of WHERE clause to the read?
    if let Some(proposal) = proposals
        .iter()
        .find(|p| p.prop_num == prop_num && p.contract == contract)
    {
        let mut proposal = proposal.clone();
        proposal.status = new_status;

        env.update(
            &proposal,
            &[
                Condition::ColumnEqualTo("contract".into(), contract.to_xdr(Limits::none()).unwrap()),
                Condition::ColumnEqualTo("prop_num".into(), prop_num.to_xdr(Limits::none()).unwrap()),
            ],
        );
        Ok(())
    } else {
        Err(GovernorError::ProposalNotFound)
    }
}

/// Write a new proposal entry to the database
pub fn write_votes(env: &EnvClient, votes: Votes) {
    env.put(&votes)
}
