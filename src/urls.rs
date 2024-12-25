use log::info;

const MARKET_BROWSER_URL: &'static str = "https://evemarketbrowser.com/region/0/type";
const GATECAMP_URL: &'static str = "https://eve-gatecheck.space/eve/#";
const ESI_SCRAPE_URL: &'static str =
    "https://data.everef.net/esi-scrape/eve-ref-esi-scrape-latest.tar.xz";
const MARKET_DATA_URL: &'static str =
    "https://data.everef.net/market-orders/market-orders-latest.v3.csv.bz2";

pub fn get_esi_scrape_url() -> String {
    ESI_SCRAPE_URL.to_string()
}

pub fn get_market_data_url() -> String {
    MARKET_DATA_URL.to_string()
}

pub fn get_market_browser_url(type_id: u32) -> String {
    info!("Getting market browser url for type id {}", type_id);
    format!("{}/{}", MARKET_BROWSER_URL, type_id)
}

pub fn get_gatecamp_url(path: Vec<u32>, flag: &str) -> String {
    if path.len() < 2 {
        return "".to_string(); // Invalid path
    }

    let mut url = format!("{}{}", GATECAMP_URL, path[0]);

    for system in &path[1..path.len() - 1] {
        url.push(':');
        url.push_str(&system.to_string());
    }

    url.push(',');
    url.push_str(&path[path.len() - 1].to_string());

    url.push(':');
    url.push_str(flag);

    url
}
