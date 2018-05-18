use std::any::TypeId;
use std::cmp::Ordering;

///
/// Const Value
///
const GET_BLOCK_HEIGHT: &'static str = "getblockheight";
const QUERY_BLOCK: &'static str = "QueryBlock";
const VERSION: &'static str = "1.0.0";
const GET_CURRENT: &'static str = "getcurrent";

///
/// Table tbl_ont_block
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OntBlock {
    pub height: u64,
    pub hash: String,
    pub privblock: Option<String>,
    pub nextblock: Option<String>,
    pub txnsroot: String,
    pub blocktime: u64,
    pub consensusdata: String,
    pub bookkeeper: String,
    pub txnnum: u64,
    pub blocksize: u64,
    pub transactions: Vec<OntTransaction>,
}

///
/// Table tbl_ont_current
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OntCurrent {
    pub height: u64,
    pub txncount: u64,
}

///
/// Table tbl_ontid_detail
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OntidDetail {
    pub txnhash: String,
    pub txntype: u8,
    pub ontid: String,
    pub txntime: u64,
    pub height: u64,
    pub description: String,
    pub fee: f64,
}

///
/// Table tbl_ont_txn_detail
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OntTxnDetail {
    pub txnhash: String,
    pub txntype: u8,
    pub txntime: u64,
    pub height: u64,
    pub amount: f64,
    pub fee: f64,
    pub assetname: String,
    pub fromaddress: String,
    pub toaddress: String,
    pub description: String,
    pub blockindex: u64,
    pub txnindex: u64,
    pub confirmflag: u8,
}

///
/// View view_ont_transaction
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OntTransaction {
    pub txnhash: String,
    pub txntype: u8,
    pub txntime: u64,
    pub height: u64,
    pub fee: f64,
    pub description: String,
    pub blockindex: u64,
    pub confirmflag: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeResponse<T> {
    pub Action: Option<&'static str>,
    pub Desc: Option<String>,
    pub Error: Option<u16>,
    pub Result: Option<T>,
    pub Version: &'static str,
}

impl<T> TypeResponse<T>
where
    T: 'static,
{
    pub fn new() -> TypeResponse<T>
    where
        T: 'static,
    {
        let mut response = TypeResponse {
            Action: None,
            Desc: None,
            Error: None,
            Result: None,
            Version: VERSION,
        };

        if TypeId::of::<T>() == TypeId::of::<OntBlock>() {
            response.Action = Some(QUERY_BLOCK);
        }

        if TypeId::of::<T>() == TypeId::of::<OntCurrent>() {
            response.Action = Some(GET_CURRENT);
        }

        response.Error = Some(0);
        response.Desc = Some("Success".to_string());

        response
    }
}
