// buybit.rs - author: steinkirch

use std::env;
use std::io::{self, Write};
use bybit::linear::{PublicResponse, PublicWebSocketApiClient};
use bybit::inverse::{PrivateResponse, PrivateWebSocketApiClient};
use bybit::spot::ws::{OrderBookItem, PublicV2Response, PublicV2WebSocketApiClient,
                      PublicResponse as OtherPublicResponse, 
                      PublicWebSocketApiClient as OtherPublicWebSocketApiClient, 
                      PrivateResponse as OtherPrivateResponse, 
                      PrivateWebSocketApiClient as OtherPrivateWebSocketApiClient};


struct OwnedOrderBookItem(String, String);


pub async fn subscribe_coin() {

    let coin = &env::var("COIN").expect("⛔️ COIN must be set on .env file");
    let mut client = PublicV2WebSocketApiClient::new();
    println!("🐊 subcribing to websockets for: {:?} \n", coin);

    client.subscribe_depth(coin, false);
    client.subscribe_trade(coin, false);
    client.subscribe_book_ticker(coin, false);
    client.subscribe_realtimes(coin, false);

    let callback = |res: PublicV2Response| match res {
        PublicV2Response::Depth(res) => println!("✅ depth: {:?}", res),
        PublicV2Response::Kline(res) => println!("✅ kline: {:?}", res),
        PublicV2Response::Trade(res) => println!("✅ trade: {:?}", res),
        PublicV2Response::BookTicker(res) => println!("✅ book ticker: {:?}", res),
        PublicV2Response::Realtimes(res) => println!("✅ realtimes: {:?}", res),
        PublicV2Response::Pong(res) => println!("✅ pong: {:?}", res),
        PublicV2Response::Ping(res) => println!("✅ ping: {:?}", res),
    };


    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}


pub async fn subscribe_pairs() {
    
    let pairs = &env::var("PAIRS").expect("⛔️ PAIRS must be set on .env file");
    let symbols: Vec<&str> = pairs.split(",").collect();
    println!("🐊 subcribing to websockets for: {:?} \n", symbols);

    let mut client = PublicWebSocketApiClient::new();

    client.subscribe_order_book_l2_25(&symbols);
    client.subscribe_order_book_l2_200(&symbols);
    client.subscribe_trade(&symbols);
    client.subscribe_instrument_info(&symbols);
    client.subscribe_kline(&symbols, "1");
    client.subscribe_liquidation(&symbols);

    let callback = |res: PublicResponse| match res {
        PublicResponse::OrderBookL2Snapshot(res) => println!("✅ order book L2 snapshot: {:?}", res),
        PublicResponse::OrderBookL2Delta(res) => println!("✅ order book L2 delta: {:?}", res),
        PublicResponse::Trade(res) => println!("✅ trade: {:?}", res),
        PublicResponse::InstrumentInfoSnapshot(res) => {
            println!("✅ instrument info snapshot: {:?}", res)
        }
        PublicResponse::InstrumentInfoDelta(res) => {
            println!("✅ instrument info delta: {:?}", res)
        }
        PublicResponse::Kline(res) => println!("✅ kline: {:?}", res),
        PublicResponse::Liquidation(res) => println!("✅ liquidation: {:?}", res),
    };


    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),    
    }
}


