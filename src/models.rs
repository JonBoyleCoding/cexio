use serde::{Deserialize, Serialize};
use strum_macros::Display;

macro_rules! symbol_enum{
    ($first:ident $(, $sym:ident)*) => {
        #[derive(Display)]
        pub enum Symbol {
            $first,
            $(
                $sym,
            )?
        }
    }
}

macro_rules! currency_struct {
    ($first:ident $(, $curr_name: ident)*) => {

        #[derive(Debug, Default, Deserialize)]
        pub struct $first {
            pub available: String,
            pub orders: String,
        }

        $(
            #[derive(Debug, Default, Deserialize)]
            pub struct $curr_name {
                pub available: String,
                pub orders: String,
            }
        )?
    }
}

macro_rules! balance_currency_def {
    ($first:ident $(, $curr:ident)*) => {
        #[derive(Debug, Deserialize)]
        pub struct BalanceResult {
            pub timestamp: String,
            pub username: String,

           #[serde(default)]
           pub $first: $first,
            $(
               #[serde(default)]
               pub $curr: $curr,
            )?
        }
    }
}

macro_rules! all_def {
    ($first:ident $(, $curr:ident)*) => {
        symbol_enum!($first $(, $curr)?);
        currency_struct!($first $(, $curr)?);
        balance_currency_def!($first $(, $curr)?);
    }
}

use lazy_static::lazy_static;
use serde::de::Visitor;
use std::collections::HashMap;
macro_rules! pairs {
    ($first_b:tt => $first_q:tt $(, $curr_b:tt => $curr_q:tt)*) => {
        lazy_static! {
            pub static ref CURRENCY_PAIRS: Vec<(Symbol, Symbol)> = vec![
                (Symbol::$first_b, Symbol::$first_q),
                $(
                    (Symbol::$curr_b, Symbol::$curr_q),
                )?
            ];
        }
    }
}

all_def!(AAVE,ADA,AKRO,ANT,ATOM,BAL,BAND,BAT,BCH,BCHA,BNT,BTC,BTT,BUSD,COMP,CREAM,CRV
,DAI,DASH,DOT,ETH,EUR,FUN,GAS,GBP,GLM,GUSD,GZIL,HOT,KAVA,KNC,KSM,LAMB,LINK,LTC,MATIC,MHC,MKR,MTA,
MUSD,NEO,OCEAN,OMG,ONG,ONT,PAXG,REN,REPV2,RUB,SNX,SRM,STORJ,SUSHI,TON,TRX,TUSD,UMA,UNI,USD,USDC,
USDT,UTK,WABI,WBTC,XLM,XRP,XTZ,YFI,YFII,ZAP,ZIL,ZRX);

