use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Context, Error};
use chrono::{Local, NaiveDate};
use htmler::Selector;
use lazy_static::lazy_static;
use reqwest::StatusCode;

use crate::dao::models::NewBing;

fn get_home() -> String {
    dirs::home_dir().unwrap().to_str().unwrap().to_string()
}

lazy_static! {
    static ref SPOTLIGHT_UHD_PATH: String =
        format!("{}{}", get_home(), "/wallpaper/spotlight/images/uhd/");
    static ref SPOTLIGHT_NORMAL_PATH: String =
        format!("{}{}", get_home(), "/wallpaper/spotlight/images/normal/");
    static ref SPOTLIGHT_ARTICLE_SELECTOR: Selector = Selector::parse(r#"article"#).unwrap();
    static ref SPOTLIGHT_DATE_SELECTOR: Selector =
        Selector::parse(r#"span[class="date"]"#).unwrap();
    static ref SPOTLIGHT_NAME_SELECTOR: Selector =
        Selector::parse(r#"span[class="entry-title hidden"]"#).unwrap();
    static ref SPOTLIGHT_IMG_SELECTOR: Selector = Selector::parse(r#"img"#).unwrap();
    static ref SPOTLIGHT_PAGE_NUMBERS_SELECTOR: Selector =
        Selector::parse(r#"a[class="page-numbers"]"#).unwrap();
    static ref BING_UHD_PATH: String = format!("{}{}", get_home(), "/wallpaper/bing/images/uhd/");
    static ref BING_NORMAL_PATH: String =
        format!("{}{}", get_home(), "/wallpaper/bing/images/normal/");
    static ref BING_IMG_LIST_SELECTOR: Selector =
        Selector::parse(r#"div[class="col-md-6 col-lg-4"]"#).unwrap();
    static ref BING_IMG_ROW_SELECTOR: Selector =
        Selector::parse(r#"div[class="image-list__container"]"#).unwrap();
    static ref BING_PIC_SELECTOR: Selector =
        Selector::parse(r#"div[class="image-list__picture lazyload"]"#).unwrap();
    static ref BING_DESC_SELECTOR: Selector =
        Selector::parse(r#"a[class="image-list__link"]"#).unwrap();
    static ref BING_DATE_SELECTOR: Selector =
        Selector::parse(r#"span[class="text-gray"]"#).unwrap();
    static ref BING_PAGE_NUMBERS_SELECTOR: Selector =
        Selector::parse(r#"a[class="page-link"]"#).unwrap();
    static ref ANIME_UHD_PATH: String = format!("{}{}", get_home(), "/wallpaper/anime/images/uhd/");
    static ref ANIME_NORMAL_PATH: String =
        format!("{}{}", get_home(), "/wallpaper/anime/images/normal/");
    static ref NAIME_PAGE_NUMBERS_SELECTOR: Selector = Selector::parse(r#"h2"#).unwrap();
    static ref NAIME_LI_SELECTOR: Selector = Selector::parse(r#"li"#).unwrap();
    static ref ANIME_NAME_SELECTOR: Selector =
        Selector::parse(r#"span[class="wall-res"]"#).unwrap();
    static ref ANIME_SECTION_SELECTOR: Selector =
        Selector::parse(r#"section[class="thumb-listing-page"]"#).unwrap();
    static ref ANIME_IMG_SELECTOR: Selector = Selector::parse(r#"img[class="lazyload"]"#).unwrap();
    static ref WALL_UHD_PATH: String = format!("{}{}", get_home(), "/wallpaper/wall/images/uhd/");
    static ref WALL_NORMAL_PATH: String =
        format!("{}{}", get_home(), "/wallpaper/wall/images/normal/");
    static ref WALL_PAGE_NUMBERS_SELECTOR: Selector =
        Selector::parse(r#"p[class="pages"]"#).unwrap();
    static ref WALL_PICS_SELECTOR: Selector = Selector::parse(r#"div[class="pics"]"#).unwrap();
    static ref WALL_PICS_VEC_SELECTOR: Selector = Selector::parse(r#"p"#).unwrap();
    static ref WALL_URL: String = "https://wallpapershome.com".to_string();
    static ref HTTP_TIME_OUT: Duration = Duration::from_secs(360);
}

pub struct BingImage(String);

pub struct WallpaperImage(String);

pub struct AnimeImage(String);

pub struct SpotlightImage(String);

#[async_trait::async_trait]
pub(crate) trait ImageTrait {
    async fn get_totals(&self) -> anyhow::Result<i32, Error>;
    async fn get_page(&self, page: i32) -> anyhow::Result<Vec<NewBing>, Error>;
}

#[async_trait::async_trait]
impl ImageTrait for BingImage {
    async fn get_totals(&self) -> anyhow::Result<i32, Error> {
        let client = reqwest::Client::new();
        match client
            .get("https://peapix.com/bing/cn")
            .timeout(*HTTP_TIME_OUT)
            .send()
            .await
        {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    let html_dom = res.text().await.unwrap();
                    let html = htmler::Html::parse_fragment(&html_dom);
                    if let Some(node) = html.select(&BING_PAGE_NUMBERS_SELECTOR).last() {
                        return Ok(node.inner_html().parse::<i32>().unwrap());
                    }
                }
            }
            Err(_) => {}
        }
        return Err(anyhow!("网络连接异常"));
    }

    async fn get_page(&self, page_size: i32) -> anyhow::Result<Vec<NewBing>, Error> {
        let client = reqwest::Client::new();
        let mut new_bing_vec: Vec<NewBing> = vec![];
        create_dir(BING_UHD_PATH.clone());
        create_dir(BING_NORMAL_PATH.clone());

        if let Ok(res) = client
            .get(format!("https://peapix.com/bing/cn?page={}", &page_size))
            .timeout(*HTTP_TIME_OUT)
            .send()
            .await
        {
            if res.status() == StatusCode::OK {
                let data = res.text().await?;
                let html = htmler::Html::parse_fragment(&data);

                let x = html.select(&BING_IMG_LIST_SELECTOR);
                for img_list in x {
                    if let Some(node) = img_list.clone().select(&BING_IMG_ROW_SELECTOR).next() {
                        let img_url = node
                            .select(&BING_PIC_SELECTOR)
                            .next()
                            .with_context(|| format!("read img url err {page_size}"))?
                            .get_attribute("data-bgset")
                            .replace("480.jpg", "240.jpg");
                        let desc = node
                            .select(&BING_DESC_SELECTOR)
                            .next()
                            .with_context(|| format!("read name error {page_size}"))?
                            .get_attribute("title");
                        let date = node
                            .select(&BING_DATE_SELECTOR)
                            .next()
                            .with_context(|| format!("read date error err {page_size}"))?
                            .inner_html();
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
}

#[async_trait::async_trait]
impl ImageTrait for WallpaperImage {
    async fn get_totals(&self) -> anyhow::Result<i32, Error> {
        let client = reqwest::Client::new();
        if let Ok(res) = client
            .get("https://wallpapershome.com/girls/?page=1")
            .timeout(*HTTP_TIME_OUT)
            .send()
            .await
        {
            if res.status() == StatusCode::OK {
                let html_dom = res.text().await.unwrap();
                let html = htmler::Html::parse_fragment(&html_dom);
                let pages = html.select(&WALL_PAGE_NUMBERS_SELECTOR).next().unwrap();
                if let Some(node) = pages
                    .select(&Selector::parse(r#"a"#).unwrap())
                    .filter(|x| x.get_attribute("class") == "")
                    .last()
                {
                    return Ok(node.inner_html().parse::<i32>().unwrap());
                }
            }
        }
        return Ok(1);
    }

    async fn get_page(&self, page: i32) -> anyhow::Result<Vec<NewBing>, Error> {
        let client = reqwest::Client::new();

        let mut new_bing_vec: Vec<NewBing> = vec![];
        let mut wall_vec: Vec<WallModal> = vec![];

        create_dir(WALL_UHD_PATH.to_string());
        create_dir(WALL_NORMAL_PATH.to_string());

        if let Ok(res) = client
            .get(format!("{}/girls/?page={}", WALL_URL.to_string(), &page))
            .timeout(*HTTP_TIME_OUT)
            .send()
            .await
        {
            if res.status() == StatusCode::OK {
                let data = res.text().await?;
                let html = htmler::Html::parse_fragment(&data);

                if let Some(pics) = html.select(&WALL_PICS_SELECTOR).next() {
                    let img_list = pics.select(&WALL_PICS_VEC_SELECTOR);
                    for img in img_list {
                        let a = img
                            .select(&Selector::parse(r#"a"#).unwrap())
                            .next()
                            .unwrap();
                        let normal_img_url = a
                            .select(&Selector::parse(r#"img[class="hor"#).unwrap())
                            .next()
                            .unwrap()
                            .get_attribute("src");
                        let img_name = a
                            .select(&Selector::parse(r#"span"#).unwrap())
                            .next()
                            .unwrap()
                            .inner_html();
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
            if let Ok(page_result) = client
                .get(x.second_url)
                .timeout(*HTTP_TIME_OUT)
                .send()
                .await
            {
                let dome = page_result.text().await?;
                let html = htmler::Html::parse_fragment(&dome);
                if let Some(img_list_div) = html
                    .select(
                        &Selector::parse(r#"div[class="block-download__resolutions--6"]"#).unwrap(),
                    )
                    .next()
                {
                    let uhd_url = img_list_div
                        .select(&Selector::parse(r#"a"#).unwrap())
                        .next()
                        .unwrap()
                        .get_attribute("href");
                    let bing = NewBing {
                        name: x.name,
                        url: x.normal_img_url.clone(),
                        uhd_url: format!("{}{}", *WALL_URL, uhd_url),
                        uhd_file_path: WALL_UHD_PATH.to_string()
                            + &x.normal_img_url.clone().split("/").last().unwrap(),
                        normal_file_path: WALL_NORMAL_PATH.to_string()
                            + &x.normal_img_url.clone().split("/").last().unwrap(),
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
}
#[async_trait::async_trait]
impl ImageTrait for AnimeImage {
    async fn get_totals(&self) -> anyhow::Result<i32, Error> {
        let client = reqwest::Client::new();
        if let Ok(res) = client
            .get("https://wallhaven.cc/search?categories=010&purity=010&resolutions=2560x1080%2C3440x1440%2C3840x1600%2C1920x1080%2C2560x1440%2C3840x2160&sorting=favorites&order=desc&ai_art_filter=1&page=2")
            .timeout(*HTTP_TIME_OUT)
            .send().await {
            if res.status() == StatusCode::OK {
                let html_dom = res.text().await.unwrap();
                let html = htmler::Html::parse_fragment(&html_dom);
                if let Some(total_page) = html.select(&NAIME_PAGE_NUMBERS_SELECTOR).map(|x| x.inner_html()).map(|x| {
                    if x.contains("thumb-listing-page-num") {
                        return x.split("/").last().unwrap().trim().parse::<i32>().unwrap();
                    }
                    return 0;
                }).max() {
                    return Ok(total_page);
                }
            }
        }
        return Ok(1);
    }

    async fn get_page(&self, page_size: i32) -> anyhow::Result<Vec<NewBing>, Error> {
        let client = reqwest::Client::new();
        let mut new_bing_vec: Vec<NewBing> = vec![];

        create_dir(ANIME_NORMAL_PATH.clone());
        create_dir(ANIME_UHD_PATH.clone());

        fn get_anime_hd_url(normal_url: String, name_ext: String) -> String {
            let string = normal_url.replace("th.wallhaven.cc/small", "w.wallhaven.cc/full");
            let name = string.split("/").last().unwrap();
            return if name_ext.is_empty() {
                let url = &string[0..string.len() - name.len()];
                let name = format!("wallhaven-{}", name);
                format!("{}{}", url, name)
            } else {
                let url = &string[0..string.len() - name.len()];
                let name = name.split(".").next().unwrap();
                let name = format!("wallhaven-{}.{}", name, name_ext.to_lowercase());
                format!("{}{}", url, name)
            };
        }

        if let Ok(res) = client.get(format!("https://wallhaven.cc/search?categories=010&purity=010&resolutions=2560x1080%2C3440x1440%2C3840x1600%2C1920x1080%2C2560x1440%2C3840x2160&sorting=favorites&order=desc&ai_art_filter=1&page={}", &page_size)).timeout(*HTTP_TIME_OUT).send().await {
            if res.status() == StatusCode::OK {
                let html_dom = res.text().await.unwrap();
                let html = htmler::Html::parse_fragment(&html_dom);
                let node = html.select(&ANIME_SECTION_SELECTOR).next().unwrap();
                let li = node.select(&NAIME_LI_SELECTOR);
                for article in li {
                    let name = article.select(&ANIME_NAME_SELECTOR.clone()).next().with_context(|| format!("read name url err {page_size}"))?.inner_html();
                    if let None = article.select(&SPOTLIGHT_IMG_SELECTOR).next() {
                        println!("{}", article.inner_html());
                    }
                    if let Some(img_node) = article.select(&ANIME_IMG_SELECTOR).next() {
                        let png = article.select(&Selector::parse(r#"span[class="png"]"#).unwrap()).next();
                        let mut name_ext = "".to_string();
                        if png.is_some() {
                            name_ext = png.unwrap().text().next().unwrap().to_string();
                        }
                        let normal_img_url = img_node.get_attribute("data-src");
                        let img_name = normal_img_url.split("/").last().unwrap();
                        let uhd_img_url = get_anime_hd_url(normal_img_url.to_string(), name_ext);
                        let uhd_file_path = format!("{}{}", ANIME_UHD_PATH.clone(), img_name);
                        let normal_file_path = format!("{}{}", ANIME_NORMAL_PATH.clone(), img_name);
                        new_bing_vec.push(NewBing {
                            name,
                            url: normal_img_url.to_string(),
                            uhd_url: uhd_img_url.to_string(),
                            uhd_file_path,
                            source: "anime".to_string(),
                            normal_file_path: normal_file_path.clone(),
                            created_date: Local::now().date_naive(),
                        });
                    }
                }
            }
        }

        Ok(new_bing_vec)
    }
}

#[async_trait::async_trait]
impl ImageTrait for SpotlightImage {
    async fn get_totals(&self) -> anyhow::Result<i32, Error> {
        let client = reqwest::Client::new();
        if let Ok(res) = client
            .get("https://windows10spotlight.com/page/1")
            .timeout(*HTTP_TIME_OUT)
            .send()
            .await
        {
            if res.status() == StatusCode::OK {
                let html_dom = res.text().await.unwrap();
                let html = htmler::Html::parse_fragment(&html_dom);
                if let Some(total_page) = html
                    .select(&SPOTLIGHT_PAGE_NUMBERS_SELECTOR)
                    .map(|x| x.inner_html())
                    .map(|x| x.replace(",", "").parse::<i32>().unwrap())
                    .max()
                {
                    return Ok(total_page);
                }
            }
        }
        Ok(1)
    }

    async fn get_page(&self, page_size: i32) -> anyhow::Result<Vec<NewBing>, Error> {
        let client = reqwest::Client::new();
        let mut new_bing_vec: Vec<NewBing> = vec![];

        create_dir(SPOTLIGHT_UHD_PATH.clone());
        create_dir(SPOTLIGHT_NORMAL_PATH.clone());

        if let Ok(res) = client
            .get(format!(
                "https://windows10spotlight.com/page/{}",
                &page_size
            ))
            .timeout(*HTTP_TIME_OUT)
            .send()
            .await
        {
            if res.status() == StatusCode::OK {
                let html_dom = res.text().await.unwrap();
                let html = htmler::Html::parse_fragment(&html_dom);
                for article in html.select(&SPOTLIGHT_ARTICLE_SELECTOR) {
                    let name = article
                        .select(&SPOTLIGHT_NAME_SELECTOR)
                        .next()
                        .with_context(|| format!("read name url err {page_size}"))?
                        .inner_html();

                    let date = article
                        .select(&SPOTLIGHT_DATE_SELECTOR)
                        .next()
                        .with_context(|| format!("read date url err {page_size}"))?
                        .inner_html();
                    if let None = article.select(&SPOTLIGHT_IMG_SELECTOR).next() {
                        println!("{}", article.inner_html());
                    }
                    if let Some(img_node) = article.select(&SPOTLIGHT_IMG_SELECTOR).next() {
                        let img = img_node
                            .get_attribute("srcset")
                            .split(",")
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>();
                        let normal_img_url = img.get(1).unwrap().split_whitespace().next().unwrap();
                        let uhd_img_url = img.get(2).unwrap().split_whitespace().next().unwrap();
                        let img_name = uhd_img_url.split("/").last().unwrap();

                        let uhd_file_path = format!("{}{}", SPOTLIGHT_UHD_PATH.clone(), img_name);
                        let normal_file_path =
                            format!("{}{}", SPOTLIGHT_NORMAL_PATH.clone(), img_name);

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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ImageSource {
    BING,
    SPOTLIGHT,
    WALLPAPERS,
    ANIME,
}

impl Display for ImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            ImageSource::BING => "bing".to_string(),
            ImageSource::SPOTLIGHT => "spotlight".to_string(),
            ImageSource::WALLPAPERS => "wallpapers".to_string(),
            ImageSource::ANIME => "anime".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for ImageSource {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() {
            "BING" => Ok(ImageSource::BING),
            "SPOTLIGHT" => Ok(ImageSource::SPOTLIGHT),
            "WALLPAPERS" => Ok(ImageSource::WALLPAPERS),
            "ANIME" => Ok(ImageSource::ANIME),
            _ => Err(()),
        }
    }
}

pub(crate) struct GetImageFactory;

impl GetImageFactory {
    pub(crate) fn new(image_source: ImageSource) -> Box<dyn ImageTrait + Send> {
        match image_source {
            ImageSource::BING => Box::new(BingImage("BING".to_string())),
            ImageSource::SPOTLIGHT => Box::new(WallpaperImage("SPOTLIGHT".to_string())),
            ImageSource::WALLPAPERS => Box::new(SpotlightImage("WALLPAPERS".to_string())),
            ImageSource::ANIME => Box::new(AnimeImage("ANIME".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::get_img_service::{GetImageFactory, ImageSource};

    #[actix_rt::test]
    async fn totals_test() {
        let factory = GetImageFactory::new(ImageSource::BING);
        let x = factory.get_totals().await.unwrap();
        println!("totals: {:?}", x);
    }
}

#[derive(Debug, Clone)]
struct WallModal {
    pub name: String,
    pub second_url: String,
    pub normal_img_url: String,
}

fn create_dir(path: String) {
    if Path::exists(path.as_ref()) == false {
        match fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
