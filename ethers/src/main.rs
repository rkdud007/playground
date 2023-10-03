use ethers::prelude::*;
use ethers_contract_derive::EthAbiType;
use ethers_core::abi::{AbiType, ParamType};
use ethers_core::types::*;
use eyre::Result;
use std::sync::Arc;
use std::{thread, time};

#[derive(EthEvent, Debug)]
#[ethevent(abi = "OutputProposed(bytes32,uint256,uint256,uint256)")]
struct OutputProposed {
    #[ethevent(indexed, name = "outputRoot")]
    output_root: Bytes,
    #[ethevent(indexed, name = "l2OutputIndex")]
    l1_output_index: I256,
    #[ethevent(indexed, name = "l2BlockNumber")]
    l2_block_number: I256,
    l1_time_stamp: I256,
}

#[derive(EthEvent, Debug)]
#[ethevent(abi = "numberTracker(uint256,string)")]
struct NumberTracker {
    num: U256,
    str: String,
}

abigen!(
    IPROXY,
    r#"[
    function version() view returns (string)
    event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp)
]"#,
);

abigen!(
    IUNISWAP,
    r#"[
    event numberTracker(uint256 num, string str)
]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
    const HTTP_URL: &str = "https://eth-goerli.g.alchemy.com/v2/OxCXO750oi6BTN1kndUMScfn6a16gFIm";
    const OP_PROPOSER_ADDRESS: &str = "0xdfe97868233d1aa22e815a266982f2cf17685a27";
    const UNISWAP_ADDRESS: &str = "0x10D219D705a900a2bBF717DFE54d00cF9F3686c6";

    let provider = Provider::<Http>::try_from(HTTP_URL)?;
    let client = Arc::new(provider);
    //let address: Address = OP_PROPOSER_ADDRESS.parse()?;
    // let contract = IPROXY::new(address, client);
    // let latest_blocknumber = provider.get_block_number().await?;

    let uni_address: Address = UNISWAP_ADDRESS.parse()?;
    //let contract = IUNISWAP::new(uni_address, client);

    // if let Ok(version_info) = contract.version().call().await {
    //     println!("Version info is {version_info:?}");
    // }
    let mut oldblocknum = U64([0]);
    let mut newblocknum = client.get_block_number().await? - 7;
    // instead of put 18000000, put latest block number - 20
    let mut filter = Filter::new()
        .address(uni_address)
        .event("numberTracker(uint256,string)")
        // .topic0(z"event_name")
        .from_block(oldblocknum)
        .to_block(newblocknum);

    // println!("{} pools found!", logs.iter().len());

    let five_sec = time::Duration::from_secs(5);
    loop {
        let logs = client.get_logs(&filter).await?;
        println!("{} pools found!", logs.iter().len());
        println!("ohayo");
        for log in logs.iter() {
            println!("ohayo2");
            // let output_root = Bytes::from(log.topics[1].as_bytes().to_vec());
            // let l2_output_index = U256::from_big_endian(&log.topics[1].as_bytes());
            // let l2_block_number = U256::from_big_endian(&log.topics[2].as_bytes());
            // let l1_timestamp = U256::from_big_endian(&log.data[29..32]);

            // println!(
            //     "output_root = {output_root}, l2OutputIndex = {l2_output_index}, l2BlockNumber = {l2_block_number}, l1Timestamp = {l1_timestamp}",
            // );
            let l1_timestamp = U256::from_big_endian(&log.data[29..32]);
            println!("result {l1_timestamp:?} and block {oldblocknum:?}/{newblocknum:?}")

            // We can get it from Event
            // if let Ok(output_proposal) = contract.get_l2_output(l2OutputIndex).call().await {
            //     println!("output_proposal is {output_proposal:?}");
            // }
        }
        thread::sleep(five_sec);
        oldblocknum = newblocknum;
        newblocknum = client.get_block_number().await? - 7;
        filter = filter.from_block(oldblocknum).to_block(newblocknum);
    }

    // listen_specific_events(&contract).await?;

    Ok(())
}

/// Given a contract instance subscribe to a single type of event.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_specific_events(contract: &IUNISWAP<Provider<Http>>) -> Result<()> {
    let provider = contract.client();
    let latest_blocknumber = provider.get_block_number().await?;
    //println!("ohayo2 {latest_blocknumber:?}");
    // let new_timelag = TimeLag::new(provider, 40);
    // let latest_blocknumber_timelag = new_timelag.get_block_number().await?;
    // // let block_stream = new_timelag.subscribe_blocks().await?;
    // // let latest_blocknumber = provider.get_block_number().await?;
    // // let to_block = latest_blocknumber - 5;
    // println!("ohayo toblock {latest_blocknumber_timelag:?}");
    // let finalized = BlockNumber::Finalized;
    // println!("ohayo finalized block {finalized}");
    let event = contract.event::<NumberTracker>().from_block(0);
    let event_name = "x03eff96be556f3f2723cf37f74bbd4e1cc33a150b8d6c459da9382b55d66a435";
    let mut stream = event.stream().await?;
    let filter = Filter::new()
        .to_block(latest_blocknumber - 5)
        .event(event_name);
    loop {
        let latest_blocknumber = provider.get_block_number().await?;

        // // let to_block = latest_blocknumber - 5;
        // let event = contract
        //     .event::<NumberTracker>()
        //     .from_block(0)
        //     .to_block(latest_blocknumber - 3);

        // let mut stream = event.stream().await?;
        let logs = provider.get_logs(&filter).await?;
        println!("{:?}", logs);
        // match stream.next().await {
        //     Some(result) => println!("ohayo {result:?}, {latest_blocknumber:?}"),
        //     None => {
        //         println!("ohayo, {latest_blocknumber:?}")
        //     }
        // }
    }
}

// use ethers::{abi::AbiDecode, prelude::*, utils::keccak256};
// use eyre::Result;
// use std::sync::Arc;

// #[tokio::main]
// async fn main() -> Result<()> {
//     let client =
//         Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
//             .await?;
//     let client = Arc::new(client);

//     let last_block = client
//         .get_block(BlockNumber::Latest)
//         .await?
//         .unwrap()
//         .number
//         .unwrap();
//     println!("last_block: {}", last_block);

//     let erc20_transfer_filter =
//         Filter::new()
//             .from_block(last_block - 3)
//             .topic0(ValueOrArray::Value(H256::from(keccak256(
//                 "Transfer(address,address,uint256)",
//             ))));

//     let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?;

//     while let Some(log) = stream.next().await {
//         println!(
//             "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
//             log.block_number,
//             log.transaction_hash,
//             log.address,
//             Address::from(log.topics[1]),
//             Address::from(log.topics[2]),
//             U256::decode(log.data)
//         );
//     }

//     Ok(())
// }
