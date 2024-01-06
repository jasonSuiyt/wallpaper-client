use std::fs;
use std::path::Path;
use std::time::Duration;
use anyhow::Context;
use chrono::{Local, NaiveDate};
use htmler::Selector;
use lazy_static::lazy_static;
use reqwest::StatusCode;
use crate::dao::models::NewBing;

lazy_static! {
    static ref SPOTLIGHT_UHD_PATH: String = format!("{}{}", env!("HOME"), "/wallpaper/spotlight/images/uhd/");
    static ref SPOTLIGHT_NORMAL_PATH: String = format!("{}{}", env!("HOME"), "/wallpaper/spotlight/images/normal/");

    static ref SPOTLIGHT_ARTICLE_SELECTOR: Selector = htmler::Selector::parse(r#"article"#).unwrap();
    static ref SPOTLIGHT_DATE_SELECTOR: Selector = htmler::Selector::parse(r#"span[class="date"]"#).unwrap();
    static ref SPOTLIGHT_NAME_SELECTOR: Selector = htmler::Selector::parse(r#"span[class="entry-title hidden"]"#).unwrap();
    static ref SPOTLIGHT_IMG_SELECTOR: Selector = htmler::Selector::parse(r#"img"#).unwrap();
    static ref SPOTLIGHT_PAGE_NUMBERS_SELECTOR: Selector = htmler::Selector::parse(r#"a[class="page-numbers"]"#).unwrap();


    static ref BING_UHD_PATH: String = format!("{}{}",  env!("HOME"),  "/wallpaper/bing/images/uhd/");
    static ref BING_NORMAL_PATH: String = format!("{}{}", env!("HOME"), "/wallpaper/bing/images/normal/");

    static ref BING_IMG_LIST_SELECTOR: Selector = htmler::Selector::parse(r#"div[class="col-md-6 col-lg-4"]"#).unwrap();
    static ref BING_IMG_ROW_SELECTOR: Selector = htmler::Selector::parse(r#"div[class="image-list__container"]"#).unwrap();
    static ref BING_PIC_SELECTOR: Selector = htmler::Selector::parse(r#"div[class="image-list__picture lazyload"]"#).unwrap();
    static ref BING_DESC_SELECTOR: Selector = htmler::Selector::parse(r#"a[class="image-list__link"]"#).unwrap();
    static ref BING_DATE_SELECTOR: Selector = htmler::Selector::parse(r#"span[class="text-gray"]"#).unwrap();
    static ref BING_PAGE_NUMBERS_SELECTOR: Selector = htmler::Selector::parse(r#"a[class="page-link"]"#).unwrap();


    static ref WALL_UHD_PATH: String = format!("{}{}", env!("HOME") ,"/wallpaper/wall/images/uhd/");
    static ref WALL_NORMAL_PATH: String =format!("{}{}", env!("HOME") , "/wallpaper/wall/images/normal/");

    static ref WALL_PAGE_NUMBERS_SELECTOR: Selector = htmler::Selector::parse(r#"p[class="pages"]"#).unwrap();
    static ref WALL_PICS_SELECTOR: Selector = htmler::Selector::parse(r#"div[class="pics"]"#).unwrap();
    static ref WALL_PICS_VEC_SELECTOR: Selector = htmler::Selector::parse(r#"p"#).unwrap();
    static ref WALL_URL: String = "https://wallpapershome.com".to_string();
    static ref HTTP_TIME_OUT: Duration = Duration::from_secs(360);

}

pub async fn get_total_page(source: &str) -> i32 {
    if source == "bing" {
        return bing_total_page().await;
    } else if source == "spotlight" {
        return spotlight_total_page().await;
    } else if source == "wallpapers" {
        return wallpapers_total_page().await;
    }
    return 0;
}

pub async fn spotlight_total_page() -> i32 {
    let client = reqwest::Client::new();
    if let Ok(res) = client.get("https://windows10spotlight.com/page/1").timeout(*HTTP_TIME_OUT).send().await {
        if res.status() == StatusCode::OK {
            let html_dom = res.text().await.unwrap();
            let html = htmler::Html::parse_fragment(&html_dom);
            if let Some(total_page) = html.select(&SPOTLIGHT_PAGE_NUMBERS_SELECTOR).map(|x| x.inner_html()).map(|x| x.replace(",", "").parse::<i32>().unwrap()).max() {
                return total_page;
            }
        }
    }
    return 1;
}


pub async fn spotlight_request(page_size: i32) -> anyhow::Result<Vec<NewBing>, anyhow::Error> {
    let client = reqwest::Client::new();
    let mut new_bing_vec: Vec<NewBing> = vec![];

    create_dir(SPOTLIGHT_UHD_PATH.clone());
    create_dir(SPOTLIGHT_NORMAL_PATH.clone());

    if let Ok(res) = client.get(format!("https://windows10spotlight.com/page/{}", &page_size)).timeout(*HTTP_TIME_OUT).send().await {
        if res.status() == StatusCode::OK {
            let html_dom = res.text().await.unwrap();
            let html = htmler::Html::parse_fragment(&html_dom);
            for article in html.select(&SPOTLIGHT_ARTICLE_SELECTOR) {
                let name = article.select(&SPOTLIGHT_NAME_SELECTOR).next().with_context(|| format!("read name url err {page_size}"))?.inner_html();

                let date = article.select(&SPOTLIGHT_DATE_SELECTOR).next().with_context(|| format!("read date url err {page_size}"))?.inner_html();
                if let None = article.select(&SPOTLIGHT_IMG_SELECTOR).next() {
                    println!("{}", article.inner_html());
                }
                if let Some(img_node) = article.select(&SPOTLIGHT_IMG_SELECTOR).next() {
                    let img = img_node.get_attribute("srcset").split(",").map(|x| x.to_string()).collect::<Vec<String>>();
                    let normal_img_url = img.get(1).unwrap().split_whitespace().next().unwrap();
                    let uhd_img_url = img.get(2).unwrap().split_whitespace().next().unwrap();
                    let img_name = uhd_img_url.split("/").last().unwrap();


                    let uhd_file_path = format!("{}{}",SPOTLIGHT_UHD_PATH.clone(), img_name);
                    let normal_file_path = format!("{}{}",SPOTLIGHT_NORMAL_PATH.clone(), img_name);

                    new_bing_vec.push(NewBing {
                        name,
                        url: normal_img_url.to_string(),
                        uhd_url: uhd_img_url.to_string(),
                        uhd_file_path,
                        source: "spotlight".to_string(),
                        normal_file_path: normal_file_path.clone(),
                        created_date: NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap(),
                    });
                }
            }
        }
    }

    Ok(new_bing_vec)
}


pub async fn bing_total_page() -> i32 {
    let client = reqwest::Client::new();
    if let Ok(res) = client.get("https://peapix.com/bing/cn").timeout(*HTTP_TIME_OUT).send().await {
        if res.status() == StatusCode::OK {
            let html_dom = res.text().await.unwrap();
            let html = htmler::Html::parse_fragment(&html_dom);
            if let Some(node) = html.select(&BING_PAGE_NUMBERS_SELECTOR).last() {
                return node.inner_html().parse::<i32>().unwrap();
            }
        }
    }
    return 1;
}

pub async fn wallpapers_total_page() -> i32 {
    let client = reqwest::Client::new();
    if let Ok(res) = client.get("https://wallpapershome.com/girls/?page=1").timeout(*HTTP_TIME_OUT).send().await {
        if res.status() == StatusCode::OK {
            let html_dom = res.text().await.unwrap();
            let html = htmler::Html::parse_fragment(&html_dom);
            let pages = html.select(&WALL_PAGE_NUMBERS_SELECTOR).next().unwrap();
            if let Some(node) = pages.select(&Selector::parse(r#"a"#).unwrap()).filter(|x| x.get_attribute("class") == "").last() {
                return node.inner_html().parse::<i32>().unwrap();
            }
        }
    }
    return 1;
}

#[derive(Debug, Clone)]
struct WallModal {
    pub name: String,
    pub second_url: String,
    pub normal_img_url: String,
}

pub(crate) async fn wallpapers_request(page_size: i32) -> anyhow::Result<Vec<NewBing>, anyhow::Error> {
    let client = reqwest::Client::new();

    let mut new_bing_vec: Vec<NewBing> = vec![];
    let mut wall_vec: Vec<WallModal> = vec![];

    create_dir(WALL_UHD_PATH.to_string());
    create_dir(WALL_NORMAL_PATH.to_string());

    if let Ok(res) = client.get(format!("{}/girls/?page={}", WALL_URL.to_string(), &page_size.to_string())).timeout(*HTTP_TIME_OUT).send().await {
        if res.status() == StatusCode::OK {
            let data = res.text().await?;
            let html = htmler::Html::parse_fragment(&data);

            if let Some(pics) = html.select(&WALL_PICS_SELECTOR).next() {
                let img_list = pics.select(&WALL_PICS_VEC_SELECTOR);
                for img in img_list {
                    let a = img.select(&Selector::parse(r#"a"#).unwrap()).next().unwrap();
                    let normal_img_url = a.select(&Selector::parse(r#"img[class="hor"#).unwrap()).next().unwrap().get_attribute("src");
                    let img_name = a.select(&Selector::parse(r#"span"#).unwrap()).next().unwrap().inner_html();
                    wall_vec.push(WallModal {
                        name: img_name,
                        second_url: format!("{}{}", *WALL_URL, a.get_attribute("href")),
                        normal_img_url: format!("{}{}", *WALL_URL, normal_img_url),
                    })
                }
            }
        }
    }

    for x in wall_vec {
        let modal = x.clone();
        if let Ok(page_result) = client.get(x.second_url).timeout(*HTTP_TIME_OUT).send().await {
            let dome = page_result.text().await?;
            let html = htmler::Html::parse_fragment(&dome);
            if let Some(img_list_div) = html.select(&Selector::parse(r#"div[class="block-download__resolutions--6"]"#).unwrap()).next() {
                let uhd_url = img_list_div.select(&Selector::parse(r#"a"#).unwrap()).next().unwrap().get_attribute("href");
                let bing = NewBing {
                    name: x.name,
                    url: x.normal_img_url.clone(),
                    uhd_url: format!("{}{}", *WALL_URL, uhd_url),
                    uhd_file_path: WALL_UHD_PATH.to_string() + &x.normal_img_url.clone().split("/").last().unwrap(),
                    normal_file_path: WALL_NORMAL_PATH.to_string() + &x.normal_img_url.clone().split("/").last().unwrap(),
                    source: "wallpapers".to_string(),
                    created_date: Local::now().date_naive(),
                };
                new_bing_vec.push(bing);
            } else {
                println!("{}", modal.second_url);
            }
        }
    }

    Ok(new_bing_vec)
}

pub async fn bing_request(page_size: i32) -> anyhow::Result<Vec<NewBing>, anyhow::Error> {
    let client = reqwest::Client::new();

    let mut new_bing_vec: Vec<NewBing> = vec![];

    create_dir(BING_UHD_PATH.clone());
    create_dir(BING_NORMAL_PATH.clone());

    if let Ok(res) = client.get(format!("https://peapix.com/bing/cn?page={}", &page_size.to_string())).timeout(*HTTP_TIME_OUT).send().await {
        if res.status() == StatusCode::OK {
            let data = res.text().await?;
            let html = htmler::Html::parse_fragment(&data);

            let x = html.select(&BING_IMG_LIST_SELECTOR);
            for img_list in x {
                if let Some(node) = img_list.clone().select(&BING_IMG_ROW_SELECTOR).next() {
                    let img_url = node.select(&BING_PIC_SELECTOR).next().with_context(|| format!("read img url err {page_size}"))?.get_attribute("data-bgset").replace("480.jpg", "240.jpg");
                    let desc = node.select(&BING_DESC_SELECTOR).next().with_context(|| format!("read name error {page_size}"))?.get_attribute("title");
                    let date = node.select(&BING_DATE_SELECTOR).next().with_context(|| format!("read date error err {page_size}"))?.inner_html();
                    let image_name = img_url.split("/").last().unwrap();
                    let uhd_file_path = format!("{}{}", *BING_UHD_PATH, image_name);
                    let normal_file_path = format!("{}{}", *BING_NORMAL_PATH, image_name);
                    new_bing_vec.push(NewBing {
                        name: desc.to_string(),
                        url: img_url.clone(),
                        uhd_url: img_url.replace("240.jpg", "2560.jpg"),
                        uhd_file_path,
                        source: "bing".to_string(),
                        normal_file_path: normal_file_path.clone(),
                        created_date: NaiveDate::parse_from_str(&date, "%B %d, %Y").unwrap(),
                    });
                }
            }
        }
    }

    Ok(new_bing_vec)
}

fn create_dir(path: String) {
    if Path::exists(path.as_ref()) == false {
        fs::create_dir_all(&path).expect("TODO: panic message");
    }
}


#[cfg(test)]
mod tests {
    use super::{bing_request, bing_total_page, spotlight_total_page, wallpapers_total_page, wallpapers_request};
    use super::spotlight_request;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::time::Duration;
    use std::path::Path;
    use std::fs::File;
    use futures_util::StreamExt;
    use std::io::Write;

    #[actix_rt::test]
    async fn bing_request_test() {
        let bing_vec = bing_request(1).await;
        println!("{:#?}", bing_vec);
        assert!(bing_vec.unwrap().len() > 0)
    }

    #[actix_rt::test]
    async fn window_request_test() {
        let count = Arc::new(Mutex::new(0));
        for i in 1..501 {
            let my_count = Arc::clone(&count);
            tokio::spawn(async move {
                println!("refresh start {i}");
                let spotlight_vec = spotlight_request(i).await.unwrap();
                println!("spotlight_vec_len={}", spotlight_vec.len());

                for x in &spotlight_vec {
                    if Path::new(&x.normal_file_path.clone()).exists() == false {
                        let client = reqwest::Client::new();
                        if let Ok(res) = client.get(&x.url).send().await {
                            let content_length = res.content_length().unwrap() as f64;
                            let mut file = File::create(&x.normal_file_path).unwrap();
                            let mut stream = res.bytes_stream();
                            let mut download_size: u64 = 0;
                            while let Some(item) = stream.next().await {
                                let bytes: &[u8] = &item.unwrap();
                                let size = bytes.len() as u64;
                                download_size += size;
                                let download_process = download_size as f64 / content_length;
                                println!("download_process = {}", download_process);
                                file.write_all(&bytes).unwrap();
                            }
                        }
                    }
                }
                let mut lock = my_count.lock().await;
                println!("refresh end {i}");
                *lock += 1;
            });
        }

        loop {
            if *count.lock().await >= 500 {
                println!("refresh end all");
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    #[actix_rt::test]
    async fn spotlight_total_page_test() {
        let total_page = spotlight_total_page().await;
        println!("{:#?}", total_page);
    }

    #[actix_rt::test]
    async fn bing_total_page_test() {
        let total_page = bing_total_page().await;
        println!("{:#?}", total_page);
    }


    #[actix_rt::test]
    async fn wallpapers_total_page_test() {
        let total_page = wallpapers_total_page().await;
        println!("{:#?}", total_page);
    }

    #[actix_rt::test]
    async fn wallpapers_request_test() {
        let total_page = wallpapers_request(1).await;
        println!("{:#?}", total_page);
    }
}