pairs!{BTC => USD, ETH => USD, BCH => USD, DASH => USD, LTC => USD, XRP => USD, XLM => USD,
       OMG => USD, MHC => USD, TRX => USD, BTT => USD, ADA => USD, NEO => USD, GAS => USD,
       BAT => USD, ATOM => USD, XTZ => USD, BTC => EUR, ETH => EUR, BCH => EUR, DASH => EUR,
       XRP => EUR, XLM => EUR, OMG => EUR, MHC => EUR, TRX => EUR, BTT => EUR, LTC => EUR,
       ADA => EUR, NEO => EUR, GAS => EUR, BAT => EUR, ATOM => EUR, XTZ => EUR, BTC => GBP,
       ETH => GBP, BCH => GBP, MHC => GBP, LTC => GBP, XRP => GBP, ADA => GBP, NEO => GBP,
       GAS => GBP, ATOM => GBP, BAT => GBP, XTZ => GBP, BTC => RUB, ETH => BTC, BCH => BTC,
       DASH => BTC, LTC => BTC, XRP => BTC, XLM => BTC, OMG => BTC, GUSD => USD, ONT => USD,
       ONG => USD, GUSD => EUR, ONT => EUR, ONG => EUR, MHC => BTC, MHC => ETH, TRX => BTC,
       BTT => BTC, ONT => BTC, ONG => BTC, USDT => USD, USDT => EUR, USDT => GBP, USDT => RUB,
       BTC => USDT, BTC => USDC, ETH => USDT, BCH => USDT, LTC => USDT, XRP => USDT, XLM => USDT,
       OMG => USDT, TRX => USDT, ONT => USDT, ONG => USDT, ADA => USDT, USDC => USD, WABI => USD,
       WABI => EUR, WABI => GBP, MATIC => USD, MATIC => EUR, MATIC => GBP, MATIC => USDT,
       LINK => USD, LINK => EUR, LINK => GBP, MKR => USD, ZRX => USD, ZRX => USDT, LAMB => USD,
       LAMB => USDT, HOT => USD, HOT => USDT, DASH => USDT, NEO => USDT, GAS => USDT, BAT => USDT,
       ATOM => USDT, XTZ => USDT, GUSD => USDT, USDC => USDT, WABI => USDT, LINK => USDT,
       DOT => USD, DOT => USDT, COMP => USD, COMP => USDT, ZIL => USD, ZIL => USDT, UNI => USD,
       UNI => USDT, UNI => ETH, UMA => USD, UMA => USDT, UMA => ETH, YFI => USD, YFI => USDT,
       YFI => ETH, SNX => USD, SNX => USDT, SNX => ETH, KNC => USD, KNC => USDT, KNC => ETH,
       BAL => USD, BAL => USDT, BAL => ETH, CRV => USD, CRV => USDT, CRV => ETH, WBTC => USDT,
       WBTC => ETH, DAI => USD, DAI => USDT, DAI => ETH, TUSD => USDT, TUSD => ETH, MTA => USDT,
       MTA => ETH, MUSD => USDT, MUSD => ETH, SUSHI => USD, SUSHI => USDT, SUSHI => ETH,
       CREAM => USDT, CREAM => ETH, YFII => USD, YFII => USDT, YFII => ETH, BUSD => USDT,
       BUSD => ETH, REN => USD, REN => USDT, REN => ETH, BAND => USDT, BAND => ETH, AKRO => USDT,
       BNT => USDT, ZAP => USDT, SRM => USDT, ANT => USDT, PAXG => USDT, OCEAN => USDT,
       STORJ => USDT, KAVA => USDT, KSM => USDT, AAVE => USDT, REPV2 => USDT, BCHA => USDT,
       KAVA => USD, GLM => USDT, GLM => USD, GZIL => USDT, GZIL => USD, TON => USDT, TON => USD,
       FUN => USD, UTK => USD, USDC => EUR}

pub fn symbols_to_string(symbols: Vec<Symbol>) -> String {
    symbols
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/")
}

#[derive(Deserialize, Debug)]
pub struct CurrencyLimitsPair {
    pub symbol1: String,
    pub symbol2: String,
    pub minLotSize: f64,
    pub minLotSizeS2: f64,
    // pub maxLotSize: f64,
    pub minPrice: String,
    pub maxPrice: String,
}

#[derive(Deserialize, Debug)]
pub struct CurrencyLimitsData {
    pub pairs: Vec<CurrencyLimitsPair>,
}

#[derive(Deserialize, Debug)]
pub struct CurrencyLimitsResult {
    pub e: String,
    pub ok: String,
    pub data: CurrencyLimitsData,
}
#[derive(Deserialize, Debug)]
pub struct TickerResult {
    pub timestamp: String,
    pub low: String,
    pub high: String,
    pub last: String,
    pub volume: String,
    pub volume30d: String,
    pub bid: f32,
    pub ask: f32,
}

#[derive(Debug, Deserialize)]
pub struct TickerMarketsData {
    pub volume: String,
    pub last: String,
    pub timestamp: String,
    pub bid: f64,
    pub high: String,
    pub ask: f64,
    pub low: String,
    pub pair: String,
    pub volume30d: String,
}

#[derive(Debug, Deserialize)]
pub struct TickerMarketsResult {
    pub ok: String,
    pub e: String,
    pub data: Vec<TickerMarketsData>,
}

#[derive(Deserialize, Debug)]
pub struct LastPriceResult {
    pub lprice: String,
    pub curr1: String,
    pub curr2: String,
}

#[derive(Deserialize, Debug)]
pub struct LastPriceData {
    pub symbol1: String,
    pub symbol2: String,
    pub lprice: String,
}

#[derive(Deserialize, Debug)]
pub struct LastPriceMarketsResult {
    pub e: String,
    pub ok: String,
    pub data: Vec<LastPriceData>,
}

#[derive(Debug, Deserialize)]
pub struct ConvertResult {
    pub amnt: f64,
}

