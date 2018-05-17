///
/// Table tbl_ont_block
///
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
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
    pub blocksize: u64
}

///
/// Table tbl_ont_current
///
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct OntCurrent {
    pub height: u64,
    pub txncount: u64
}

///
/// Table tbl_ontid_detail
///
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct OntidDetail {
    pub txnhash: String,
    pub txntype: u16,
    pub ontid: String,
    pub txntime: u64,
    pub height: u64,
    pub description: String,
    pub fee: f64
}

///
/// Table tbl_ont_txn_detail
///
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct OntTxnDetail {
    pub txnhash: String,
    pub txnty: u16,
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
    pub confirmflag: u8
}

///
/// View view_ont_transaction
///
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct OntTransaction {
    pub txnhash: String,
    pub txnty: u16,
    pub txntime: u64,
    pub height: u64,
    pub fee: f64,
    pub description: String,
    pub blockindex: u64,
    pub confirmflag: u8
}
