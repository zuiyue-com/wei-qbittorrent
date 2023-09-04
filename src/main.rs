use serde_json::json;
use std::path::Path;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei-qbittorrent");
    let args: Vec<String> = std::env::args().collect();

    let mut command = "";

    if args.len() > 1 {
        command = args[1].as_str();
    }

    let main_url = "http://127.0.0.1:10001/";
    let client = reqwest::Client::builder().build()?;

    match command {
        "run" => {
            run().await?;
        }
        "add" => {
            if args.len() < 4 {
                help();
                return Ok(());
            }
        
            let url = args[2].clone();
            let path = args[3].clone();
        
            let form = reqwest::multipart::Form::new()
                .text("urls", url)
                .text("autoTMM", "false")
                .text("savepath", path)
                .text("rename", "")
                .text("category", "")
                .text("paused", "false")
                .text("stopCondition", "None")
                .text("contentLayout", "Original")
                .text("dlLimit", "NaN")
                .text("upLimit", "NaN");
        
            let add_torrent_url = main_url.to_owned() + "api/v2/torrents/add";
            let data = client.post(add_torrent_url).multipart(form).send().await?.text().await?;
            if data.contains("Ok") {
                print!("{}", json!({
                    "code": "200",
                    "msg": "success"
                }).to_string());
            } else {
                print!("{}", json!({
                    "code": "400",
                    "msg": "error"
                }).to_string());
            }
        },
        "get" => {
            if args.len() < 3 {
                help();
                return Ok(());
            }
            let hash = args[2].as_str();
            let url = main_url.to_owned() + "api/v2/torrents/info?hashes=" + hash;
            let data = client.get(url).send().await?.text().await?;
            print!("{}", json!({
                "code": "200",
                "msg": "success",
                "data": data
            }).to_string());
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
            let data = client.post(url).multipart(form).send().await?.text().await?;
            if data.contains("Ok") {
                print!("{}", json!({
                    "code": "200",
                    "msg": "success"
                }).to_string());
            } else {
                print!("{}", json!({
                    "code": "400",
                    "msg": "error"
                }).to_string());
            }
        },
        "list" => {
            // [{"added_on":1693566863,"amount_left":12112389,"auto_tmm":false,"availability":0,"category":"","completed":3964928,"completion_on":-28800,"content_path":"C:\\Users\\Wei\\Desktop\\work\\wei-updater\\test\\data\\new\\0.1.2","dl_limit":0,"dlspeed":0,"download_path":"","downloaded":4187692,"downloaded_session":4187692,"eta":9751,"f_l_piece_prio":false,"force_start":false,"hash":"858d5efde656ff3b75ef5be8204cec505983a4d0","infohash_v1":"858d5efde656ff3b75ef5be8204cec505983a4d0","infohash_v2":"","last_activity":1693566863,"max_ratio":-1,"max_seeding_time":-1,"name":"0.1.2","num_complete":2,"num_incomplete":1,"num_leechs":0,"num_seeds":0,"priority":1,"progress":0.24661627310079162,"ratio":0,"ratio_limit":-2,"save_path":"C:\\Users\\Wei\\Desktop\\work\\wei-updater\\test\\data\\new","seeding_time":0,"seeding_time_limit":-2,"seen_complete":1693617341,"seq_dl":false,"size":16077317,"state":"stalledDL","super_seeding":false,"tags":"","time_active":50548,"total_size":16077317,"tracker":"udp://city21.pk:6969/announce","trackers_count":143,"up_limit":0,"uploaded":0,"uploaded_session":0,"upspeed":0}]
            let mut name = "";
            if args.len() >= 3 {
                name = args[2].as_str();
            }
            let url = main_url.to_owned() + "api/v2/torrents/info";
            let data = client.get(url).send().await?.text().await?;
            let data = data.replace(",\",\"seeding_time\"", ",\"seeding_time\"");
            if name != "" {
                //let regex_string = format!(r#""name":"{}","num_complete":2,"num_incomplete":1,"num_leechs":0,"num_seeds":0,"priority":1,"progress":(?P<progress>\d+(\.\d+)?),"#, name);
                let regex_string = format!(r#""name":"{}"(.*),"progress":(?P<progress>\d+(\.\d+)?),""#, name);
                let re = regex::Regex::new(&regex_string).unwrap();
                
                //let haystack = r#"[{"added_on":1693566863,"amount_left":12112389,"auto_tmm":false,"availability":0,"category":"","completed":3964928,"completion_on":-28800,"content_path":"C:\\Users\\Wei\\Desktop\\work\\wei-updater\\test\\data\\new\\0.1.2","dl_limit":0,"dlspeed":0,"download_path":"","downloaded":4187692,"downloaded_session":4187692,"eta":9751,"f_l_piece_prio":false,"force_start":false,"hash":"858d5efde656ff3b75ef5be8204cec505983a4d0","infohash_v1":"858d5efde656ff3b75ef5be8204cec505983a4d0","infohash_v2":"","last_activity":1693566863,"max_ratio":-1,"max_seeding_time":-1,"name":"0.1.2","num_complete":2,"num_incomplete":1,"num_leechs":0,"num_seeds":0,"priority":1,"progress":0.24661627310079162,"ratio":0,"ratio_limit":-2,"save_path":"C:\\Users\\Wei\\Desktop\\work\\wei-updater\\test\\data\\new","seeding_time":0,"seeding_time_limit":-2,"seen_complete":1693617341,"seq_dl":false,"size":16077317,"state":"stalledDL","super_seeding":false,"tags":"","time_active":50548,"total_size":16077317,"tracker":"udp://city21.pk:6969/announce","trackers_count":143,"up_limit":0,"uploaded":0,"uploaded_session":0,"upspeed":0}]"#;
                let caps = re.captures(data.as_str()).unwrap();

                println!("{}", &caps["progress"]);
                return Ok(());
            }
            println!("{}", data);
        },
        "quit" => {
            let main_url = "http://127.0.0.1:10001/";
            let client = reqwest::Client::builder().build()?;
            let url = main_url.to_owned() + "api/v2/app/shutdown";
            let data = client.post(url).send().await?.text().await?;
            println!("quit: {:?}", data);
        }
        "help" => {
            help();
        }
        _ => {
            run().await?;
        }
    }

    Ok(())
}

