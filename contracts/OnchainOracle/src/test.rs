#![cfg(test)]
use crate::{AssertedData, ClaimStatus, OnchainOracle, OnchainOracleClient, CLAIM_ID};
use soroban_sdk:: {log, symbol_short, testutils::Events, Env, String}; 
extern crate std;

#[test]
fn test() {
    let env = Env::default(); 
    let contract_id = env.register(OnchainOracle, ());
    let client = OnchainOracleClient::new(&env, &contract_id);

    // Log contract registration
    log!(&env, "Contract registered with ID: {}", contract_id);
    let challenge_period = 1000;


    let data = &String::from_str(&env, "Barca won against real madrid");
    let data_id: u64 = client.set_claim(data);
    
    let asserted_data = AssertedData {
        claim_id: 1, 
        claim: data.clone(),
        challenge_period,
        claim_status: ClaimStatus::PROPOSED,
    };

        
    let asserted_data_disputed = AssertedData {
        claim_id: 1,
        claim: data.clone(),
        challenge_period,
        claim_status: ClaimStatus::DISPUTED,
    };

    // Log the data ID
    log!(&env, "Data ID generated: {}", data_id);
     
    let result = client.get_claim(&data_id); 
    assert_eq!(result.unwrap(), asserted_data); 

    let dispute_claim = client.dispute_claim(&data_id);
    assert_eq!(dispute_claim.unwrap(), asserted_data_disputed);

    // let expected_event = (CLAIM_ID, symbol_short!("CLAIMID"));
    // let events = env.events().all(); // Get all emitted events


    // for event in events.iter() {
    //     std::println!("{:?}", event.1.get(0))
    //     // log!(&env, "SISHIR", event.1.);
         
    // }
   
}