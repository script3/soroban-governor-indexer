pub extern "C" fn handle_proposal_event(
    event_name: &Vec<u8>,
    contract_id: [u8; 32],
    topics: &Vec<Vec<u8>>,
    data: &Vec<u8>,
) -> (Vec<&'static str>, Vec<Vec<u8>>) {
    let (columns, segments) = match event_name.as_slice() {
        b"vote_cast" => (
            vec![
                "contract_id",
                "type",
                "proposal_id",
                "voter",
                "support",
                "amount",
            ],
            vec![
                contract_id.to_vec(),
                topics.get(0).cloned().unwrap_or_default(),
                topics.get(1).cloned().unwrap_or_default(),
                topics.get(2).cloned().unwrap_or_default(),
                vec![data.get(0).cloned().unwrap_or(0)],
                vec![data.get(1).cloned().unwrap_or(0)],
            ],
        ),
        b"votes_changed" => (vec![], vec![]),
        b"deposit" => (vec![], vec![]),
        b"withdraw" => (vec![], vec![]),
        _ => (vec![], vec![]), // Default case, you may want to handle this based on your use case
    };
    (columns, segments)
}
