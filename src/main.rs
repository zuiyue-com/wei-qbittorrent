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
            if args.len() < 3 {
                help();
                return Ok(());
            }
            let add_torrent_url = main_url.to_owned() + "api/v2/torrents/add";

            let path = args[2].as_str();
            let file = std::path::Path::new(path);
            let bytes = std::fs::read(file).expect("Unable to read file");
        
            let part = reqwest::multipart::Part::bytes(bytes)
                .file_name("0.1.2.torrent")
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

            let data = client.post(add_torrent_url).headers(headers).multipart(form).send().await?.text().await?;
            println!("add: {:?}", data);
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
    println!("  {} get <url>", args[0]);
    println!("  {} del <hash>", args[0]);
    println!("  {} list", args[0]);
}

// 检查windows系统是否安装了qbittorrent,并且是否开启了端口10001,如果没有
// fn check() {

// }