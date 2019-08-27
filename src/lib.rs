#![allow(non_snake_case)]

#[macro_use]
extern crate throttled_json_rpc;

use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Balance {
    XRP(BigDecimal),
    Other {
        currency: String,
        issuer: String,
        value: BigDecimal,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/**
* Starts with r
* Length is 25-35 chars in length
* 1: https://xrpl.org/basic-data-types.html#addresses
*/
pub struct Account(String);

fn account_validate(s: &str) -> Result<String, String> {
    const MIN_LENGTH: usize = 25;
    const MAX_LENGTH: usize = 35;
    if s.len() < MIN_LENGTH {
        return Err(format!("{:?} is shorter than {} chars ", s, MIN_LENGTH));
    }
    if let Some(first_char) = s.chars().nth(0) {
        if first_char != 'r' {
            return Err(format!("{:?} does not start with r", s));
        }
    }
    if s.len() > MAX_LENGTH {
        return Err(format!("{:?} is longer than {} chars ", s, MAX_LENGTH));
    }
    Ok(s.into())
}

impl FromStr for Account {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        account_validate(s).map(|account| Account(account))
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AccountInfoParams<'a> {
    pub account: &'a Account,
    pub strict: bool,

    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
    pub queue: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct AccountTxParams<'a, 'b> {
    pub account: &'a Account,
    pub ledger_index_min: Option<i64>,
    pub ledger_index_max: Option<i64>,
    pub ledger_hash: Option<&'b str>,

    #[serde(flatten)]
    pub ledger_index: Option<LedgerIndex>,

    pub binary: Option<bool>,
    pub forward: Option<bool>,
    pub limit: Option<u64>,
}

#[derive(Serialize)]
pub struct LedgerInfoParams {
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<String>,
    pub full: Option<bool>,
    pub accounts: Option<bool>,
    pub transactions: Option<bool>,
    pub expand: Option<bool>,
    pub owner_funds: Option<bool>,
    pub binary: Option<bool>,
    pub queue: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub enum LedgerEntryType {
    AccountRoot, // WHY DOES THIS EVEN EXIST???
}

#[derive(Deserialize, Debug)]
pub struct AccountData {
    pub Account: String,
    pub Balance: BigDecimal,
    pub Flags: BigDecimal,
    pub LedgerEntryType: LedgerEntryType,
    pub OwnerCount: BigDecimal,
    pub PreviousTxnID: String,
    pub PreviousTxnLgrSeq: BigDecimal,
    pub Sequence: BigDecimal,
    pub index: String,
}

#[derive(Deserialize, Debug)]
pub struct QueuedTransaction {
    pub LastLedgerSequence: Option<BigDecimal>,
    pub auth_change: bool,
    pub fee: BigDecimal,
    pub fee_level: BigDecimal,
    pub max_spend_drops: BigDecimal,
    pub seq: BigDecimal,
}

#[derive(Deserialize, Debug)]
pub struct AccountTransaction {
    pub meta: serde_json::Value,
    pub tx: AccountTransactionTx,
    pub validated: bool,
}

#[derive(Deserialize, Debug)]
pub struct AccountTransactionTx {
    pub ledger_index: u64,
}

#[derive(Deserialize, Debug)]
pub struct QueueData {
    pub auth_change_queued: bool,
    pub highest_sequence: BigDecimal,
    pub lowest_sequence: BigDecimal,
    pub max_spend_drops_total: BigDecimal,
    pub transactions: Vec<QueuedTransaction>,
    pub txn_count: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LedgerIndex {
    Current { ledger_current_index: BigDecimal },
    Number { ledger_index: serde_json::Number },
    StrValue { ledger_index: String },
}

#[derive(Deserialize, Debug)]
pub struct AccountInfo {
    pub account_data: AccountData,
    pub queue_data: Option<LaziedQueueData>,
    pub status: String,
    pub validated: Option<bool>,

    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
}

#[derive(Deserialize, Debug)]
pub struct AccountTx {
    pub account: Account,
    pub ledger_index_min: i64,
    pub ledger_index_max: i64,
    pub limit: i64,
    pub transactions: Vec<AccountTransaction>,
}

/**
 *  Some fields may be omitted because the values are calculated "lazily" by the queuing mechanism. [1]
 * 1: https://xrpl.org/account_info.html
 */
#[derive(Deserialize, Debug)]
pub struct LaziedQueueData {
    pub auth_change_queued: Option<bool>,
    pub highest_sequence: Option<BigDecimal>,
    pub lowest_sequence: Option<BigDecimal>,
    pub max_spend_drops_total: Option<BigDecimal>,
    pub transactions: Option<Vec<QueuedTransaction>>,
    pub txn_count: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct PathInfo {
    pub currency: String,
    pub issuer: Option<String>,
    #[serde(rename = "type")]
    pub currency_type: BigDecimal,
    pub type_hex: String,
}

#[derive(Deserialize)]
pub struct FinalFieldInfo {
    pub Account: Option<String>,
    pub Balance: Option<Balance>,
    pub Flags: isize,
    pub OwnerCount: Option<BigDecimal>,
    pub Sequence: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct PreviousFieldInfo {
    pub Balance: Option<Balance>,
    pub Sequence: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct ModifiedNodeInfo {
    pub FinalFields: FinalFieldInfo,
    pub PreviousFields: Option<PreviousFieldInfo>, // is this really optional ???
    pub LedgerEntryType: String,
    pub LedgerIndex: String,
    pub PreviousTxnID: Option<String>,
    pub PreviousTxnLgrSeq: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct AffectedNodeInfo {
    pub ModifiedNode: Option<ModifiedNodeInfo>,
}

#[derive(Deserialize)]
pub struct MetaTxInfo {
    pub AffectedNodes: Vec<AffectedNodeInfo>,
    pub TransactionIndex: BigDecimal,
    pub TransactionResult: String,
}

#[derive(Deserialize)]
pub struct TransactionInfo {
    pub Account: String,
    pub Amount: Option<Balance>,
    pub Destination: Option<String>,
    pub Fee: BigDecimal,
    pub Flags: isize,
    pub Paths: Option<Vec<Vec<PathInfo>>>,
    pub SendMax: Option<Balance>,
    pub Sequence: BigDecimal,
    pub SigningPubKey: String,
    pub TransactionType: String,
    pub TxnSignature: String,
    pub hash: String,
    pub LedgerIndex: Option<String>,
    pub metaData: MetaTxInfo,
    pub validated: Option<bool>, //option of a bool???
}

#[derive(Deserialize)]
pub struct NestedLedgerInfo {
    pub accepted: bool,
    pub account_hash: String,
    pub close_flags: isize,
    pub close_time: BigDecimal,
    pub close_time_human: String,
    pub close_time_resolution: BigDecimal,
    pub closed: bool,
    pub hash: String,
    pub ledger_hash: String,
    pub ledger_index: String,
    pub parent_close_time: BigDecimal,
    pub parent_hash: String,
    pub seqNum: String,
    pub totalCoins: String,
    pub total_coins: BigDecimal,
    pub transaction_hash: String,
    pub transactions: Option<Vec<TransactionInfo>>,
}

#[derive(Deserialize)]
pub struct LedgerInfo {
    pub ledger: NestedLedgerInfo,
    pub ledger_hash: String,
    pub ledger_index: BigDecimal,
    pub transactions: Option<Vec<TransactionInfo>>,
    pub status: String,
    pub validated: bool,
}

jsonrpc_client!(pub struct XRPClient {
    single:
        pub fn account_info(&self, params: AccountInfoParams) -> Result<AccountInfo>;
        pub fn account_tx(&self, params: AccountTxParams) -> Result<AccountTx>;
        pub fn ledger(&self, params: LedgerInfoParams) -> Result<LedgerInfo>;
    enum:
});

#[test]
fn json_test() {
    let _: LedgerInfo =
        serde_json::from_reader(std::fs::File::open("ledger.json").unwrap()).unwrap();
}
