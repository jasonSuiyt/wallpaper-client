use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
use crate::dao::models::{Bing, DownloadPayload, NewBing};
use crate::dao::wallpaper_dao;
use crate::service::get_img_service;
use tokio::time::Duration;

use futures_util::{AsyncReadExt, StreamExt};
use tauri::Window;
use crate::service::trans_service::translate;
use std::fs;


#[tauri::command]
pub async fn refresh(window: Window, source: String) {
    let total_page = get_img_service::get_total_page(&source).await;
    println!("total_page={}", total_page);
    let count = Arc::new(Mutex::new(0));
    let page = wallpaper_dao::find_all(1, &source).unwrap();
    if page.data.len() > 0 {
        refresh_sync(&window, &source, total_page).await;
    }else {
        refresh_async(window, source, total_page, &count).await;
    }
}

async fn refresh_async(window: Window, source: String, total_page: i32, count: &Arc<Mutex<i32>>) {
    for i in 1..total_page + 1 {
        let my_count = Arc::clone(&count);
        let source = source.clone();

        tokio::spawn(async move {
            let bing_vec_res = get_image_vec(i, &source).await;
            match bing_vec_res {
                Ok(mut bing_vec) => {
                    translate_title(&source, &mut bing_vec).await;
                    save_normal_img(&mut bing_vec).await;
                    let mut lock = my_count.lock().await;
                    wallpaper_dao::insert(bing_vec);
                    *lock += 1;
                }
                Err(err) => {
                    let mut lock = my_count.lock().await;
                    println!("异常信息 {:?}", err);
                    *lock += 1;
                }
            }
        });
    }

    loop {
        if *count.lock().await >= total_page {
            println!("refresh end all");
            if source == "bing" {
                window.emit("bing_refresh_finished", true).unwrap();
            } else {
                window.emit("spotlight_refresh_finished", true).unwrap();
            }
            break;
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
        println!("{}", *count.lock().await)
    }
}

async fn get_image_vec(i: i32, source: &String) -> Result<Vec<NewBing>, anyhow::Error> {
    let bing_vec_res;
    if source == "bing" {
        bing_vec_res = get_img_service::bing_request(i).await;
    } else if source == "spotlight" {
        bing_vec_res = get_img_service::spotlight_request(i).await;
    } else if source == "anime" {
        println!("page={}", i);
        bing_vec_res = get_img_service::anime_request(i).await;
    } else {
        bing_vec_res = get_img_service::wallpapers_request(i).await;
    }
    bing_vec_res
}

async fn refresh_sync(window: &Window, source: &String, total_page: i32) {
    for i in 1..total_page + 1 {
        let bing_vec_res = get_image_vec(i, &source).await;
        match bing_vec_res {
            Ok(mut bing_vec) => {
                translate_title(source, &mut bing_vec).await;
                save_normal_img(&mut bing_vec).await;
                let is_new = wallpaper_dao::insert(bing_vec);
                if !is_new {
                    break;
                }
            }
            Err(err) => {
                println!("异常信息 {:?}", err);
            }
        }
    }

    if source == "bing" {
        window.emit("bing_refresh_finished", true).unwrap();
    } else {
        window.emit("spotlight_refresh_finished", true).unwrap();
    }
}

async fn translate_title(source: &str, bing_vec: &mut Vec<NewBing>) {
    if source != "bing" && source != "anime" {
        for x in  bing_vec {
            if Path::new(&x.normal_file_path).exists() == false {
                let result = translate(x.name.clone()).await;
                match result {
                    Ok(result) => {
                        x.name = result;
                    }
                    Err(_) => {}
                }
            }
        }
    }
}

async fn save_normal_img(bing_vec: &mut Vec<NewBing>) {
    for x in bing_vec {
        if Path::new(&x.normal_file_path).exists() == false {
            let client = reqwest::Client::new();
            if let Ok(res) = client.get(x.to_owned().url).send().await {
                if let Ok(bytes) = res.bytes().await {
                    fs::write(&x.normal_file_path, bytes).unwrap();
                }
            }
        }
    }
}


#[tauri::command]
pub fn get_wallpaper(current_page: i64, source: String) -> Vec<Bing> {
    println!("current page {} source {}", current_page, source);
    if let Ok(res) = wallpaper_dao::find_all(current_page, &source) {
        return res.data;
    }
    vec![]
}


#[tauri::command]
pub async fn set_wallpaper(window: Window, wallpaper: Bing) -> bool {
    let wallpaper = Arc::new(wallpaper);

    println!("wallpaper {:#?}", wallpaper);

    if Path::new(&wallpaper.uhd_file_path).exists() == false {
        window.emit("download_progress", DownloadPayload {
            id: wallpaper.id.clone(),
            process: 100f64,
            text: "正在下载壁纸".to_string(),
        }).unwrap();
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));
         match  client.get(&wallpaper.uhd_url.clone()).headers(headers).send().await {
             Ok(res) => {
                 let content_length = res.content_length().unwrap() as f64;
                 let mut stream = res.bytes_stream();
                 let mut download_size: u64 = 0;
                 let mut all_bytes = vec![];
                 while let Some(item) = stream.next().await {
                     let bytes: &[u8] = &item.unwrap();
                     bytes[..bytes.len()].iter().for_each(|x|all_bytes.push(x.clone()));
                     let size = bytes.len() as u64;
                     download_size += size;
                     let download_process = download_size as f64 / content_length;
                     let download_process_text = download_process * 100f64;
                     window.emit("download_progress", DownloadPayload {
                         id: wallpaper.id.clone(),
                         text: format!("下载中 {:.2} %", download_process_text),
                         process: download_process,
                     }).unwrap();
                 }
                 match File::create(&wallpaper.uhd_file_path) {
                     Ok(mut file) => {
                         file.write_all(all_bytes.as_slice()).unwrap();
                     }
                     Err(_) => {}
                 }

             }
             Err(e) => {
                 eprintln!("{}", e);
             }
         }
    }

    if Path::new(&wallpaper.normal_file_path).exists() == false {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"));
        if let Ok(res) = client.get(&wallpaper.url.clone()).send().await {
            let mut file = File::create(&wallpaper.normal_file_path).unwrap();
            let mut stream = res.bytes_stream();
            while let Some(item) = stream.next().await {
                file.write_all(&item.unwrap()).unwrap();
            }
        }
    }

    window.emit("download_progress", DownloadPayload {
        id: wallpaper.id.clone(),
        process: 100f64,
        text: "设置壁纸中".to_string(),
    }).unwrap();

    if Path::new(&&wallpaper.uhd_file_path).exists(){
        match wallpaper::set_from_path(&wallpaper.uhd_file_path) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    window.emit("download_progress", DownloadPayload {
        id: wallpaper.id.clone(),
        process: 0f64,
        text: "".to_string(),
    }).unwrap();
    true
}




#[cfg(test)]
mod tests {
    // use crate::router::handlers::load_all;
    //
    // #[test]
    // fn load_all_test() {
    //     load_all();
    //     println!("11111");
    // }
}