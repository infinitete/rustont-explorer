extern crate mysql;

use std::sync::Arc;
use self::mysql::from_row;
use self::mysql::Pool;

use datastruct::OntCurrent;
use datastruct::OntBlock;
use datastruct::OntTransaction;

pub fn get_block_height(arc: Arc<Pool>) -> OntCurrent {
    let pool = Arc::try_unwrap(arc).unwrap_err();

    let result = pool.first_exec("SELECT height, txncount FROM tbl_ont_current LIMIT 1", ());

    let (height, txncount): (u64, u64) = from_row(result.unwrap().unwrap());

    return OntCurrent {
        height: height,
        txncount: txncount,
    };
}

pub fn get_block_detail<'a>(arc: Arc<Pool>, param: String) -> Option<OntBlock> {
    let pool = Arc::try_unwrap(arc).unwrap_err();

    let sql: &str;

    if param.len() <= 12 {
        sql = "SELECT height, hash, prevblock, nextblock, txnsroot, blocktime, consensusdata, bookkeeper, txnnum, blocksize FROM tbl_ont_block where height = :param";
    } else {
        sql = "SELECT height, hash, prevblock, nextblock, txnsroot, blocktime, consensusdata, bookkeeper, txnnum, blocksize FROM tbl_ont_block where hash = :param";
    }

    let result = pool.first_exec(sql, (params!{"param" => param}));

    match result.unwrap() {
        Some(_res) => {
            let (
                height,
                hash,
                privblock,
                nextblock,
                txnsroot,
                blocktime,
                consensusdata,
                bookkeeper,
                txnnum,
                blocksize,
            ): (
                u64,
                String,
                String,
                String,
                String,
                u64,
                String,
                String,
                u64,
                u64,
            ) = from_row(_res);

            return Some(OntBlock {
                height: height,
                hash: hash,
                privblock: Some(privblock),
                nextblock: Some(nextblock),
                txnsroot: txnsroot,
                blocktime: blocktime,
                consensusdata: consensusdata,
                bookkeeper: bookkeeper,
                txnnum: txnnum,
                blocksize: blocksize,
                transactions: vec![],
            });
        }

        None => None,
    }
}

pub fn get_transactions_for_block(arc: Arc<Pool>, height: u64) -> Vec<OntTransaction> {
    let pool = Arc::try_unwrap(arc).unwrap_err();

    let sql: &str;

    sql = "SELECT txnhash, txntype, txntime, height, fee, description, blockindex, confirmflag FROM tbl_ont_txn_detail where height = :height";

    let result = pool.prep_exec(sql, (params!{"height" => height}));

    let mut transacions = vec![];

    for row in result.unwrap() {
        let (txnhash, txntype, txntime, height, fee, description, blockindex, confirmflag): (String, u8, u64, u64, f64, String, u64, u8) = from_row(row.unwrap());

        transacions.push(OntTransaction {
            txnhash: txnhash,
            txntype: txntype,
            txntime: txntime,
            height: height,
            fee: fee,
            description: description,
            blockindex: blockindex,
            confirmflag: confirmflag,
        });
    }

    transacions.sort_unstable_by(|a, b| a.blockindex.cmp(&b.blockindex));

    transacions
}
