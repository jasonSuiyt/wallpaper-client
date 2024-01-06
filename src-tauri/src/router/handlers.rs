use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use crate::dao::models::{Bing, DownloadPayload};
use crate::dao::wallpaper_dao;
use crate::service::get_img_service;
use tokio::time::Duration;

use futures_util::StreamExt;
use tauri::Window;
use crate::service::trans_service::translate;
use std::fs;

#[tauri::command]
pub async fn refresh(window: Window, source: String) {
    let total_page = get_img_service::get_total_page(&source).await;
    println!("total_page={}", total_page);
    let count = Arc::new(Mutex::new(0));
    for i in 1..total_page + 1 {
        let my_count = Arc::clone(&count);
        let source = source.clone();
        tokio::spawn(async move {
            let bing_vec_res;
            if source == "bing" {
                bing_vec_res = get_img_service::bing_request(i).await;
            } else if source == "spotlight" {
                bing_vec_res = get_img_service::spotlight_request(i).await;
            } else {
                bing_vec_res = get_img_service::wallpapers_request(i).await;
            }

            match bing_vec_res {
                Ok(mut bing_vec) => {
                    if source != "bing" {
                        for x in &mut bing_vec {
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
                    for x in &bing_vec {
                        if Path::new(&x.normal_file_path).exists() == false {
                            let client = reqwest::Client::new();
                            if let Ok(res) = client.get(x.to_owned().url).send().await {
                                if let Ok(bytes) = res.bytes().await {
                                    fs::write(&x.normal_file_path, bytes).unwrap();
                                }
                            }
                        }
                    }
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
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"));
        if let Ok(res) = client.get(&wallpaper.uhd_url.clone()).headers(headers).timeout(Duration::from_secs(360)).send().await {
            let content_length = res.content_length().unwrap() as f64;
            let mut file = File::create(&wallpaper.uhd_file_path).unwrap();
            let mut stream = res.bytes_stream();
            let mut download_size: u64 = 0;
            while let Some(item) = stream.next().await {
                let bytes: &[u8] = &item.unwrap();
                let size = bytes.len() as u64;
                download_size += size;
                let download_process = download_size as f64 / content_length;
                let download_process_text = download_process * 100f64;
                window.emit("download_progress", DownloadPayload {
                    id: wallpaper.id.clone(),
                    text: format!("下载中 {:.2} %", download_process_text),
                    process: download_process,
                }).unwrap();
                file.write_all(&bytes).unwrap();
            }
        } else {
            println!("下载壁纸失败");
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

    match run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "System Events" to tell every desktop to set picture to {}"#,
                enquote::enquote('"', &wallpaper.uhd_file_path),
            ),
        ],
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }


    tokio::time::sleep(Duration::from_millis(1000)).await;
    window.emit("download_progress", DownloadPayload {
        id: wallpaper.id.clone(),
        process: 0f64,
        text: "".to_string(),
    }).unwrap();
    true
}


type Res<T> = Result<T, Box<dyn Error>>;


#[cfg(unix)]
fn run(command: &str, args: &[&str]) -> Res<String> {
    use std::process::Command;

    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().into())
    } else {
        Err(format!(
            "{} exited with status code {}",
            command,
            output.status.code().unwrap_or(-1),
        )
            .into())
    }
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