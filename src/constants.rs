pub struct Venue {
    pub name: &'static str,
    pub rest: &'static str,
    pub fix: &'static str,
}

macro_rules! venues {
    ($($id:ident, $rest:expr, $fix:expr;)+) => {
        $(
            venues!{$id, $rest, $fix}
        )+
    };

    ($id:ident, $rest:expr, $fix:expr) => {
        pub const $id: Venue = Venue {
            name: stringify!($id),
            rest: $rest,
            fix: $fix,
        };
    };
}

venues! {
    BINANCE, "https://apibn.apifiny.com/ac/v2", "fixapibn.apifiny.com:1443";
    BINANCEUS, "https://apibnu.apifiny.com/ac/v2", "fixapibnu.apifiny.com:1443";
    COINBASEPRO, "https://apicb.apifiny.com/ac/v2", "fixapicb.apifiny.com:1443";
    FTX, "https://apiftx.apifiny.com/ac/v2", "fixapiftx.apifiny.com:1443";
    HUOBI, "https://apihb.apifiny.com/ac/v2", "fixapihb.apifiny.com:1443";
    KUCOIN, "https://apikc.apifiny.com/ac/v2", "fixapikc.apifiny.com:1443";
    OKEX, "https://apiok.apifiny.com/ac/v2", "fixapiok.apifiny.com:1443";
    OKCOIN, "https://apiokc.apifiny.com/ac/v2", "fixapiokc.apifiny.com:1443";
}
