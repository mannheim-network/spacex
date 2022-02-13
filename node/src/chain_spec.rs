// Copyright (C) 2019-2021 Spacex Network Technologies Ltd.
// This file is part of Spacex.

use hex_literal::hex;
use sp_core::{Pair, Public, sr25519, crypto::UncheckedInto};
use spacex_runtime::{
    AuthorityDiscoveryId, BalancesConfig, GenesisConfig, ImOnlineId,
    AuthorityDiscoveryConfig, SessionConfig, SessionKeys, StakerStatus,
    StakingConfig, IndicesConfig, SystemConfig, StorageConfig, SudoConfig,
    CouncilConfig, DemocracyConfig, ElectionsConfig, TechnicalCommitteeConfig,
    WASM_BINARY
};
use cstrml_staking::Forcing;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_consensus_babe::AuthorityId as BabeId;
use primitives::{constants::currency::HEIM, *};
use sc_service::ChainType;
use sp_runtime::{traits::{Verify, IdentifyAccount}, Perbill};

const DEFAULT_PROTOCOL_ID: &str = "heim";
// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Helper function to generate spacex session key
fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// The `ChainSpec parametrised for spacex runtime`.
pub type SpacexChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
type AccountPublic = <Signature as Verify>::Signer;

/// Spacex development config (single validator Alice)
pub fn development_config() -> Result<SpacexChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Local test wasm not available")?;

    Ok(SpacexChainSpec::from_genesis(
        "Rubik",
        "rubik",
        ChainType::Development,
        move || testnet_genesis(
            wasm_binary,
            vec![
                get_authority_keys_from_seed("Alice")
            ],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            vec![
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            ],
            true,
        ),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default()
    ))
}

/// Spacex local testnet config (multi-validator Alice + Bob)
pub fn local_testnet_config() -> Result<SpacexChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Local test wasm not available")?;

    Ok(SpacexChainSpec::from_genesis(
        "Rubik",
        "rubik",
        ChainType::Local,
        move || testnet_genesis(
            wasm_binary,
            vec![
                get_authority_keys_from_seed("Alice"),
                get_authority_keys_from_seed("Bob"),
            ],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            vec![
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Dave"),
                get_account_id_from_seed::<sr25519::Public>("Eve"),
                get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
            ],
            true,
        ),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default()
    ))
}

/// Spacex rubik(aka. internal testnet) config
pub fn rubik_config() -> Result<SpacexChainSpec, String> {
    SpacexChainSpec::from_json_bytes(&include_bytes!("../res/rubik.json")[..])
}

/// Spacex mainnet(aka. open testnet) config
pub fn mainnet_config() -> Result<SpacexChainSpec, String> {
    SpacexChainSpec::from_json_bytes(&include_bytes!("../res/mainnet.json")[..])
}

/// Spacex rubik staging config
pub fn rubik_staging_config() -> Result<SpacexChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Rubik wasm not available")?;

    Ok(SpacexChainSpec::from_genesis(
        "Rubik",
        "rubik",
        ChainType::Live,
        move || rubik_staging_testnet_config_genesis(wasm_binary),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default()
    ))
}

/// Spacex mainnet staging config
pub fn mainnet_staging_config() -> Result<SpacexChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Mainnet wasm not available")?;

    Ok(SpacexChainSpec::from_genesis(
        "Mannheim",
        "mannheim",
        ChainType::Live,
        move || mainnet_staging_testnet_config_genesis(wasm_binary),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default()
    ))
}

/// The genesis spec of spacex dev/local test network
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    _root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    let num_endowed_accounts = endowed_accounts.len();
    const ENDOWMENT: u128 = 1_000_000 * HEIM;
    const STASH: u128 = 20_000 * HEIM;
    GenesisConfig {
        pallet_sudo: Some(SudoConfig {
            key: endowed_accounts[0].clone(),
        }),
        frame_system: Some(SystemConfig {
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        balances_Instance1: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig {
            indices: vec![],
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            validator_count: 4,
            minimum_validator_count: 1,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: Forcing::NotForcing,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts.iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts.iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_membership_Instance1: Some(Default::default()),
        market: Some(Default::default()),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![]
        }),
        storage: Some(StorageConfig {
            init_codes: vec![]
        }),
        pallet_treasury: Some(Default::default()),
    }
}