#[derive(Deserialize, Debug)]
pub struct OpenOrderResult {
    pub id: String,
    pub time: String,
    pub r#type: String,
    pub price: String,
    pub amount: String,
    pub pending: String,
    pub symbol1: String,
    pub symbol2: String,
}

#[derive(Deserialize, Debug)]
pub struct CancelOrdersByPairResult {
    pub e: String,
    pub ok: String,
    pub data: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PlaceOrderResult {
    pub complete: bool,
    pub id: String,
    pub time: f64,
    pub pending: String,
    pub amount: String,
    pub r#type: String,
    pub price: String,
}

#[derive(Deserialize, Debug)]
pub struct GetOrderResult {
    pub id: String,
    pub r#type: String,
    pub time: i64,
    pub lastTxTime: String,
    pub lastTx: String,
    //  pub pos: null,
    pub user: String,
    pub status: String,
    pub symbol1: String,
    pub symbol2: String,
    pub amount: String,
    pub price: String,
    // #[serde(rename = "fa:USD")]
    // pub fa_USD: String,
    // #[serde(rename = "ta:USD")]
    // pub ta_USD: String,
    pub remains: String,
    // #[serde(rename = "a:BTC:cds")]
    // pub a_BTC_cds: String,
    // #[serde(rename = "a:USD:cds")]
    // pub a_USD_cds: String,
    // #[serde(rename = "f:USD:cds")]
    // pub f_USD_cds: String,
    pub tradingFeeMaker: String,
    pub tradingFeeTaker: String,
    pub tradingFeeStrategy: String,
    pub orderId: String,
}

#[derive(Deserialize, Debug)]
pub struct PlaceOrderResult2 {
    pub symbol2Amount: String,
    pub symbol1Amount: String,
    pub time: i64,
    pub message: String,
    pub r#type: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct GetOrderTxVtx {
    pub id: String,
    pub r#type: String,
    pub time: String,
    pub user: String,
    pub c: String,
    pub d: String,
    pub a: String,
    pub amount: String,
    pub balance: String,
    pub symbol: String,
    pub order: String,
    // pub buy: null,
    // pub sell: null,
    // pub pair: null,
    // pub pos: null,
    pub cs: String,
    // pub ds: String,
}

#[derive(Deserialize, Debug)]
pub struct GetOrderTxData {
    pub id: String,
    pub r#type: String,
    pub time: i64,
    // lastTxTime: String, // TODO API different
    pub lastTxTime: i64,
    pub lastTx: String,
    pub user: String,
    pub status: String,
    pub symbol1: String,
    pub symbol2: String,
    pub amount: String,
    pub price: String,
    // #[serde(rename = "fa:USD")]
    // fa_USD: String,
    // #[serde(rename = "ta:USD")]
    // ta_USD: String,
    pub remains: String,
    // #[serde(rename = "a:BTC:cds")]
    // a_BTC_cds: String,
    #[serde(rename = "a:USD:cds")]
    pub a_USD_cds: String,
    // #[serde(rename = "f:USD:cds")]
    // f_USD_cds: String,
    pub tradingFeeMaker: String,
    pub tradingFeeTaker: String,
    pub tradingFeeStrategy: String,
    pub orderId: String,
    pub vtx: Vec<GetOrderTxVtx>,
    pub next: bool,
    // pub prev: bool,
}

#[derive(Debug, Default)]
pub struct FullOrder {
    pub id: String,
    pub time: i64,
    pub r#type: String,
    pub kind: String,
    pub price: String,
    pub amount: String,
    pub pending: String,
    pub symbol1: String,
    pub symbol2: String,
    pub symbol1Amount: String,
    pub symbol2Amount: String,
    pub lastTxTime: i64,
    pub lastTx: String,
    pub tradingFeeUserVolumeAmount: String,
    pub totalMakerAmount: String,
    pub totalTakerAmount: String,
    pub feeMakerAmount: String,
    pub feeTakerAmount: String,
    pub creditDebitSalvo: String,
    pub fCreditDebitSalvo: String,
    pub tradingFeeMaker: String,
    pub tradingFeeTaker: String,
    pub tradingFeeStrategy: String,
    pub orderId: String,
    pub remains: String,
    pub pos: String,
    pub status: String,
    pub unknown_fields: HashMap<String, String>,
}

impl<'de> Deserialize<'de> for FullOrder {
    fn deserialize<D>(deserializer: D) -> Result<FullOrder, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Id,
            Time,
            TxnType,
            Kind,
            Price,
            Amount,
            Pending,
            Symbol1,
            Symbol2,
            Symbol1Amount,
            Symbol2Amount,
            LastTxTime,
            LastTx,
            TradingFeeUserVolumeAmount,
            TotalMakerAmount,
            TotalTakerAmount,
            FeeMakerAmount,
            FeeTakerAmount,
            CreditDebitSalvo,
            FCreditDebitSalvo,
            TradingFeeMaker,
            TradingFeeTaker,
            TradingFeeStrategy,
            OrderId,
            Remains,
            Pos,
            Status,
            Unknown(String),
        }

