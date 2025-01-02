#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, log, Address, Env, FromVal, Symbol, Map, String, Vec};

mod contract_a {
    use shared::AssertedData;

    soroban_sdk::contractimport!(
            file = "../../target/wasm32-unknown-unknown/release/onchain_oracle.wasm"
    );
}

const VOTE_KEY: Symbol = symbol_short!("VOTE");
const ORACLE_CONTRACT: Symbol = symbol_short!("OORACLE"); // Optimistic oracle contract address

#[contract]
pub struct VoteOracle;

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct Votes {
    pub votes: Vec<Vote>, // Dynamic array of Vote
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct Vote {
    pub voter: String, 
    pub vote: u32, // True or false
}

#[contractimpl]
impl VoteOracle {
    // TODO: optimistic oracle contract Address needs to be defined in the constructor 
    pub fn __constructor(
        env: Env,
        onchain_oracle: Address
    ) { 
        env.storage().instance().set(&ORACLE_CONTRACT, &onchain_oracle);
    }   


    pub fn vote_on_claim(env: Env, claim_id: u64, vote: u32, voter: String) -> Votes {
        
        // TODO: only proceed if this claim has been disputed

        // Retrieve stored votes from the contract storage, or create a new map if it doesn't exist.
        let mut stored_votes: Map<u64, Votes> = env.storage().instance().get(&VOTE_KEY)
        .unwrap_or_else(|| Map::new(&env)); 

        let mut oracle_contract = Self::get_oracle_address(env.clone());

        let client = contract_a::Client::new(&env, &oracle_contract);
        let result = client.get_claim(&claim_id);

        if let Some(data) = &result {
            log!(&env, "claimFound");
        } else {
            log!(&env, "claimNotFound");
        }

         // Retrieve the existing votes for the claim or create an empty list if no votes exist
        let mut current_votes = stored_votes.get(claim_id).unwrap_or_else(|| Votes { votes: Vec::new(&env) });
        
        // Create a new vote
        let new_vote = Vote {
            voter: voter.clone(),     // Voter's identifier (String)
            vote,             // 1 for agree, 0 for disagree
        };

        // Add the new vote to the existing votes for this claim
        current_votes.votes.push_back(new_vote);
        
        let cloned_votes = current_votes.clone();

        // Update the stored votes map with the modified votes
        stored_votes.set(claim_id, cloned_votes);

        log!(&env, "Voted on claim", claim_id);
        // Save the updated votes back to storage
        env.storage().instance().set(&VOTE_KEY, &stored_votes);

        // Return the updated votes for the claim
        current_votes


        // [APPROACH 2] revisit if needed 
        // 
        // let mut votes_vec = Vec::<Votes>::new(&env);
        // for (_, val) in stored_votes.iter() {
        //     votes_vec.push_back(val);
        // }   

        // // TODO: voter should be the user address that should be automatically retrieved in this function, 
        // // if not check the list of staked users reference if voter is available in the list 
        // let new_vote = Vote {
        //     voter: voter,     
        //     vote: 1                   // 1 agree, 0 disagree
        // }; 

        // let mut vote_vec = Vec::<Vote>::new(&env); 
        // vote_vec.push_back(new_vote); 

        // stored_votes.set(claim_id, votes);

        // env.storage().instance().set(&VOTE_KEY, &stored_votes);
        // env.storage().instance().extend_ttl(100, 100);
    
        // votes // Return the result
    }   

    pub fn get_vote_on_claim(env: Env, claim_id: u64) {

    }

    pub fn get_vote(env: Env, vote_id: u64) {

    }

    pub fn get_all_votes(env:Env) {

    }
    
    // Gets the stored oracle address
    pub fn get_oracle_address(env: Env) -> Address {
        let oracle_address: Address = env.storage().instance().get(&ORACLE_CONTRACT).expect("Oracle address not found");  // Handle the case if not found
        oracle_address
    }

    pub fn string_to_address(string: String) -> Address {
        Address::from_string(&string)
    }

    pub fn address_to_string(address: Address) -> String {
        address.to_string()
    }
}

mod test;