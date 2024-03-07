use rs_zephyr_sdk::{
    stellar_xdr::next::{ContractEventBody, TransactionMeta},
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
    let ledger_sequence = reader.ledger_sequence();
    let processing = reader.tx_processing();
    let event_types = event_handler::EventTypes::new();
    for tx_processing in processing {
        if let TransactionMeta::V3(meta) = &tx_processing.tx_apply_processing {
            if let Some(soroban) = &meta.soroban_meta {
                if !soroban.events.is_empty() {
                    for event in soroban.events.iter() {
                        let contract_id = event.contract_id.as_ref().unwrap().0;
                        if let Some(contract_hash) = event.contract_id {
                            // ScAddress::from_hash() is not implemented yet?
                            let contract_id = contract_hash.0;
                            match &event.body {
                                ContractEventBody::V0(v0) => {
                                    if let Some(topic0) = v0.topics.get(0) {
                                        // TODO: Handle failures? Probably don't want to panic anywhere in this thread otherwise
                                        //       events will get missed.
                                        if topic0 == &event_types.vote_cast {
                                            event_handler::handle_vote_cast(
                                                &env,
                                                contract_id,
                                                &v0,
                                                ledger_sequence,
                                            );
                                        } else if topic0 == &event_types.proposal_created {
                                            event_handler::handle_proposal_created(
                                                &env,
                                                contract_id,
                                                &v0,
                                                ledger_sequence,
                                            );
                                        } else if topic0 == &event_types.proposal_updated {
                                            event_handler::handle_proposal_updated(
                                                &env,
                                                contract_id,
                                                &v0,
                                                ledger_sequence,
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