fn help() {
    let args: Vec<String> = std::env::args().collect();
    println!("Usage:");
    println!("  {} run", args[0]);
    println!("  {} add <url> <savepath>", args[0]);
    println!("  {} get <hash>", args[0]);
    println!("  {} del <hash>", args[0]);
    println!("  {} list <option:name>", args[0]);
    println!("  {} help", args[0]);
}


// 查询当前系统，win or linux or mac
// 执行./qbittorrent/qbittorrent<取决于系统增加.exe后缀> 失败则运行
// 执行../wei-release/windows/qbittorrent/qbittorrent.exe 失败则下载
// get http://download.zuiyue.com/windows/qbittorrent/qbittorrent.exe
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    use single_instance::SingleInstance;
    let instance = SingleInstance::new("wei-qbittorrent").unwrap();
    if !instance.is_single() { 
        std::process::exit(1);
    };

    let main_url = "http://127.0.0.1:10001/";
    let client = reqwest::Client::builder().build()?;
    let url = main_url.to_owned() + "api/v2/app/shutdown";
    client.post(url).send().await?.text().await?;

    let os = std::env::consts::OS;
    let command = match os {
        "windows" => "./qbittorrent/qbittorrent.exe",
        _ => "./qbittorrent/qbittorrent",
    };

    // 复制./qbittorrent/qbittorrent.ini 到 -> AppData/Roaming/qBittorrent/qBittorrent.ini
    let home = std::env::var("USERPROFILE").unwrap();
    let qbittorrent_ini = format!("{}/AppData/Roaming/qBittorrent/qBittorrent.ini", home);
    let qbittorrent_ini_path = Path::new(&qbittorrent_ini);
    if !qbittorrent_ini_path.exists() {
        let qbittorrent_ini_dir = format!("{}/AppData/Roaming/qBittorrent", home);
        let qbittorrent_ini_dir_path = Path::new(&qbittorrent_ini_dir);
        if !qbittorrent_ini_dir_path.exists() {
            std::fs::create_dir_all(qbittorrent_ini_dir_path).unwrap();
        }
        let source = "./qbittorrent/qBittorrent.ini";
        let source_path = Path::new(source);
        if source_path.exists() {
            std::fs::copy(source_path, qbittorrent_ini_path).unwrap();
        }
    }

    match Command::new(command).output() {
        Ok(output) => {
            println!("output: {:?}", output);
        }
        Err(_) => {
            download("http://download.zuiyue.com/windows/qbittorrent/qbittorrent.exe").await;
            download("http://download.zuiyue.com/windows/qbittorrent/qBittorrent.ini").await;
            download("http://download.zuiyue.com/windows/qbittorrent/Qt6Core.dll").await;
            download("http://download.zuiyue.com/windows/qbittorrent/Qt6Network.dll").await;
            download("http://download.zuiyue.com/windows/qbittorrent/Qt6Sql.dll").await;
            download("http://download.zuiyue.com/windows/qbittorrent/Qt6Xml.dll").await;
        }
    }

    Ok(())
} 

// 检查windows系统是否安装了qbittorrent,并且是否开启了端口10001,如果没有
// 检查系统进程是否存在qbittorrent,如果不存在则运行qbittorrent.检查api端口是否开启成功


// 下载文件，并存放到指定目录
async fn download(url_str: &str) {
    let url = reqwest::Url::parse(url_str).unwrap();

    let dir = "./qbittorrent";
    let file_path = format!("{}/{}", dir, url.path_segments().unwrap().last().unwrap());

    // Create directory if it doesn't exist
    if !Path::new(dir).exists() {
        std::fs::create_dir_all(dir).unwrap();
    }

    let response = reqwest::get(url).await.unwrap();

    let content = response.bytes().await.unwrap();
    std::fs::write(&file_path, content).expect("Unable to write file");
    println!("Download completed!");    
}