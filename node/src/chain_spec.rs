// Copyright (C) 2019-2021 Spacex Network Technologies Ltd.
// This file is part of Spacex.

use hex_literal::hex;
use sp_core::{Pair, Public, sr25519, crypto::UncheckedInto};
use spacex_runtime::{
    AuthorityDiscoveryId, BalancesConfig, GenesisConfig, ImOnlineId,
    AuthorityDiscoveryConfig, SessionConfig, SessionKeys, StakerStatus,
    StakingConfig, IndicesConfig, SystemConfig, SworkConfig, SudoConfig,
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
        "Development",
        "dev",
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
        "Local Testnet",
        "local_testnet",
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
        move || rocky_staging_testnet_config_genesis(wasm_binary),
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
        market: Some(Default::default()),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![]
        }),
        swork: Some(SworkConfig {
            init_codes: vec![]
        }),
        pallet_treasury: Some(Default::default()),
    }
}

/// The genesis spec of spacex rocky test network
fn rocky_staging_testnet_config_genesis(wasm_binary: &[u8]) -> GenesisConfig {
    // subkey inspect "$SECRET"
    let endowed_accounts: Vec<AccountId> = vec![
        // 5FZdjMwHfF3aXbvasamC91xDdj6PvF76rT8KywEpwHB1FuTj
        hex!["9ac4ed83b7e8a71807536bf1a9cb0865bd419073993e7980489ca43dffa11046"].into(),
        // 5DJnpPJAiq9fmaKNnDwG9323P68p8FBj9dSyprsBigZf84aS
        hex!["36fa5e3a775b5840fc039044c9eecda9d3ae6ba55a7a33c7be4a4e3aac3d0f70"].into(),
        // 5Gso9cGhiDDSeXHSuYmtxXYnT24dEYn1KLYJSVov7tRNBhcZ
        hex!["d4db485333e494f8a4fb3252017ae39d07b5e83891b1b402ca9eac67af154346"].into(),
        // 5FZtbMgrTAHvJ9KnMFLegueCJxq7LSLK8KoheTu1FBjsM6Dh
        hex!["9af6f52d87127c53286c9dd15043b9f322bd935ac7316a2a7f2fb52c1eb884c2"].into(),
        // 5HT4Si96jrM6fN3ohfFRJrXn11CbDttNui2tsHaEvFjz1vsB
        hex!["6d9484ac62fd60762acc9f37da9183398a47b3d867b4e798bf1730ba5e8e449b"].into(),
        // 5DQ1X73256aGXLjfEEAjtQdD2JQLQGV8enTPFAd9WGozXwo3
        hex!["3af55aa368fa00254a85de996fe687faeac3387ce3859dbc7cabff18d3170833"].into()
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
    )];

    // Constants
    const ENDOWMENT: u128 = 2_500_000 * HEIM;
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
        market: Some(Default::default()),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![]
        }),
        swork: Some(SworkConfig {
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
        market: Some(Default::default()),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![]
        }),
        swork: Some(SworkConfig {
            init_codes: vec![]
        }),
        pallet_treasury: Some(Default::default()),
    }
}