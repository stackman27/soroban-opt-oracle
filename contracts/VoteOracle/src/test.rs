#![cfg(test)]

use super::*;
use shared::{AssertedData, ClaimStatus};
use soroban_sdk::{symbol_short, vec, Env, String};
use crate::{VoteOracleClient};
use soroban_sdk::testutils::{Ledger, Address};

#[test]
fn test() {
    let env = Env::default();
    // Maybe use let admin = Address::generate(&env);
    let contract_a_id = env.register(contract_a::WASM, ());
    let contract_b_id = env.register(VoteOracle, (contract_a_id.clone(), ));

 
    let client_a = contract_a::Client::new(&env, &contract_a_id);
    let client_b = VoteOracleClient::new(&env, &contract_b_id); 

    let data_id = 1; 
    let data = &String::from_str(&env, "Barca won against real madrid");
    let challenge_period = 1000;
    let vote = 1; 

    let expected_voter1 = String::from_str(&env, "0xsishir");
    let expected_voter2 = String::from_str(&env, "0xshreeejit");

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
    
    // create a mock claim 
    client_a.set_claim(data);

    log!(&env, "Data ID generated: {}", data_id);
     
    let result = client_a.get_claim(&data_id); 
    assert_eq!(result.unwrap(), asserted_data); 

    let dispute_claim = client_a.dispute_claim(&data_id);
    assert_eq!(dispute_claim.unwrap(), asserted_data_disputed);


    // Vote on a claim 
    let vote1_result = client_b.vote_on_claim(&data_id, &vote, &expected_voter1);
    assert_eq!(vote1_result.votes.len(), 1); // Should contain one vote

    for val in vote1_result.votes.iter() {
        assert_eq!(val.voter, expected_voter1);
        assert_eq!(val.vote, 1);
    }   

    let vote2_result = client_b.vote_on_claim(&data_id, &vote, &expected_voter2);
    assert_eq!(vote2_result.votes.len(), 2); 

    for (index, val1) in vote2_result.votes.iter().enumerate() { 
        // Assert each voter matches the expected value
        if index == 0 {
            assert_eq!(val1.voter, expected_voter1);
            assert_eq!(val1.vote, vote); // votes aren't summed it's just 1 or 0, we'll sum it using a differrent function
        } else if index == 1 {
            assert_eq!(val1.voter, expected_voter2);
            assert_eq!(val1.vote, vote); // votes aren't summed it's just 1 or 0, we'll sum it using a differrent function 
        }
    }

}

// #[test]
// fn test_vote_on_claim_success() {
//     let (env, contract_address) = setup();

//     // Mock data for the contract client
//     let claim_id = 1;
//     let vote = 1; // 1 for "agree"
//     let voter = "voter123".to_string();

//     // Create a mock claim
//     let mock_claim = Some(()); // Mock claim data

//     // Mock client call to `get_claim`
//     let mock_client = contract_a::Client::new(&env, &contract_address);
//     mock_client.mock_get_claim(claim_id, mock_claim);

//     // Call the `vote_on_claim` function
//     let result = vote_on_claim(env, contract_address, claim_id, vote, voter);

//     // Verify the result
//     assert_eq!(result.votes.len(), 1); // Should contain one vote
//     assert_eq!(result.votes[0].voter, "voter123");
//     assert_eq!(result.votes[0].vote, 1);
// }


// #[test]
// fn test() {
//     let env = Env::default();

//     // Register contract A using the imported WASM.
//     let contract_a_id = env.register(contract_a::WASM, ());

//     // Register contract B defined in this crate.
//     let contract_b_id = env.register(ContractB, ());

//     // Create a client for calling contract B.
//     let client = ContractBClient::new(&env, &contract_b_id);

//     // Invoke contract B via its client. Contract B will invoke contract A.
//     let sum = client.add_with(&contract_a_id, &5, &7);
//     assert_eq!(sum, 12);
// }