        // Implementing a Field Deserialiser
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "A FullOrder struct as defined by CEX.IO Rest API")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: serde::de::Error,
                    {
                        if value.starts_with("ta:") {
                            return Ok(Field::TotalMakerAmount);
                        } else if value.starts_with("tta:") {
                            return Ok(Field::TotalTakerAmount);
                        } else if value.starts_with("fa:") {
                            return Ok(Field::FeeMakerAmount);
                        } else if value.starts_with("tfa:") {
                            return Ok(Field::FeeTakerAmount);
                        } else if value.starts_with("a:") {
                            return Ok(Field::CreditDebitSalvo);
                        } else if value.starts_with("f:") {
                            return Ok(Field::FCreditDebitSalvo);
                        }

                        const standard_fields: [&str; 27] = [
                            "id",
                            "time",
                            "type",
                            "kind",
                            "price",
                            "amount",
                            "pending",
                            "symbol1",
                            "symbol2",
                            "symbol1Amount",
                            "symbol2Amount",
                            "lastTxTime",
                            "lastTx",
                            "tradingFeeUserVolumeAmount",
                            "tradingFeeMaker",
                            "tradingFeeTaker",
                            "tradingFeeStrategy",
                            "orderId",
                            "remains",
                            "pos",
                            "status",
                            "ta:{symbol2}",
                            "tta:{symbol2}",
                            "fa:{symbol2}",
                            "tfa:{symbol2}",
                            "a:{symbol1}:cds",
                            "f:{symbol2:cds",
                        ];

                        match value {
                            "id" => Ok(Field::Id),
                            "time" => Ok(Field::Time),
                            "type" => Ok(Field::TxnType),
                            "kind" => Ok(Field::Kind),
                            "price" => Ok(Field::Price),
                            "amount" => Ok(Field::Amount),
                            "pending" => Ok(Field::Pending),
                            "symbol1" => Ok(Field::Symbol1),
                            "symbol2" => Ok(Field::Symbol2),
                            "symbol1Amount" => Ok(Field::Symbol1Amount),
                            "symbol2Amount" => Ok(Field::Symbol2Amount),
                            "lastTxTime" => Ok(Field::LastTxTime),
                            "lastTx" => Ok(Field::LastTx),
                            "tradingFeeUserVolumeAmount" => Ok(Field::TradingFeeUserVolumeAmount),
                            "tradingFeeMaker" => Ok(Field::TradingFeeMaker),
                            "tradingFeeTaker" => Ok(Field::TradingFeeTaker),
                            "tradingFeeStrategy" => Ok(Field::TradingFeeStrategy),
                            "orderId" => Ok(Field::OrderId),
                            "remains" => Ok(Field::Remains),
                            "pos" => Ok(Field::Pos),
                            "status" => Ok(Field::Status),
                            _ => Ok(Field::Unknown(value.to_string())),
                        }
                    }
                }


                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct OrderVisitor;

        impl<'de> serde::de::Visitor<'de> for OrderVisitor {
            type Value = FullOrder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A FullOrder struct as defined by CEX.IO Rest API")
            }

            fn visit_map<V>(self, mut map: V) -> Result<FullOrder, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut return_order = FullOrder::default();

                while let Some(key) = map.next_key()? {

                    let next_value: Option<String> = map.next_value()?;

                    let next_value = match next_value {
                        Some(n) => n,
                        None => "".to_string(),
                    };

                    match key {
                        Field::Id => {return_order.id = next_value;},
                        Field::Time => {return_order.time = chrono::DateTime::parse_from_rfc3339(&next_value).unwrap().timestamp();},
                        Field::TxnType => {return_order.r#type = next_value;},
                        Field::Kind => {return_order.kind = next_value;}
                        Field::Price => {return_order.price = next_value;},
                        Field::Amount => {return_order.amount = next_value;},
                        Field::Pending => {return_order.pending = next_value;},
                        Field::Symbol1 => {return_order.symbol1 = next_value;},
                        Field::Symbol2 => {return_order.symbol2 = next_value;},
                        Field::Symbol1Amount => {return_order.symbol1Amount = next_value;},
                        Field::Symbol2Amount => {return_order.symbol2Amount = next_value;},
                        Field::LastTxTime => {return_order.lastTxTime = chrono::DateTime::parse_from_rfc3339(&next_value).unwrap().timestamp();},
                        Field::LastTx => {return_order.lastTx = next_value;},
                        Field::TradingFeeUserVolumeAmount => {return_order.tradingFeeUserVolumeAmount = next_value;},
                        Field::TotalMakerAmount => {return_order.totalMakerAmount = next_value;},
                        Field::TotalTakerAmount => {return_order.totalTakerAmount = next_value;},
                        Field::FeeMakerAmount => {return_order.feeMakerAmount= next_value;},
                        Field::FeeTakerAmount => {return_order.feeTakerAmount= next_value;},
                        Field::CreditDebitSalvo => {return_order.creditDebitSalvo = next_value;},
                        Field::FCreditDebitSalvo => {return_order.fCreditDebitSalvo = next_value;},
                        Field::TradingFeeMaker => {return_order.tradingFeeMaker = next_value;},
                        Field::TradingFeeTaker => {return_order.tradingFeeTaker = next_value;},
                        Field::TradingFeeStrategy => {return_order.tradingFeeStrategy = next_value;},
                        Field::OrderId => {return_order.orderId = next_value;},
                        Field::Remains => {return_order.remains = next_value;},
                        Field::Pos => {return_order.pos = next_value;},
                        Field::Status => {return_order.status = next_value;},
                        Field::Unknown(t) => {return_order.unknown_fields.insert(t, next_value);}
                    }
                }

                Ok(return_order)
            }
        }

        deserializer.deserialize_struct("FullOrder", &[], OrderVisitor)

    }
}

