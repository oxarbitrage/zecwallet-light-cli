//! Zebra integration tests
//!
//! This tests will only work if there is a lightwalletd running (backed with zebra) in `LIGHTWALLETD_SERVER`

use tonic::transport::Channel;
use tonic::Request;

use crate::compact_formats::compact_tx_streamer_client::CompactTxStreamerClient;

use crate::compact_formats::{AddressList, BlockId, BlockRange, Empty, TransparentAddressBlockFilter};

const LIGHTWALLETD_SERVER: &str = "http://127.0.0.1:8081";

#[tokio::test]
async fn zebra_backed_lightwalletd() {
    // get a client
    let uri: http::Uri = LIGHTWALLETD_SERVER.parse().unwrap();
    let mut client = CompactTxStreamerClient::new(Channel::builder(uri).connect().await.unwrap());

    // test get_lightd_info
    let response = client
        .get_lightd_info(Request::new(Empty {}))
        .await
        .unwrap()
        .into_inner();
    println!("{:?}", response);

    // test get_current_zec_price
    let response = client.get_current_zec_price(Empty {}).await.unwrap().into_inner();
    println!("{:?}", response);

    // build some block ids and range
    let block_start = BlockId {
        height: 1,
        hash: vec![],
    };

    let block_end = BlockId {
        height: 1500,
        hash: vec![],
    };

    let block_range = BlockRange {
        start: Some(block_start),
        end: Some(block_end.clone()),
    };

    // test get_block
    let response = client.get_block(block_end).await.unwrap().into_inner();
    println!("{:?}", response);

    // build some address types
    let address = AddressList {
        addresses: vec!["t3Vz22vK5z2LcKEdg16Yv4FFneEL1zg9ojd".to_string()],
    };

    let address_and_block = TransparentAddressBlockFilter {
        address: "t3Vz22vK5z2LcKEdg16Yv4FFneEL1zg9ojd".to_string(),
        range: Some(block_range),
    };

    // test get_taddress_txids
    let response = client.get_taddress_txids(address_and_block).await.unwrap().into_inner();

    println!("{:?}", response);

    // test get_taddress_balance
    let response = client.get_taddress_balance(address).await.unwrap().into_inner();

    println!("{:?}", response);
}
