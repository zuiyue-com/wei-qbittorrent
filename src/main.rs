use std::env::args;
use url::Url;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE, COOKIE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei-qbittorrent");

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        help();
        return Ok(())
    }

    let command = args[1].as_str();

    let client = reqwest::Client::builder().build()?;
    let main_url = "http://frp004.xlai.cc:8080/";

    let login_url = main_url.to_owned() + "api/v2/auth/login";
    let form = [("username", "admin"), ("password", "adminadmin")];
    let data = client.post(login_url).form(&form).send().await?;
    
    let cookies = data.headers().get_all(SET_COOKIE).iter()
        .map(|value| value.to_str().unwrap().to_owned())
        .collect::<Vec<String>>().join("; ");
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookies)?);

    match command {
        "add" => {
            add().await?;
        },
        "get" => {
            if args.len() < 3 {
                help();
                return Ok(());
            }
            let hash = args[2].as_str();
            let url = main_url.to_owned() + "api/v2/torrents/info?hashes=" + hash;
            let data = client.get(url).headers(headers).send().await?.text().await?;
            println!("get: {:?}", data);
        },
        "del" => {
            if args.len() < 3 {
                help();
                return Ok(());
            }
            let hash = args[2].clone();
            let url = main_url.to_owned() + "api/v2/torrents/delete";
            let form = reqwest::multipart::Form::new()
                .text("hashes", hash)
                .text("deleteFiles", "true");
            let data = client.post(url).headers(headers).multipart(form).send().await?.text().await?;
            println!("del: {:?}", data);
        },
        "list" => {
            let url = main_url.to_owned() + "api/v2/torrents/info";
            let data = client.get(url).headers(headers).send().await?.text().await?;
            println!("list: {:?}", data);
        },
        _ => {
            help();
            return Ok(());
        }
    }

    Ok(())
}

fn help() {
    let args: Vec<String> = std::env::args().collect();
    println!("Usage:");
    println!("  {} add <url>", args[0]);
    println!("  {} get <hash>", args[0]);
    println!("  {} del <hash>", args[0]);
    println!("  {} list", args[0]);
}

async fn add() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        help();
        return Ok(());
    }

    let client = reqwest::Client::builder().build()?;
    let main_url = "http://frp004.xlai.cc:8080/";

    let login_url = main_url.to_owned() + "api/v2/auth/login";
    let form = [("username", "admin"), ("password", "adminadmin")];
    let data = client.post(login_url).form(&form).send().await?;
    
    let cookies = data.headers().get_all(SET_COOKIE).iter()
        .map(|value| value.to_str().unwrap().to_owned())
        .collect::<Vec<String>>().join("; ");
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookies)?);

    let url = args[2].as_str();

    // 获取url的文件名
    let url_filename = Url::parse(url).unwrap();
    let segments = url_filename.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
    let file_name = segments.last().unwrap();

    // 下载文件并保存文件
    let response = reqwest::get(url).await?;
    fs::create_dir_all("./download/").await?;
    let path = format!("./download/{}", file_name.clone());
    let mut out = fs::File::create(path).await?;
    let bytes = response.bytes().await?;
    out.write_all(&bytes).await?;

    // 上传文件到API接口
    let path = format!("./download/{}", file_name.clone());
    let file = std::path::Path::new(&path);
    let bytes = std::fs::read(file).expect("Unable to read file");
    
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name(file_name.clone().to_owned())
        .mime_str("application/x-bittorrent")?;

    let form = reqwest::multipart::Form::new()
        .part("fileselect[]", part)
        .text("autoTMM", "false")
        .text("savepath", "/home/")
        .text("rename", "")
        .text("category", "")
        .text("paused", "false")
        .text("stopCondition", "None")
        .text("contentLayout", "Original")
        .text("dlLimit", "NaN")
        .text("upLimit", "NaN");

    let add_torrent_url = main_url.to_owned() + "api/v2/torrents/add";
    let data = client.post(add_torrent_url).headers(headers).multipart(form).send().await?.text().await?;
    println!("add: {:?}", data);
    Ok(())
}


// 查询当前系统，win or linux or mac
// 执行./qbittorrent/qbittorrent<取决于系统增加.exe后缀> 失败则运行
// 执行../wei-release/windows/qbittorrent/qbittorrent.exe 失败则下载
// get https://download.zuiyue.com/windows/qbittorrent/qbittorrent.exe

// 检查windows系统是否安装了qbittorrent,并且是否开启了端口10001,如果没有
// 检查系统进程是否存在qbittorrent,如果不存在则运行qbittorrent.检查api端口是否开启成功
