use soroban_sdk::{contracttype, String};


#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub enum ClaimStatus {
    PROPOSED,
    DISPUTED,
    SUCCESS, 
    FAILED,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct AssertedData { 
    pub claim_id: u64, 
    pub claim: String,
    pub challenge_period: u64,
    pub claim_status: ClaimStatus,
}