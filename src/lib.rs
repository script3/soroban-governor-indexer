use rs_zephyr_sdk::{
    stellar_xdr::next::{ContractEventBody, ScVal, TransactionMeta},
    EnvClient,
};

pub(crate) mod db;
mod event_handler;

pub enum GovernorError {
    ProposalNotFound,
}

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();
    let reader = env.reader();
    let ledger_sequence = ScVal::U32(reader.ledger_sequence());
    let processing = reader.tx_processing();
    let event_types = event_handler::EventTypes::new();
    for tx_processing in processing {
        if let TransactionMeta::V3(meta) = &tx_processing.tx_apply_processing {
            if let Some(soroban) = &meta.soroban_meta {
                if !soroban.events.is_empty() {
                    for event in soroban.events.iter() {
                        if let Some(contract_id) = event.contract_id.clone() {
                            match &event.body {
                                ContractEventBody::V0(v0) => {
                                    if let Some(topic0) = v0.topics.get(0) {
                                        if topic0 == &event_types.vote_cast {
                                            event_handler::handle_vote_cast(
                                                &env,
                                                contract_id,
                                                &v0,
                                                ledger_sequence.clone(),
                                            );
                                        } else if topic0 == &event_types.proposal_created {
                                            event_handler::handle_proposal_created(
                                                &env,
                                                contract_id,
                                                &v0,
                                            );
                                        } else if topic0 == &event_types.proposal_canceled {
                                            event_handler::handle_proposal_updated(
                                                &env,
                                                contract_id,
                                                &v0,
                                                ScVal::U32(5),
                                            );
                                        } else if topic0 == &event_types.proposal_executed {
                                            event_handler::handle_proposal_updated(
                                                &env,
                                                contract_id,
                                                &v0,
                                                ScVal::U32(4),
                                            );
                                        } else if topic0 == &event_types.proposal_expired {
                                            event_handler::handle_proposal_updated(
                                                &env,
                                                contract_id,
                                                &v0,
                                                ScVal::U32(3),
                                            );
                                        } else if topic0 == &event_types.proposal_voting_closed {
                                            event_handler::handle_proposal_voting_closed(
                                                &env,
                                                contract_id,
                                                &v0,
                                            );
                                        } else {
                                            // untracked event occurred
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
