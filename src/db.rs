use crate::GovernorError;
use zephyr_sdk::{
    soroban_sdk::xdr::{Hash, Limits, ReadXdr, ScVal, WriteXdr},
    Condition, DatabaseDerive, DatabaseInteract, EnvClient,
};

#[derive(DatabaseDerive, Clone)]
#[with_name("proposals")]
pub struct Proposal {
    pub contract: Hash,                 // governor contract address
    pub prop_num: ScVal,                // proposal number
    pub title: ScVal,                   // scval type -> string
    pub descr: ScVal,                   // scval type -> string
    pub action: ScVal,                  // custom scval type -> ProposalAction
    pub creator: ScVal,                 // scavl type -> address
    pub status: ScVal,                  // proposal status
    pub v_start: ScVal,                 // vote start ledger
    pub v_end: ScVal,                   // vote end ledger
    pub eta: ScVal,                     // execution eta, or zero if no execution scheduled
    pub votes: ScVal,                   // final votes at voting close
    pub proposal_execution_hash: ScVal, // hash of the proposal execution
}

#[derive(DatabaseDerive, Clone)]
#[with_name("votes")]
pub struct Votes {
    pub contract: Hash,  // governor contract address
    pub prop_num: ScVal, // proposal number
    pub voter: ScVal,    // user who voted
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

    // TODO: Update to only change a single row
    if let Some(proposal) = proposals
        .iter()
        .find(|p| p.prop_num == prop_num && p.contract == contract)
    {
        let mut proposal = proposal.clone();
        proposal.status = new_status;

        env.update()
            .column_equal_to_bytes(
                "contract",
                contract.to_xdr(Limits::none()).unwrap().as_slice(),
            )
            .column_equal_to_bytes(
                "prop_num",
                prop_num.to_xdr(Limits::none()).unwrap().as_slice(),
            )
            .execute(&proposal)
            .unwrap();
        Ok(())
    } else {
        Err(GovernorError::ProposalNotFound)
    }
}

/// Update a proposal entry with a new status
///
/// Errors if the proposal could not be found
pub fn update_proposal_voting_closed(
    env: &EnvClient,
    new_status: ScVal,
    eta: ScVal,
    votes: ScVal,
    contract: Hash,
    prop_num: ScVal,
) -> Result<(), GovernorError> {
    let proposals = env.read::<Proposal>();

    // TODO: Update to only change a single row
    if let Some(proposal) = proposals
        .iter()
        .find(|p| p.prop_num == prop_num && p.contract == contract)
    {
        let mut proposal = proposal.clone();
        proposal.status = new_status;
        proposal.eta = eta;
        proposal.votes = votes;

        env.update()
            .column_equal_to_bytes(
                "contract",
                contract.to_xdr(Limits::none()).unwrap().as_slice(),
            )
            .column_equal_to_bytes(
                "prop_num",
                prop_num.to_xdr(Limits::none()).unwrap().as_slice(),
            )
            .execute(&proposal)
            .unwrap();
        Ok(())
    } else {
        Err(GovernorError::ProposalNotFound)
    }
}

pub fn update_proposal_execution_hash(
    env: &EnvClient,
    execution_hash: ScVal,
    contract: Hash,
    prop_num: ScVal,
) -> Result<(), GovernorError> {
    let proposals = env.read::<Proposal>();
    // TODO: Update to only change a single row
    if let Some(proposal) = proposals
        .iter()
        .find(|p| p.prop_num == prop_num && p.contract == contract)
    {
        let mut proposal = proposal.clone();
        proposal.proposal_execution_hash = execution_hash;

        env.update()
            .column_equal_to_bytes(
                "contract",
                contract.to_xdr(Limits::none()).unwrap().as_slice(),
            )
            .column_equal_to_bytes(
                "prop_num",
                prop_num.to_xdr(Limits::none()).unwrap().as_slice(),
            )
            .execute(&proposal)
            .unwrap();
        Ok(())
    } else {
        Err(GovernorError::ProposalNotFound)
    }
}

/// Write a new proposal entry to the database
pub fn write_votes(env: &EnvClient, votes: Votes) {
    env.put(&votes)
}