#[derive(Deserialize, Debug)]
pub struct GetOrderTxResult {
    pub e: String,
    pub ok: String,
    pub data: GetOrderTxData,
}

#[derive(Deserialize, Debug)]
pub struct ActiveOrderStatusResult {
    pub e: String,
    pub ok: String,
    pub data: Vec<[String; 3]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BTC_USD {
    pub buy: String,
    pub sell: String,
    pub buyMaker: String,
    pub sellMaker: String,
}

#[derive(Deserialize, Debug)]
pub struct DataMyfee {
    #[serde(rename = "BTC:USD")]
    pub BTC_USD: BTC_USD,
}

#[derive(Deserialize, Debug)]
pub struct GetMyfeeResult {
    pub e: String,
    pub ok: String,
    pub data: DataMyfee,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAddressResult {
    pub ok: String,
    pub e: String,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct LongShort {
    #[serde(rename = "2")]
    _2: Vec<String>,
    #[serde(rename = "3")]
    _3: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct BTCUSDMarginalFee {
    short: LongShort,
    long: LongShort,
}

#[derive(Deserialize, Debug)]
struct DataMarginalFee {
    #[serde(rename = "BTC:USD")]
    BTC_USD: BTCUSDMarginalFee,
}

#[derive(Deserialize, Debug)]
pub struct GetMarginalFeeResult {
    e: String,
    ok: String,
    data: DataMarginalFee,
}

#[derive(Deserialize, Debug)]
pub struct TradeHistoryResult {
    pub r#type: String,
    pub date: String,
    pub amount: String,
    pub price: String,
    pub tid: String,
}

#[derive(Deserialize, Debug)]
pub struct OhlcvResult {
    pub time: i32,
    pub data1m: String,
    pub data1h: String,
    pub data1d: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookResult {
    pub timestamp: i64,
    pub bids: Vec<[f32; 2]>,
    pub asks: Vec<[f32; 2]>,
    pub pair: String,
    pub id: i64,
    pub sell_total: String,
    pub buy_total: String,
}

#[derive(Deserialize, Debug)]
pub struct PriceStatsResult {
    pub tmsp: i64,
    pub price: String,
}
