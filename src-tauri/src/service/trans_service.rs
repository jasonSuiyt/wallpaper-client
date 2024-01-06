use std::collections::HashMap;

use aes::Aes128;
use anyhow::anyhow;
use base64::{alphabet, Engine, engine};
use base64::engine::general_purpose;
use block_modes::block_padding::Pkcs7;
use block_modes::BlockMode;
use block_modes::Cbc;
use chrono::Local;
use lazy_static::lazy_static;
use md5::Digest;
use reqwest::Client;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue, REFERER, USER_AGENT};
use serde_json::Value;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

lazy_static! {
    static ref API_URL:String = "https://dict.youdao.com/webtranslate".to_string();
    static ref  KEY: Digest = md5::compute("ydsecret://query/key/B*RGygVywfNBwpmBaZg*WT7SIOUP2T0C9WHMZN39j^DAdaZhAnxvGcCY6VYFwnHl");
    static ref  IV: Digest = md5::compute("ydsecret://query/iv/C@lZe2YzHtZ2CYgaXKSVfsb7Y4QWHjITPPZ0nQp87fBeJ!Iv6v^6fvi2WN@bYpJ4");
}

pub async fn translate(q: String) -> anyhow::Result<String> {

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36").unwrap());
    headers.insert(COOKIE, HeaderValue::from_str("OUTFOX_SEARCH_USER_ID_NCOO=976405377.6815147; OUTFOX_SEARCH_USER_ID=-198948307@211.83.126.235; _ga=GA1.2.1162596953.1667349221; search-popup-show=12-2").unwrap());
    headers.insert(REFERER, HeaderValue::from_str("https://fanyi.youdao.com/").unwrap());


    let mystic_time = Local::now().timestamp_millis();
    let key = "fsdsogkndfokasodnaso";
    let point_param = "client,mysticTime,product";
    let key_from = "fanyi.web";
    let app_version = "1.0.0";
    let vendor = "web";
    let client = "fanyideskweb";
    let product = "webfanyi";

    let sign = || {
        let sign_str = format!("client={client}&mysticTime={mystic_time}&product={product}&key={key}");
        let md5_str = format!("{:x}", md5::compute(&sign_str));
        return md5_str;
    };

    let data = HashMap::from([
        ("sign", sign()),
        ("client", client.to_string()),
        ("product", product.to_string()),
        ("appVersion", app_version.to_string()),
        ("vendor", vendor.to_string()),
        ("pointParam", point_param.to_string()),
        ("mysticTime", mystic_time.to_string()),
        ("keyfrom", key_from.to_string()),
        ("i", q.clone()),
        ("form", "AUTO".to_string()),
        ("to", "AUTO".to_string()),
        ("domain", "0".to_string()),
        ("dictResult", "true".to_string()),
        ("keyid", "webfanyi".to_string())
    ]);
    if let Ok(res) = Client::new().post(&API_URL.clone()).headers(headers).form(&data).send().await {
        let str = res.text().await.unwrap();
        let vec = engine::GeneralPurpose::new(
            &alphabet::URL_SAFE,
            general_purpose::PAD)
            .decode(str.trim_end()).unwrap();

        let decryptor = Aes128Cbc::new_from_slices(&KEY.0, &IV.0).unwrap();

        let plaintext = decryptor.decrypt_vec(&vec).unwrap();
        let result = String::from_utf8(plaintext).unwrap();
        let res: Value = serde_json::from_str(&result).unwrap();
        let code: String = res["code"].to_string();
        if code == "0" {
            let res = res["translateResult"][0][0]["tgt"].as_str().unwrap();
            println!("q={} ,res={}", q.clone(), res);
            println!("{}", "-".repeat(100));
            return Ok(res.to_string());
        }else {
            println!("q={} value = {}", q.clone(), res);
            println!("{}", "-".repeat(100));
            return Err(anyhow!(format!("调用翻译接口失败 {}", &q.clone())))
        }
    }
    Err(anyhow!(format!("调用翻译接口失败 {}", &q.clone())))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[actix_rt::test]
    async fn yd_translate_test() {
        for _ in 0..1 {
            if let Ok(res) = translate("Lake Lago Pehoe and Los Cuernos, Torres del Paine National Park, Chile, Patagonia".to_string()).await {
                println!("res: {}", res);
            }
        }
    }
}