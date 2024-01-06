
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tiberius::{AuthMethod, Config};
use tokio::{
    net::TcpStream,
    time::{sleep, Duration},
};
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[derive(Debug, Clone, Default)]
pub struct InitData {
    pub librarie_id: String,
    pub printers: Vec<String>,
    pub btws: Vec<String>,
}
pub async fn init_all() -> InitData {
    // sleep(Duration::from_millis(1500)).await;
    let first = get_libraries().await;
    let mut data = InitData::default();
    match first {
        Ok(id) => {
            data.librarie_id = id.clone();
            let second = load_btws(id).await;
            match second {
                Ok(s) => {
                    data.btws = s;
                    let third = load_printers().await;
                    match third {
                        Ok(b) => {
                            data.printers = b;
                            data
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
pub async fn sql_init(sql: String) -> String {
    let mut config = Config::new();
    config.host(sql);
    config.port(1433);
    config.database("TESTDATA");
    // config.authentication(AuthMethod::sql_server("cytest", "cytest"));
    config.authentication(AuthMethod::sql_server("hztest", "hztest"));
    config.trust_cert();
    let tcp = TcpStream::connect(config.get_addr()).await;
    match tcp {
        Ok(tcp) => {
            tcp.set_nodelay(true).unwrap();
            let client = tiberius::Client::connect(config, tcp.compat_write()).await;
            match client {
                Ok(_) => "成功".to_string(),
                Err(_e) => "".to_string(),
            }
        }
        Err(_e) => "".to_string(),
    }
}
pub async fn get_libraries() -> String{
    let url = "http://localhost/BarTender/api/v1/libraries".to_string();
    let client = reqwest::Client::new();
    let mut hm = HashMap::new();
    let res = client.get(url.clone()).send().await;
    match res {
        Ok(res) => {
            match res.status().is_success() {
                true => {
                    for (key, val) in res.headers().into_iter() {
                        hm.insert(
                            key.as_str().to_owned(),
                            val.to_str().ok().unwrap_or("").to_owned(),
                        );
                    }
                    let req = Req {
                        status: res.status().as_u16(),
                        url: url.clone(),
                        body: res.json().await.unwrap(),
                        headers: hm,
                    };
                    let j = req.body.get(1);
                    match j {
                        Some(j) => {
                            let j = serde_json::to_string(&j["id"]).unwrap();
                            let id: String = j
                                .chars()
                                .map(|x| match x {
                                    '"' => ' ',
                                    '\\' => ' ',
                                    _ => x,
                                })
                                .collect();
                            id
                        }
                        None => "".to_string(),
                    }
                }
                false => "".to_string()
            }
        }
        Err(_e) => "".to_string()
    }
    
}
pub async fn load_printers() -> Vec<String> {
    let url = "http://localhost/BarTender/api/v1/printers".to_string();
    let client = reqwest::Client::new();
    let mut hm = HashMap::new();
    let res = client.get(url.clone()).send().await;
    match res {
        Ok(res) => {
            match res.status().is_success() {
                true => {
                    for (key, val) in res.headers().into_iter() {
                        hm.insert(
                            key.as_str().to_owned(),
                            val.to_str().ok().unwrap_or("").to_owned(),
                        );
                    }
                    let req = Req {
                        status: res.status().as_u16(),
                        url: url.clone(),
                        body: res.json().await.unwrap(),
                        headers: hm,
                    };
                    let server_printers = req.body.get("serverPrinters").unwrap();
                    let x = server_printers.as_array().unwrap();
                    let remote_printers = req.body.get("remotePrinters").unwrap();
                    let y = remote_printers.as_array().unwrap();
                    let mut printerlist = vec![];
                    for i in x {
                        let j = serde_json::to_string(i).unwrap();
                        let s: String = j
                            .chars()
                            .map(|x| match x {
                                '"' => ' ',
                                '\\' => ' ',
                                _ => x,
                            })
                            .collect();
                        let printer = s;
                        printerlist.push(printer);
                    }
                    for i in y {
                        let j = serde_json::to_string(i).unwrap();
                        let s: String = j
                            .chars()
                            .map(|x| match x {
                                '"' => ' ',
                                '\\' => ' ',
                                _ => x,
                            })
                            .collect();
                        let printer =  s ;
                        printerlist.push(printer);
                    }
                    printerlist
                }
                false => vec![],
            }
        },
        Err(_e) => vec![],
    }
    
}
pub async fn load_btws(id: String) -> Vec<String> {
    let url = format!("http://localhost/BarTender/api/v1/libraries/{}", id);
    let mut hm = HashMap::new();
    let res = ureq::get(&url).call();
    match res {
        Ok(res) => {
                    
                    let server_printers = res;
                    let x = server_printers.as_array().unwrap();
                    let mut btwlist = vec![];
                    for i in x {
                        let j = serde_json::to_string(i).unwrap();
                        let s: String = j
                            .chars()
                            .map(|x| match x {
                                '"' => ' ',
                                '\\' => ' ',
                                _ => x,
                            })
                            .collect();
                        btwlist.push(s);
                    }
        
                    btwlist
                
            
        },
        Err(_) => vec![],
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Req {
    pub url: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
}
