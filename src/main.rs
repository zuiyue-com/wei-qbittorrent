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
            let url = args[2].as_str();
            let add_torrent_url = "http://localhost:10001/api/v2/torrents/add";
            let form = [
                ("urls", url),
                ("autoTMM", "false"),
                ("savepath", "C:\\Users\\Wei\\Downloads"),
                ("cookie", ""),
                ("rename", ""),
                ("category", ""),
                ("paused", "false"),
                ("stopCondition", "None"),
                ("contentLayout", "Original"),
                ("dlLimit", "NaN"),
                ("upLimit", "NaN")
            ];

            let data = client.post(add_torrent_url).headers(headers).form(&form).send().await?.text().await?;
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