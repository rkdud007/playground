use rlp::Rlp;
use rusqlite::{Connection, Result};
use sqlx::sqlite::SqlitePool;
use std::env::{self, Args};

#[derive(Debug)]
struct Blocks {
    block_number: i32,
    blockheader: Vec<u8>,
}

#[derive(Debug)]
pub struct EvmBlockHeader {
    pub parent_hash: String,
    pub uncle_hash: String,
    pub coinbase: String,
    pub state_root: String,
    pub transactions_root: String,
    pub receipts_root: String,
    pub logs_bloom: String,
    pub difficulty: u64,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: String,
    pub mix_hash: String,
    pub nonce: String,
    pub base_fee_per_gas: Option<u64>,
    pub withdrawals_root: Option<String>,
}

fn blocks_to_evm_blockheader(blocks: Blocks) -> Result<EvmBlockHeader, rusqlite::Error> {
    let mut rlp = Rlp::new(&blocks.blockheader);
    let rlp = Rlp::new(&blocks.blockheader);

    let base_fee_per_gas = if let Ok(val) = rlp.at(15).unwrap().as_val::<Vec<u8>>() {
        Some(u64::from_str_radix(&hex::encode(val), 16).unwrap())
    } else {
        None
    };

    let withdrawals_root = if let Ok(val) = rlp.at(16).unwrap().as_val::<Vec<u8>>() {
        Some(hex::encode(val))
    } else {
        None
    };

    Ok(EvmBlockHeader {
        parent_hash: hex::encode(rlp.at(0).unwrap().as_val::<Vec<u8>>().unwrap()),
        uncle_hash: hex::encode(rlp.at(1).unwrap().as_val::<Vec<u8>>().unwrap()),
        coinbase: hex::encode(rlp.at(2).unwrap().as_val::<Vec<u8>>().unwrap()),
        state_root: hex::encode(rlp.at(3).unwrap().as_val::<Vec<u8>>().unwrap()),
        transactions_root: hex::encode(rlp.at(4).unwrap().as_val::<Vec<u8>>().unwrap()),
        receipts_root: hex::encode(rlp.at(5).unwrap().as_val::<Vec<u8>>().unwrap()),
        logs_bloom: hex::encode(rlp.at(6).unwrap().as_val::<Vec<u8>>().unwrap()),
        difficulty: u64::from_str_radix(
            &hex::encode(rlp.at(7).unwrap().as_val::<Vec<u8>>().unwrap()),
            16,
        )
        .unwrap(),
        number: u64::from_str_radix(
            &hex::encode(rlp.at(8).unwrap().as_val::<Vec<u8>>().unwrap()),
            16,
        )
        .unwrap(),
        gas_limit: u64::from_str_radix(
            &hex::encode(rlp.at(9).unwrap().as_val::<Vec<u8>>().unwrap()),
            16,
        )
        .unwrap(),
        gas_used: u64::from_str_radix(
            &hex::encode(rlp.at(10).unwrap().as_val::<Vec<u8>>().unwrap()),
            16,
        )
        .unwrap(),
        timestamp: u64::from_str_radix(
            &hex::encode(rlp.at(11).unwrap().as_val::<Vec<u8>>().unwrap()),
            16,
        )
        .unwrap(),
        extra_data: hex::encode(rlp.at(12).unwrap().as_val::<Vec<u8>>().unwrap()),
        mix_hash: hex::encode(rlp.at(13).unwrap().as_val::<Vec<u8>>().unwrap()),
        nonce: hex::encode(rlp.at(14).unwrap().as_val::<Vec<u8>>().unwrap()),
        base_fee_per_gas,
        withdrawals_root,
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let path = "blocks_goerli.db";
    let conn = Connection::open(path)?;

    //Uncomment below if the table has not been created yet.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blocks (
            block_number INTEGER PRIMARY KEY,
            blockheader BLOB
        )",
        (),
    )?;

    let mut stmt = conn.prepare("SELECT block_number, blockheader FROM blocks")?;
    let block_iter = stmt.query_map([], |row| {
        Ok(Blocks {
            block_number: row.get(0)?,
            blockheader: row.get(1)?,
        })
    })?;

    for block_result in block_iter {
        let block = block_result.unwrap();
        let block_number = block.block_number;
        println!("block number {:?}", block_number);
        let blockheaderfrom = blocks_to_evm_blockheader(block);
        println!("blockheaderfrom :{:?}", blockheaderfrom);
    }
    Ok(())
}