/// The genesis spec of spacex rocky test network
fn rubik_staging_testnet_config_genesis(wasm_binary: &[u8]) -> GenesisConfig {
    // subkey inspect "$SECRET"
    let endowed_accounts: Vec<AccountId> = vec![
        // 5FZdjMwHfF3aXbvasamC91xDdj6PvF76rT8KywEpwHB1FuTj
        hex!["9ac4ed83b7e8a71807536bf1a9cb0865bd419073993e7980489ca43dffa11046"].into(),
        // 5DJnpPJAiq9fmaKNnDwG9323P68p8FBj9dSyprsBigZf84aS
        hex!["36fa5e3a775b5840fc039044c9eecda9d3ae6ba55a7a33c7be4a4e3aac3d0f70"].into(),
        // 5HT4Si96jrM6fN3ohfFRJrXn11CbDttNui2tsHaEvFjz1vsB
        hex!["ee39de2f7678211e7cb72fec510de56d1f8a2d29dd9809f5b79c931db73c4b12"].into(),
        // 5DQ1X73256aGXLjfEEAjtQdD2JQLQGV8enTPFAd9WGozXwo3
        hex!["3af55aa368fa00254a85de996fe687faeac3387ce3859dbc7cabff18d3170833"].into(),
        // 5DoAgLJ7mrwDXxand6sL2XAYP15imWMYo8aehk9GXz8im71h
        hex!["4c9ed4a7a7ebd38e69cb50a8793706103b9fa1d1c3d891537f8f7d02fd672012"].into(),
        // 5F48rNcU9AGFnS1pgmtyfvGGQgHZuA7jeJDLEcuDbkhcdtuY
        hex!["84459866a19c7fadcf55599d17ba5031cfbd6fc7660e63b91d24c4deef6853f8"].into(),
        // 5EqyBCbDWbp8Erxec7TauCeuxvUYaKiqmqAfEKsrM88b1wWA
        hex!["7afe148732d7e363bd77e08cddd4a6cbf93cdd985e10942aba9c43ff799e747e"].into(),
        // 5GfZ3ysS1KudB6B8mM45SmQfm73ZBx9itGTETLuHkmdFk8Si
        hex!["cb84e201eb5e6333ec472e3284ed0343e8c4b4f81f60733d331fcb521242e1ef"].into(),
    ];

    // for i in 1; do for j in {stash, controller}; do subkey inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in authority_discovery; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![(
            // 5FZdjMwHfF3aXbvasamC91xDdj6PvF76rT8KywEpwHB1FuTj
            hex!["9ac4ed83b7e8a71807536bf1a9cb0865bd419073993e7980489ca43dffa11046"].into(),
            // 5DJnpPJAiq9fmaKNnDwG9323P68p8FBj9dSyprsBigZf84aS
            hex!["36fa5e3a775b5840fc039044c9eecda9d3ae6ba55a7a33c7be4a4e3aac3d0f70"].into(),
            // 5FZtbMgrTAHvJ9KnMFLegueCJxq7LSLK8KoheTu1FBjsM6Dh
            hex!["9af6f52d87127c53286c9dd15043b9f322bd935ac7316a2a7f2fb52c1eb884c2"].unchecked_into(),
            // 5Gso9cGhiDDSeXHSuYmtxXYnT24dEYn1KLYJSVov7tRNBhcZ
            hex!["d4db485333e494f8a4fb3252017ae39d07b5e83891b1b402ca9eac67af154346"].unchecked_into(),
            // 5Gso9cGhiDDSeXHSuYmtxXYnT24dEYn1KLYJSVov7tRNBhcZ
            hex!["d4db485333e494f8a4fb3252017ae39d07b5e83891b1b402ca9eac67af154346"].unchecked_into(),
            // 5Gso9cGhiDDSeXHSuYmtxXYnT24dEYn1KLYJSVov7tRNBhcZ
            hex!["d4db485333e494f8a4fb3252017ae39d07b5e83891b1b402ca9eac67af154346"].unchecked_into(),
        ),(
            // 5HT4Si96jrM6fN3ohfFRJrXn11CbDttNui2tsHaEvFjz1vsB
            hex!["ee39de2f7678211e7cb72fec510de56d1f8a2d29dd9809f5b79c931db73c4b12"].into(),
            // 5DQ1X73256aGXLjfEEAjtQdD2JQLQGV8enTPFAd9WGozXwo3
            hex!["3af55aa368fa00254a85de996fe687faeac3387ce3859dbc7cabff18d3170833"].into(),
            // 5Fr1JrHHBJznZowxRvqSjQWNpzYCnpNPPMbNRx5LjKC7uGDR
            hex!["a7417d800df985265f9e8b2b1b384a0e70aca6dba382ac3faaeabde3be8e2119"].unchecked_into(),
            // 5HTjRDDhv1tZMDBzFVDfHm3nayrtoifBSjqaTQ3GgxunxYpF
            hex!["eebd110ef7e544f10dace1ac328c5cf1db73c8f8313e8850c332dfbbaecfdb33"].unchecked_into(),
            // 5Fr1JrHHBJznZowxRvqSjQWNpzYCnpNPPMbNRx5LjKC7uGDR
            hex!["a7417d800df985265f9e8b2b1b384a0e70aca6dba382ac3faaeabde3be8e2119"].unchecked_into(),
            // 5HTjRDDhv1tZMDBzFVDfHm3nayrtoifBSjqaTQ3GgxunxYpF
            hex!["eebd110ef7e544f10dace1ac328c5cf1db73c8f8313e8850c332dfbbaecfdb33"].unchecked_into(),
        ),(
           // 5DoAgLJ7mrwDXxand6sL2XAYP15imWMYo8aehk9GXz8im71h
           hex!["4c9ed4a7a7ebd38e69cb50a8793706103b9fa1d1c3d891537f8f7d02fd672012"].into(),
           // 5F48rNcU9AGFnS1pgmtyfvGGQgHZuA7jeJDLEcuDbkhcdtuY
           hex!["84459866a19c7fadcf55599d17ba5031cfbd6fc7660e63b91d24c4deef6853f8"].into(),
           // 5DqtbLdHdQiLTzZHtTcDyWjqpxSKNjWvEzW4jLuLMt5e7tMH
           hex!["4eb26a8de3cf2e064d8c6a8c3912aa2471c6cd6cfcf005e255b1a18acca00e19"].unchecked_into(),
           // 5FR2CVZgqcP84CDD54d3QGd28xXL7fLgSTMRyENWTRNQJmG2
           hex!["94335a3a9fc18aa1acc6a829c762778ad632250ec3e06795f3b7757835eb482f"].unchecked_into(),
           // 5DqtbLdHdQiLTzZHtTcDyWjqpxSKNjWvEzW4jLuLMt5e7tMH
           hex!["4eb26a8de3cf2e064d8c6a8c3912aa2471c6cd6cfcf005e255b1a18acca00e19"].unchecked_into(),
           // 5FR2CVZgqcP84CDD54d3QGd28xXL7fLgSTMRyENWTRNQJmG2
           hex!["94335a3a9fc18aa1acc6a829c762778ad632250ec3e06795f3b7757835eb482f"].unchecked_into(),
       ),(
           // 5EqyBCbDWbp8Erxec7TauCeuxvUYaKiqmqAfEKsrM88b1wWA
           hex!["7afe148732d7e363bd77e08cddd4a6cbf93cdd985e10942aba9c43ff799e747e"].into(),
           // 5GfZ3ysS1KudB6B8mM45SmQfm73ZBx9itGTETLuHkmdFk8Si
           hex!["cb84e201eb5e6333ec472e3284ed0343e8c4b4f81f60733d331fcb521242e1ef"].into(),
           // 5CsRhpzmw5zW4co8MPafW2Kt84Cc2HGMxNzBxnPE5uAaJATH
           hex!["23a228fce8f7e79f3b4350dd6458d15f626c3ecfbf71dcba5f5766fae7da2e5c"].unchecked_into(),
           // 5Cth6WcUmy2ZMFUthwi1YqGyDHuWs1qPFnQL82xWrD9QohKJ
           hex!["2499376e7db1c15be07b187f8a34d858b0bd4edd91b12c3f7570de95b4c0d900"].unchecked_into(),
           // 5CsRhpzmw5zW4co8MPafW2Kt84Cc2HGMxNzBxnPE5uAaJATH
           hex!["23a228fce8f7e79f3b4350dd6458d15f626c3ecfbf71dcba5f5766fae7da2e5c"].unchecked_into(),
           // 5Cth6WcUmy2ZMFUthwi1YqGyDHuWs1qPFnQL82xWrD9QohKJ
           hex!["2499376e7db1c15be07b187f8a34d858b0bd4edd91b12c3f7570de95b4c0d900"].unchecked_into(),
       )];
    let num_endowed_accounts = endowed_accounts.len();
    // Constants
    const ENDOWMENT: u128 = 125_000_000 * HEIM;
    const STASH: u128 = 1_250_000 * HEIM;

    GenesisConfig {
        pallet_sudo: Some(SudoConfig {
            key: endowed_accounts[0].clone(),
        }),
        frame_system: Some(SystemConfig {
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        balances_Instance1: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig {
            indices: vec![],
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities.iter().map(|x| (
                x.0.clone(),
                x.0.clone(),
                session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
            )).collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            validator_count: 10,
            minimum_validator_count: 1,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: Forcing::NotForcing,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts.iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts.iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_membership_Instance1: Some(Default::default()),
        market: Some(Default::default()),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![]
        }),
        storage: Some(StorageConfig {
            init_codes: vec![]
        }),
        pallet_treasury: Some(Default::default()),
    }
}

