use chrono::Local;
use serde_json::json;
// use super::init::{InitData, get_libraries, load_btws, load_printers};
use flutter_rust_bridge::spawn;
use tiberius::{numeric::Numeric, AuthMethod, Config};
pub async fn init_all(sql: String) -> InitData {
    // sleep(Duration::from_millis(1500)).await;
    let first = get_libraries().await;
    let mut data = InitData::default();
    data.librarie_id = first.clone();
    let second = load_btws(first).await;
    data.btws = second;
    let third = load_printers().await;
    data.printers = third;
    let sqlstatus = sql_init(sql).await;
    data.sqlstatus = sqlstatus;
    println!("data = {:?}", data);
    data
}

#[derive(Debug, Clone, Default)]
pub struct InitData {
    pub librarie_id: String,
    pub printers: Vec<String>,
    pub btws: Vec<String>,
    pub sqlstatus: bool,
}

pub async fn sql_init(sql: String) -> bool {
    let mut config = Config::new();
    config.host(sql);
    config.port(1433);
    config.database("TESTDATA");
    // config.authentication(AuthMethod::sql_server("cytest", "cytest"));
    config.authentication(AuthMethod::sql_server("hztest", "hztest"));
    config.trust_cert();
    let tcp = tokio::net::TcpStream::connect(config.get_addr()).await;
    match tcp {
        Ok(tcp) => {
            tcp.set_nodelay(true).unwrap();
            let client = tiberius::Client::connect(
                config,
                tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tcp),
            )
            .await;
            match client {
                Ok(_) => true,
                Err(_e) => false,
            }
        }
        Err(_e) => false,
    }
}
pub async fn get_libraries() -> String {
    let url = "http://192.168.2.186/BarTender/api/v1/libraries";
    let res = ureq::get(&url).call().unwrap();
    let value = res.into_json::<serde_json::Value>().unwrap();
    let ids = &value.get(1).unwrap()["id"];
    let s: String = ids
        .to_string()
        .chars()
        .map(|x| match x {
            '"' => ' ',
            '\\' => ' ',
            _ => x,
        })
        .collect();
    let len = s.len();
    let id = &s[1..len - 1];
    id.to_string()
}
pub async fn load_printers() -> Vec<String> {
    let url = "http://192.168.2.186/BarTender/api/v1/printers";
    let res = ureq::get(url).call().unwrap();
    let value = res.into_json::<serde_json::Value>().unwrap();
    let x = value.get("serverPrinters").unwrap().as_array().unwrap();
    let mut list = vec![];
    for i in x {
        let x = i.to_string();
        let s: String = x
            .chars()
            .map(|x| match x {
                '"' => ' ',
                '\\' => ' ',
                _ => x,
            })
            .collect();
        let len = s.len();
        let f = &s[1..len - 1];
        list.push(f.to_string())
    }
    list
}
pub async fn load_btws(id: String) -> Vec<String> {
    let url = format!("http://192.168.2.186/BarTender/api/v1/libraries/{}", id);
    let res = ureq::get(&url).call().unwrap();
    let value = res.into_json::<serde_json::Value>().unwrap();

    let btws = value["contents"].as_array().unwrap();
    let mut list = vec![];
    for i in btws {
        let x = i.to_string();
        let s: String = x
            .chars()
            .map(|x| match x {
                '"' => ' ',
                '\\' => ' ',
                _ => x,
            })
            .collect();
        let len = s.len();
        let f = &s[1..len - 1];
        list.push(f.to_string())
    }
    list
}
pub async fn client(
    sql: String,
) -> tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>> {
    let mut config = Config::new();
    config.host(sql);
    config.port(1433);
    config.database("TESTDATA");
    // config.authentication(AuthMethod::sql_server("cytest", "cytest"));
    config.authentication(AuthMethod::sql_server("hztest", "hztest"));
    config.trust_cert();
    // let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    let tcp = tokio::net::TcpStream::connect(config.get_addr())
        .await
        .unwrap();
    tcp.set_nodelay(true).unwrap();
    let client = tiberius::Client::connect(
        config,
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tcp),
    )
    .await
    .unwrap();
    client
}
pub async fn run_query(sn: String, sql: String) -> Vec<DataInfo> {
    let sn = sn.clone();
    let mut list = vec![];
    let mut client = client(sql.clone()).await;
    let queryvalues =
        "cus_pn, SNTitle, In_name, Inloss1, Reloss1, out_name, Inloss2, Reloss1, print_num";
    let query_ty = format!("where SN = '{}'", sn);
    let query_code = format!("select {} from  testdata_cy {}", queryvalues, query_ty);
    let stream = client.simple_query(query_code).await.unwrap();
    let result = stream.into_row().await.unwrap();
    match result {
        Some(r) => {
            let sn = sn.clone();
            let cus_pn = r.get::<&str, _>(0).unwrap().to_string();
            let sntitle = r.get::<&str, _>(1).unwrap().to_string();
            let in_name = r.get::<&str, _>(2).unwrap().to_string();
            let inloss1 = r.get::<Numeric, _>(3).unwrap().to_string();
            let reloss1 = r.get::<Numeric, _>(4).unwrap().to_string();
            let out_name = r.get::<&str, _>(5).unwrap().to_string();
            let inloss2 = r.get::<Numeric, _>(6).unwrap().to_string();
            let reloss2 = r.get::<Numeric, _>(7).unwrap().to_string();
            let print_num = r.get::<i32, _>(8).unwrap();
            let datainfo = DataInfo {
                sn,
                cus_pn,
                sntitle,
                in_name,
                inloss1,
                reloss1,
                out_name,
                inloss2,
                reloss2,
                print_num,
            };
            list.push(datainfo);
            list
        }
        None => vec![],
    }
}
async fn examine(sn: String, sql: String) -> String {
    let mut client = client(sql).await;
    let query_ty = format!("where SN = '{}'", sn);
    let query_code = format!("select print_num from  testdata_cy {}", query_ty);
    let stream = client.simple_query(query_code).await.unwrap();
    let result = stream.into_row().await.unwrap();
    match result {
        Some(row) => {
            let status = row.get::<i32, _>(0).unwrap();
            if status == 0 {
                "Ok".to_string()
            } else {
                "条码非第一次打印".to_string()
            }
        }
        None => "Sn不存在".to_string(),
    }
}
pub async fn do_print(
    sn: String,
    sql: String,
    id: String,
    btw: String,
    printer: String,
    float: u32,
) -> String {
    let sn_check = examine(sn.clone(), sql.clone()).await;
    if sn_check == "Ok" {
        let datainfos = run_query(sn, sql.clone()).await;
        let datainfo = datainfos.get(0).unwrap();
        let sn = datainfo.sn.clone();
        let pn = datainfo.cus_pn.clone();
        let sntitle = datainfo.sntitle.clone();
        let in_name = datainfo.in_name.clone();
        let out_name = datainfo.out_name.clone();
        let inloss1_c = &datainfo.inloss1[..3];
        let inloss2_c = &datainfo.inloss2[..3];
        let pt: usize = if float == 2 { 5 } else { 2 };
        let printer = printer.clone();
        let reloss1_c = &datainfo.reloss1[..pt];
        let reloss2_c = &datainfo.reloss2[..pt];
        let btw = &btw.clone();
        let data = json!({
                                // 模版库的ID
                            "LibraryID": format!("{}",id),
                                // 模版的绝对路径,与相对路径二者选其一
                            // "AbsolutePath": "global_test.btw",
                                // 模版的相对路径，例如：Automotive/AIAG/B-10/BMW.btw
                            "relativePath": format!("{}",btw),

                                // 打印机名称
                            "printer": format!("{}",printer),
                                // 起始位置（一般不传，从参数中拿掉）
                            // "StartingPosition": 1,
                                // /打印份数
                            "Copies": 1,
                                // 自增序列
                            "SerialNumbers": format!("{}",0),
                                // 老版软件设置参数接口
                            // "DataEntryControls": {
                                // 新版软件设置参数接口
                            "namedDataSources": {
                                "SN":format!("{}",sn),
                                "PN":format!("{}",pn),
                                "TITLE":format!("{}",sntitle),
                                "INLOSS1":format!("≤{}dB",inloss1_c),
                                "INLOSS2":format!("≤{}dB",inloss2_c),
                                "JK1":format!("{}",in_name),
                                "JK2":format!("{}",out_name),
                                "RELOSS1":format!("≥{}dB",reloss1_c),
                                "RELOSS2":format!("≥{}dB",reloss2_c),
        }
            });
        println!("data = {}", data);

        let worker_thread = spawn(async move {
            let url = "http://192.168.2.186/BarTender/api/v1/print";
            let res = ureq::post(url).send_json(data).unwrap();
            let value = res.status_text();
            let res = if value == "OK" {
                updata(datainfos, sql).await
            } else {
                "打印错误".to_string()
            };
            res
        });
        match worker_thread.await {
            Ok(b) => b,

            Err(_e) => "打印错误".to_string(),
        }
    } else {
        sn_check
    }
}

pub async fn updata(list: Vec<DataInfo>, sql: String) -> String {
    let data = list.get(0).unwrap();
    let print_num = data.print_num.clone();
    let sn = data.sn.clone();
    let new_print_num = print_num + 1;
    let date = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut client = client(sql).await;
    let stream = client
        .execute(
            format!(
                "UPDATE testdata_cy set print_num = '{}', print_date = '{}' WHERE sn = '{}'",
                new_print_num, date, sn
            ),
            &[&1i32],
        )
        .await;
    match stream {
        Ok(_) => "打印完成!!!".to_string(),
        Err(e) => e.to_string(),
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[derive(Debug, Clone, Default)]
pub struct DataInfo {
    pub sn: String,
    pub cus_pn: String,
    pub sntitle: String,
    pub in_name: String,
    pub inloss1: String,
    pub reloss1: String,
    pub out_name: String,
    pub inloss2: String,
    pub reloss2: String,
    pub print_num: i32,
}
