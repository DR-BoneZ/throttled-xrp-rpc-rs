use num_traits::cast::ToPrimitive;
use serde_json::json;
use serde_json::value::Value;
use throttled_xrp_rpc::LedgerInfoParams;
use throttled_xrp_rpc::{Account, AccountInfoParams, AccountTxParams, LedgerIndex, XRPClient};

#[macro_use]
extern crate lazy_static;

const FALL_BACK_URL: &'static str = "https://s1.ripple.com:51234/";

lazy_static! {
    static ref URL: String = std::env::var("XRP_NODE").unwrap_or_else(|_| {
        println!("Falling back for the url to {:?}", FALL_BACK_URL);
        FALL_BACK_URL.into()
    });
}

#[test]
fn account_info_tests() {
    let bitpay_account_id: Account = "r9HwsqBnAUN4nF6nDqxd4sgP8DrDnDcZP3".parse().unwrap();
    let client = reqwest::Client::new();

    let account_params = AccountInfoParams {
        account: &bitpay_account_id,
        strict: true,
        ledger_index: LedgerIndex::StrValue {
            ledger_index: "current".into(),
        },
        queue: true,
    };
    let raw_response = client
        .post(&URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response = XRPClient::new(URL.clone().into(), None, None, 0, 0.0, 0)
        .account_info(account_params.clone());
    assert!(
        account_response.is_ok(),
        "Getting back an error {:?} from the server given the input {:?}, raw was {:?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );

    let new_index: i64 = match account_response.unwrap().ledger_index {
        LedgerIndex::Number { ledger_index } => ledger_index,
        LedgerIndex::Current {
            ledger_current_index,
        } => ledger_current_index
            .to_i64()
            .expect("Converting ledger current index into i64")
            .into(),
        a => panic!("Expecting that the type is Number and is instead {:?}", a),
    }
    .as_i64()
    .expect("Unwrap of Number")
        - 5;
    let account_params = AccountInfoParams {
        account: &bitpay_account_id,
        strict: false,
        ledger_index: LedgerIndex::Number {
            ledger_index: new_index.into(),
        },
        queue: false,
    };
    let raw_response = client
        .post(&URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response = XRPClient::new(URL.clone().into(), None, None, 0, 0.0, 0)
        .account_info(account_params.clone());
    assert!(
        account_response.is_ok(),
        "Getting back an error {:#?} from the server given the input {:#?}, raw was {:#?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );
}

#[test]
fn account_tx_test() {
    let bitpay_account_id: Account = "r9HwsqBnAUN4nF6nDqxd4sgP8DrDnDcZP3".parse().unwrap();
    let client = reqwest::Client::new();

    let account_params = AccountTxParams {
        account: &bitpay_account_id,
        binary: Some(false),
        forward: Some(false),
        ledger_hash: None,
        ledger_index: Some(LedgerIndex::StrValue {
            ledger_index: "current".into(),
        }),
        ledger_index_max: Some(-1),
        ledger_index_min: Some(-1),
        limit: Some(2),
    };
    let raw_response = client
        .post(&URL.clone())
        .json(&json!({
        "method": "account_tx",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_tx = XRPClient::new(URL.clone().into(), None, None, 0, 0.0, 0)
        .account_tx(account_params.clone());
    assert!(
        account_tx.is_ok(),
        "Getting back an error {:#?} from the server given the input {:#?}, raw was {:#?}",
        account_tx,
        serde_json::to_string(&account_params),
        raw_response
    );
}

#[test]
fn account_ledger_test() {
    let ledger_params = LedgerInfoParams {
        ledger_hash: None,
        ledger_index: Some(LedgerIndex::StrValue {
            ledger_index: "validated".into(),
        }),
        full: Some(false),
        accounts: Some(false),
        transactions: Some(false),
        expand: Some(false),
        owner_funds: Some(false),
        binary: None,
        queue: None,
    };
    let ledger = XRPClient::new(
        "https://s.altnet.rippletest.net:51234".into(),
        None,
        None,
        0,
        0.0,
        0,
    )
    .ledger(ledger_params.clone());
    assert!(
        ledger.is_ok(),
        "Getting back an error {:#?} from the server given the input {:#?}",
        ledger,
        serde_json::to_string(&ledger_params),
    );
}