/// The genesis spec of spacex mainnet test network
fn mainnet_staging_testnet_config_genesis(wasm_binary: &[u8]) -> GenesisConfig {
    // subkey inspect "$SECRET"
    let endowed_accounts: Vec<AccountId> = vec![
        // cTLTsgfb9aCwEbPrkErFzP8ijhwfkFDTvqBxXa17WQ6dcai7N
        hex!["b6763bf3933231c6bf164e33339ae8a8bfcf6cc08477e47816af30a989810d79"].into(),
        // cTMKoe6bJL1Wud7w9z2mTW1nQwJFspudz68H7W8K1TSXFzzhw
        hex!["1c37d81ef1ebfc2953216a566cf490c7d53db3adaa4aeab15acc4ca2d6577a1d"].into(),
    ];

    // for i in 1; do for j in {stash, controller}; do subkey inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    // for i in 1; do for j in authority_discovery; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![(
        // cTMKoe6bJL1Wud7w9z2mTW1nQwJFspudz68H7W8K1TSXFzzhw
        hex!["1c37d81ef1ebfc2953216a566cf490c7d53db3adaa4aeab15acc4ca2d6577a1d"].into(),
        // cTMedEiTGjKYJw8U1P9R7wPPS3Fwe5KCbiWy5pAkza7CPKubM
        hex!["683a26127e98e79c45f1bb08c6941179e2932b416017c6ac6cb0fd5665d7354e"].into(),
        // cTMKoe6bJL1Wud7w9z2mTW1nQwJFspudz68H7W8K1TSXFzzhw --ed25519
        hex!["ad9996dcf1123ea5a1fc134a2124b958f2faeb16dacebf2923192702b33a8a0c"].unchecked_into(),
        // cTMKoe6bJL1Wud7w9z2mTW1nQwJFspudz68H7W8K1TSXFzzhw --sr25519
        hex!["1c37d81ef1ebfc2953216a566cf490c7d53db3adaa4aeab15acc4ca2d6577a1d"].unchecked_into(),
        // cTMKoe6bJL1Wud7w9z2mTW1nQwJFspudz68H7W8K1TSXFzzhw --sr25519
        hex!["1c37d81ef1ebfc2953216a566cf490c7d53db3adaa4aeab15acc4ca2d6577a1d"].unchecked_into(),
        // cTMKoe6bJL1Wud7w9z2mTW1nQwJFspudz68H7W8K1TSXFzzhw --sr25519
        hex!["1c37d81ef1ebfc2953216a566cf490c7d53db3adaa4aeab15acc4ca2d6577a1d"].unchecked_into(),
    )];
    let num_endowed_accounts = endowed_accounts.len();
    // Constants
    const ENDOWMENT: u128 = 10 * HEIM;
    const STASH: u128 = 10 * HEIM;

    GenesisConfig {
        pallet_sudo: Some(SudoConfig {
            key: endowed_accounts[0].clone(),
        }),
        frame_system: Some(SystemConfig {
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        balances_Instance1: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig {
            indices: vec![],
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities.iter().map(|x| (
                x.0.clone(),
                x.0.clone(),
                session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
            )).collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            validator_count: 1,
            minimum_validator_count: 1,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: Forcing::NotForcing,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts.iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts.iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_membership_Instance1: Some(Default::default()),
        market: Some(Default::default()),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![]
        }),
        storage: Some(StorageConfig {
            init_codes: vec![]
        }),
        pallet_treasury: Some(Default::default()),
    }
}