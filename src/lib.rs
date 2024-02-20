use rs_zephyr_sdk::{
    stellar_xdr::next::{ContractEventBody, Limits, TransactionMeta, WriteXdr},
    EnvClient,
};

mod proposal;
use proposal::handle_proposal_event;

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();
    let reader = env.reader();
    let processing = reader.tx_processing();
    for tx_processing in processing {
        if let TransactionMeta::V3(meta) = &tx_processing.tx_apply_processing {
            if let Some(soroban) = &meta.soroban_meta {
                if !soroban.events.is_empty() {
                    for event in soroban.events.iter() {
                        let contract_id = event.contract_id.as_ref().unwrap().0;

                        let (topics, data) = match &event.body {
                            ContractEventBody::V0(v0) => (
                                v0.topics
                                    .iter()
                                    .map(|topic| topic.to_xdr(Limits::none()).unwrap())
                                    .collect::<Vec<Vec<u8>>>(),
                                v0.data.to_xdr(Limits::none()).unwrap(),
                            ),
                        };
                        /*
                         *  check what event is being sent and verify it's one of the ones we want to track
                         */
                        let proposal_topic_list = [
                            "proposal_created".as_bytes().to_vec(),
                            "proposal_defeated".as_bytes().to_vec(),
                            "proposal_queued".as_bytes().to_vec(),
                            "proposal_canceled".as_bytes().to_vec(),
                            "proposal_executed".as_bytes().to_vec(),
                            "vote_cast".as_bytes().to_vec(),
                        ];
                        let event_name = topics.get(0).unwrap();
                        let is_proposal_event = proposal_topic_list.contains(event_name);
                        if !is_proposal_event {
                            continue;
                        }
                        if is_proposal_event {
                            let (columns, segments) =
                                handle_proposal_event(event_name, contract_id, &topics, &data);
                            let segments_refs: Vec<&[u8]> =
                                segments.iter().map(|segment| segment.as_slice()).collect();
                            env.db_write("sg_events", &columns, &segments_refs).unwrap();
                        }
                    }
                }
            }
        }
    }
}
