use std::collections::HashMap;

use ic_verifiable_credentials::{build_credential_jwt, issuer_api::{ArgumentValue, CredentialSpec}, CredentialParams};

#[ic_cdk::update]
pub fn create_credential_jwt(
    credential_type : String ,
    credential_args :Option<HashMap<String, ArgumentValue>>,
    
    credential_id_url: String,
    issuer_url: String,
    expiration_timestamp_s: u32, 
)->String{
    let params = CredentialParams {
        spec :CredentialSpec{
            credential_type : credential_type,
            arguments : credential_args
        },
        subject_id : format!("https://github.com/Sumit240803/stealth-dao-WCHL-2025/credentials/{}",ic_cdk::api::msg_caller().to_string()),
        credential_id_url : credential_id_url.to_string(),
        issuer_url : issuer_url.to_string(),
        expiration_timestamp_s : expiration_timestamp_s

    };

    build_credential_jwt(params)
}