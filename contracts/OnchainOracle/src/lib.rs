#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Env, IntoVal, Map, String, Symbol, TryFromVal, Val, Vec};
use shared::{ClaimStatus, AssertedData};

const CLAIM_KEY: Symbol = symbol_short!("CLAIM");
const CLAIM_ID: Symbol = symbol_short!("CLAIMID");

#[contract]
pub struct OnchainOracle; 

#[contractimpl]
impl OnchainOracle { 
    pub fn set_claim(env:Env, value: String) -> u64 { 
        let mut claimId: u64 = env.storage().instance().get(&CLAIM_ID).unwrap_or(0);

        claimId += 1;

        let data = &AssertedData { 
            claim_id: claimId,
            claim: value.clone(),
            challenge_period: 1000,
            claim_status: ClaimStatus::PROPOSED,
        };

        let mut storage: Map<u64, AssertedData> = env.storage().instance().get(&CLAIM_KEY).unwrap_or_else(|| Map::new(&env)); 
        storage.set(claimId, data.clone());

        env.storage().instance().set(&CLAIM_KEY, &storage);
        env.storage().instance().set(&CLAIM_ID, &claimId); 
        env.storage().instance().extend_ttl(100, 100);

        let claim_id_val: Val = claimId.into_val(&env);
        let claim_val: Val = data.claim.into_val(&env);
        let challenge_period_val: Val = data.challenge_period.into_val(&env); 
        let claim_status: Val = data.claim_status.into_val(&env);

        // Log the operation 
        env.events().publish(("CLAIM_PROPOSED",), (claim_id_val, claim_val, challenge_period_val, claim_status));
        // log!(&env, "Saved data onchain with ID {}", claimId);

      claimId 
    }

    pub fn dispute_claim(env: Env, id: u64) -> Option <AssertedData> {
        // TODO: Assert the claim is within the challenge period
       let claim: Option<AssertedData>  = Self::get_claim(env.clone(), id);

       if let Some(mut claim) = claim {
            claim.claim_status = ClaimStatus::DISPUTED;

            let mut storage: Map<u64, AssertedData> = env.storage().instance().get(&CLAIM_KEY).unwrap_or_else(|| Map::new(&env)); 
            storage.set(id, claim.clone());     
            env.storage().instance().set(&CLAIM_KEY, &storage);

            // log the update 
            // log!(&env, "Updated claim with ID {} {:?}", id, claim);
            let claim_id_val: Val = id.into_val(&env);

            env.events().publish(("CLAIM_DISPUTED",), claim_id_val);
            Some(claim)
       } else {
            log!(&env, "No claim found for the id {}", id);
            None
       }  
    }

    pub fn get_claim(env: Env, id: u64) -> Option<AssertedData> { 
        let storage: Map<u64, AssertedData> = env
        .storage()
        .instance()
        .get(&CLAIM_KEY)
        .unwrap_or_else(|| Map::new(&env));
        
        if storage.contains_key(id) {
            storage.get(id)
        } else {
            None
        }
    }

    pub fn get_claim_by_status(env: Env, status: ClaimStatus) -> Vec<AssertedData> {
        let storage: Map<u64, AssertedData> = env.storage().instance().get(&CLAIM_KEY).unwrap_or_else(|| Map::new(&env)); 

     
        let mut matching_claims = Vec::<AssertedData>::new(&env); 

        for (_, claim) in storage.iter() {
            if claim.claim_status == status {
                matching_claims.push_back(claim);
            }
        }

        matching_claims
    }

    pub fn get_all_claims(env:Env) -> Vec<AssertedData> {
        let storage: Map<u64, AssertedData> = env.storage().instance().get(&CLAIM_KEY).unwrap_or_else(|| Map::new(&env));

        let mut all_claims = Vec::<AssertedData>::new(&env); 

        for (_, claim) in storage.iter() {
            all_claims.push_back(claim); 
        }

        all_claims
    } 


}


mod test;