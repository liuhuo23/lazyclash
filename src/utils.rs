use color_eyre::{eyre::Ok, Result};
use config::Config;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
pub mod help;

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

/// 根据给定的url下载对应的订阅配置文件
pub async fn download_yaml(url: &str, config: &Config) -> Result<()> {
    let res = reqwest::get(url).await?.json::<Value>().await?;
    // 这里假设我们获取的 JSON 数据是一个对象，我们将其保存为字符串
    let json_string = serde_json::to_string_pretty(&res)?;
    // 将 JSON 字符串写入文件
    let mut file = File::create("data.json")?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;
    use reqwest::header::HeaderValue;
    use reqwest::StatusCode;
    use serde_json::Value;

    #[tokio::test]
    async fn download() -> Result<()> {
        // let url = "https://sub.cloudlion.me/api/v1/client/subscribe?token=6a5a4667da647891b46dc4748422b94c";
        let url =
            "https://d.bbydy.org/api/v1/client/subscribe?token=bccc05030e3d153e369fd0a49d359ed2";
        let mut builder = reqwest::ClientBuilder::new().use_rustls_tls().no_proxy();
        builder = builder.danger_accept_invalid_certs(false);
        builder = builder.user_agent("clash-verge/unknown");
        let resp = builder.build()?.get(url).send().await?;
        let status_code = resp.status();
        if !StatusCode::is_success(&status_code) {
            eprintln!("failt {status_code}");
        }
        let header = resp.headers();
        println!("headers: {:?}", header);
        // let bytes = res.unwrap().as_bytes();
        // let str = String::from_utf8(bytes.to_vec()).unwrap();
        // let value: Value = serde_json::from_str(&str).unwrap();
        // println!("value: {}", value);
        // let endpoints = value.as_object().unwrap().get("endpoints").unwrap();
        // for endpoint in endpoints.as_array().unwrap() {
        //     println!("{}", endpoint.as_object().unwrap().get("url").unwrap());
        // }

        // let res = res.to_vec();
        // let res = String::from_utf8(res).unwrap();
        // eprint!("res: {res}");
        // 这里假设我们获取的 JSON 数据是一个对象，我们将其保存为字符串
        // let json_string = serde_json::to_string_pretty(&res).unwrap();
        // println!("{json_string}");
        Ok(())
    }
}
