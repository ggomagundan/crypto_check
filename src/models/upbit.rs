use serde::{Deserialize, Serialize};
// https://api.upbit.com/v1/ticker?markets=KRW-ETH
#[derive(Deserialize, Serialize, Debug)]
pub struct UpbitTick {
    market: String,
    trade_date: String,
    trade_time: String,
    trade_date_kst: String,
    trade_time_kst: String,
    trade_timestamp: i64,
    opening_price: f64,
    high_price: f64,
    low_price: f64,
    trade_price: f64,
    prev_closing_price: f64,
    change: String,
    change_price: f64,
    change_rate: f64,
    signed_change_price: f64,
    signed_change_rate: f64,
    trade_volume: f64,
    acc_trade_price: f64,
    acc_trade_price_24h: f64,
    acc_trade_volume: f64,
    acc_trade_volume_24h: f64,
    highest_52_week_price: f64,
    highest_52_week_date: String,
    lowest_52_week_price: f64,
    lowest_52_week_date: String,
    timestamp: i64
}