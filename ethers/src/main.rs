use ethers::prelude::*;
use ethers_contract_derive::EthAbiType;
use ethers_core::abi::{AbiType, ParamType};
use ethers_core::types::*;
use eyre::Result;
use std::sync::Arc;

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
#[ethevent(abi = "numberTracker(uint256 num, string str)")]
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
    let address: Address = OP_PROPOSER_ADDRESS.parse()?;
    // let contract = IPROXY::new(address, client);
    // let latest_blocknumber = provider.get_block_number().await?;

    let uni_address: Address = UNISWAP_ADDRESS.parse()?;
    let contract = IUNISWAP::new(uni_address, client);

    // if let Ok(version_info) = contract.version().call().await {
    //     println!("Version info is {version_info:?}");
    // }

    listen_specific_events(&contract).await?;

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
    println!("ohayo2 {latest_blocknumber:?}");
    let new_timelag = TimeLag::new(provider, 40);
    let latest_blocknumber_timelag = new_timelag.get_block_number().await?;
    // let block_stream = new_timelag.subscribe_blocks().await?;
    // let latest_blocknumber = provider.get_block_number().await?;
    // let to_block = latest_blocknumber - 5;
    println!("ohayo toblock {latest_blocknumber_timelag:?}");
    let finalized = BlockNumber::Finalized;
    println!("ohayo finalized block {finalized}");
    let event = contract.event::<NumberTracker>();

    let mut stream = event.stream().await?;
    loop {
        // let latest_blocknumber = provider.get_block_number().await?;
        // let to_block = latest_blocknumber - 5;
        // println!("ohayo toblock {to_block:?}");
        // let new_event =
        // let mut stream = contract.event::<NumberTracker>().to_block(to_block).stream().await?;
        match stream.next().await {
            Some(result) => println!("ohayo {result:?}, {latest_blocknumber:?}"),
            None => {
                println!("ohayo, {latest_blocknumber:?}")
            }
        }
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
