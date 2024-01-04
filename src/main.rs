use std::{env, time::Duration, path::{PathBuf, Path}, fs::{self, File}, io::Write};

use anyhow::Ok;
use log::LevelFilter;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, json};

const POSTMAN_BASE_URL: &str = "https://api.getpostman.com";

fn get_client(api_key: &str) -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0"));
    headers.insert("X-API-Key", HeaderValue::from_str(api_key).unwrap());
    reqwest::Client::builder().default_headers(headers).timeout(Duration::from_secs(10)).build().unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter(Some("postman_backup"), LevelFilter::Debug)
        .init();

    let api_key = match env::var_os("POSTMAN_API_KEY") {
        None => panic!("Missing ENV parameter 'POSTMAN_API_KEY'"),
        Some(x) => x.to_str().unwrap().to_string(),
    };

    log::info!("获取用户信息...");
    let me_body: Value = get_client(&api_key).get(format!("{POSTMAN_BASE_URL}/me")).send().await?.json().await?;
    write_file(PathBuf::new().join("backup/user_info.json").as_path(), me_body.to_string())?;
    log::info!("Hello {}", me_body["user"]["fullName"].as_str().unwrap());

    log::info!("获取所有工作区...");
    let workspaces_body: Value = get_client(&api_key).get(format!("{POSTMAN_BASE_URL}/workspaces")).send().await?.json().await?;
    for workspace in workspaces_body["workspaces"].as_array().unwrap() {
        log::info!("备份工作区 {}", workspace["name"].as_str().unwrap());
        let workspace_id = workspace["id"].as_str().unwrap();
        write_file(PathBuf::new().join(format!("backup/workspace/{workspace_id}/workspace_info.json")).as_path(), workspace.to_string())?;
        
        let mut archive: Value = json!({"environment": {}, "collection": {}});

        log::info!("获取 {} 工作区内所有收集...", workspace["name"].as_str().unwrap());
        let collections_body: Value = get_client(&api_key).get(format!("{POSTMAN_BASE_URL}/collections?workspace={workspace_id}")).send().await?.json().await?;
        for collection in collections_body["collections"].as_array().unwrap() {
            log::info!("备份 {} 工作区内 {} 收集...", workspace["name"].as_str().unwrap(), collection["name"].as_str().unwrap());
            let collection_id = collection["id"].as_str().unwrap();
            archive["collection"][collection_id] = Value::Bool(true);
            let collection_body: Value = get_client(&api_key).get(format!("{POSTMAN_BASE_URL}/collections/{collection_id}")).send().await?.json().await?;
            write_file(PathBuf::new().join(format!("backup/workspace/{workspace_id}/collection/{}.json", collection_id)).as_path(), collection_body["collection"].to_string())?;
        }

        log::info!("获取 {} 工作区内所有环境...", workspace["name"].as_str().unwrap());
        let environments_body: Value = get_client(&api_key).get(format!("{POSTMAN_BASE_URL}/environments?workspace={workspace_id}")).send().await?.json().await?;
        for environment in environments_body["environments"].as_array().unwrap() {
            log::info!("备份 {} 工作区内 {} 环境...", workspace["name"].as_str().unwrap(), environment["name"].as_str().unwrap());
            let environment_id = environment["id"].as_str().unwrap();
            archive["environment"][environment_id] = Value::Bool(true);
            let environment_body: Value = get_client(&api_key).get(format!("{POSTMAN_BASE_URL}/environments/{environment_id}")).send().await?.json().await?;
            write_file(PathBuf::new().join(format!("backup/workspace/{workspace_id}/environment/{}.json", environment_id)).as_path(), environment_body["environment"].to_string())?;
        }

        write_file(PathBuf::new().join(format!("backup/workspace/{workspace_id}/archive.json")).as_path(), archive.to_string())?;
    }
    log::info!("备份完成");

    Ok(())
}

pub fn write_file(path: &Path, str: String)-> anyhow::Result<()> {
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(&parent).unwrap_or_else(|e| {
            panic!("Could not create target directory: {}, {:?}", parent.display(), e)
        });
    }
    let mut file: File = File::create(path)?;
    file.write(str.as_bytes())?;
    file.flush()?;
    Ok(())
}