pub async fn subscribe_exec() {
    
    let api_key = &env::var("BYBIT_API_KEY").expect("⛔️ BYBIT_API_KEY must be set on .env file");
    let api_secret = &env::var("BYBIT_API_SECRET").expect("⛔️ BYBIT_API_SECRET must be set on .env file");

    println!("🐊 subcribing to private websockets: \n");

    let client = OtherPrivateWebSocketApiClient::builder().testnet()
                                .build_with_credentials(&api_key, &api_secret);

    let callback = |res: OtherPrivateResponse| match res {
        OtherPrivateResponse::ExecutionReportSequence(seq) => println!("✅ execution report: {:?}", seq),
        OtherPrivateResponse::TicketInfoSequence(seq) => println!("✅ ticket info: {:?}", seq),
        OtherPrivateResponse::OutboundAccountInfoSequence(seq) => {
            println!("✅ outbound account info: {:?}", seq)
        },
        OtherPrivateResponse::Pong(res) => println!("✅ pong: {:?}", res),
        OtherPrivateResponse::Ping(res) => println!("✅ ping: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}


pub async fn subscribe_positions() {
    
    let api_key = &env::var("BYBIT_API_KEY").expect("⛔️ BYBIT_API_KEY must be set on .env file");
    let api_secret = &env::var("BYBIT_API_SECRET").expect("⛔️ BYBIT_API_SECRET must be set on .env file");

    println!("🐊 subcribing to private positions websockets: \n");

    let mut client = PrivateWebSocketApiClient::new(api_key, api_secret);

    client.subscribe_position();
    client.subscribe_execution();
    client.subscribe_order();
    client.subscribe_stop_order();
    client.subscribe_wallet();

    let callback = |res: PrivateResponse| match res {
        PrivateResponse::Position(res) => println!("✅ position: {:?}", res),
        PrivateResponse::Execution(res) => println!("✅ execution: {:?}", res),
        PrivateResponse::Order(res) => println!("✅ order: {:?}", res),
        PrivateResponse::StopOrder(res) => println!("✅ stop order: {:?}", res),
        PrivateResponse::Wallet(res) => println!("✅ wallet: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}


pub async fn subscribe_spot () {

    let coin = &env::var("COIN").expect("⛔️ COIN must be set on .env file");
    let mut client = OtherPublicWebSocketApiClient::new();

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    let mut latest_price: String = String::new();
    let mut direction = "🔺";
    let mut asks: Vec<OwnedOrderBookItem> = Vec::new();
    let mut bids: Vec<OwnedOrderBookItem> = Vec::new();

    client.subscribe_trade(coin, false);
    client.subscribe_diff_depth(coin, false);

    let callback = |res: OtherPublicResponse| {
        match res {
            OtherPublicResponse::Trade(res) => {
                let price = res.data[0].p.to_owned();
                if price < latest_price {
                    direction = "🔻";
                } else if price > latest_price {
                    direction = "🔺";
                }
                latest_price = price
            }
            OtherPublicResponse::Depth(res) => {
                res.data[0].a.iter().for_each(|&OrderBookItem(price, qty)| {
                    asks.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                });
                res.data[0].b.iter().for_each(|&OrderBookItem(price, qty)| {
                    bids.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                });
            }

            OtherPublicResponse::DiffDepth(res) => {
            
                ////////////
                // ASKS
                ////////////
                let a = &res.data[0].a;
                let mut i: usize = 0;
                let mut j: usize = 0;

                while i < a.len() {
                    let OrderBookItem(price, qty) = a[i];

                    while j < asks.len() {
                        let item = &mut asks[j];
                        let item_price: &str = &item.0;
                        if price < item_price {
                            asks.insert(j, OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                            i += 1;
                            j += 1;
                            break;
                        }

                        if price == item_price {
                            if qty != "0" {
                                item.1 = qty.to_owned();
                                i += 1;
                                j += 1;
                            } else {
                                asks.remove(j);
                                i += 1;
                            }
                            break;
                        }

                        j += 1;
                    }

                    if j == asks.len() {
                        a.iter().skip(i).for_each(|&OrderBookItem(price, qty)| {
                            asks.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                        });
                        break;
                    }
                }

                ////////////
                // BIDS
                ////////////
                let b = &res.data[0].b;
                let mut i: usize = 0;
                let mut j: usize = 0;

                while i < b.len() {
                    let OrderBookItem(price, qty) = b[i];

                    while j < bids.len() {
                        let item = &mut bids[j];
                        let item_price: &str = &item.0;
                        if price > item_price {
                            bids.insert(j, OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                            i += 1;
                            j += 1;
                            break;
                        }

                        if price == item_price {
                            if qty != "0" {
                                item.1 = qty.to_owned();
                                i += 1;
                                j += 1;
                            } else {
                                bids.remove(j);
                                i += 1;
                            }
                            break;
                        }

                        j += 1;
                    }

                    if j == bids.len() {
                        b.iter().skip(i).for_each(|&OrderBookItem(price, qty)| {
                            bids.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                        });
                        break;
                    }
                }
            }
            _ => {}
        }

        ////////////
        // ASKS
        ////////////
        write!(handle, "\n✨🐊{} orderbook\n\n", coin).unwrap();
        write!(handle, "{:<20} {:<20}\n", "💰 price", "🛍 quantity").unwrap();
        let mut asks_10 = asks.iter().take(10).collect::<Vec<_>>().clone();
        asks_10.reverse();
        asks_10.iter().for_each(|item| {
            write!(handle, "{:<20} {:<20}\n", item.0, item.1).unwrap();
        });
        write!(handle, "\n{} {}\n\n", direction, latest_price).unwrap();
        bids.iter().take(10).for_each(|item| {
            write!(handle, "{:<20} {:<20}\n", item.0, item.1).unwrap();
        });
        handle.flush().unwrap();
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

}
