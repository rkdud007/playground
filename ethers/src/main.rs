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
abigen!(
    IPROXY,
    r#"[
    function version() view returns (string)
    event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp)
]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
    const HTTP_URL: &str = "";
    const OP_PROPOSER_ADDRESS: &str = "0xdfe97868233d1aa22e815a266982f2cf17685a27";

    let provider = Provider::<Http>::try_from(HTTP_URL)?;
    let client = Arc::new(provider);
    let address: Address = OP_PROPOSER_ADDRESS.parse()?;
    let contract = IPROXY::new(address, client);

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
async fn listen_specific_events(contract: &IPROXY<Provider<Http>>) -> Result<()> {
    let events = contract.event::<OutputProposed>().from_block(0);
    let mut stream = events.stream().await?.take(1);

    while let Some(Ok(f)) = stream.next().await {
        println!("ApprovalFilter event: {f:?}");
    }

    Ok(())
}
