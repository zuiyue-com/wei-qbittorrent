use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE, COOKIE};

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

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

    let login_url = "http://localhost:10001/api/v2/auth/login";
    let form = [("username", "admin"), ("password", "adminadmin")];
    let data = client.post(login_url).form(&form).send().await?;
    let cookies = data.headers().get_all(SET_COOKIE).iter()
        .map(|value| value.to_str().unwrap().to_owned())
        .collect::<Vec<String>>().join("; ");
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookies)?);

    match command {
        "add" => {
            if args.len() < 3 {
                help();
                return Ok(());
            }
            let add_torrent_url = "http://localhost:10001/api/v2/torrents/add";

            // let path = args[2].as_str();
            // let path = "C:/Users/Wei/Desktop/work/wei-release/windows/0.1.2.torrent";
            // let file = reqwest::multipart::Part::file(path)?;

            let path = args[2].as_str();
            let file = File::open(path).await?;

            let stream = FramedRead::new(file, BytesCodec::new());
            let data = stream.map(|i| i.unwrap()).collect::<Vec<_>>().await;
            let data = std::io::Bytes::from(data.concat());
        
            let file_part = reqwest::multipart::Part::stream(data)
                .file_name("0.1.2.torrent")
                .mime_str("application/x-bittorrent")?;

            
            let form = reqwest::multipart::Form::new()
                .part("fileselect[]", file_part)
                .text("autoTMM", "false")
                .text("savepath", "C:\\Users\\Wei\\Downloads")
                .text("rename", "")
                .text("category", "")
                .text("paused", "false")
                .text("stopCondition", "None")
                .text("contentLayout", "Original")
                .text("dlLimit", "NaN")
                .text("upLimit", "NaN");

            let data = client.post(add_torrent_url).headers(headers).multipart(form).send().await?.text().await?;
            println!("add: {:?}", data);
        },
        "get" => {
            if args.len() < 3 {
                help();
                return Ok(());
            }
            let url = args[2].as_str();
            // wei_qbittorrent::get(url).await?;
        },
        "del" => {
            if args.len() < 3 {
                help();
                return Ok(());
            }
            let hash = args[2].as_str();
            // wei_qbittorrent::del(hash).await?;
        },
        "list" => {
            // wei_qbittorrent::list().await?;
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
    println!("  {} get <url>", args[0]);
    println!("  {} del <hash>", args[0]);
    println!("  {} list", args[0]);
}

// 检查windows系统是否安装了qbittorrent,并且是否开启了端口10001,如果没有
// fn check() {

// }