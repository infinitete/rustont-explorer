extern crate mysql;

use std::sync::Arc;
use self::mysql::from_row;
use self::mysql::Pool;

use datastruct::OntCurrent;

pub fn get_block_height(arc: Arc<Pool>) -> OntCurrent {

    let pool = Arc::try_unwrap(arc).unwrap_err();;

    let result = pool.first_exec("SELECT height, txncount FROM tbl_ont_current LIMIT 1", ());

    let (height, txncount): (u64, u64) = from_row(result.unwrap().unwrap());

    return OntCurrent { height: height, txncount: txncount }
}
