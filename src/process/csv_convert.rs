

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

// 定义运动员结构体,生日，球员号码等需要序列化转换格式
// 这里使用serde来转换
// rename设定为大小写驼峰
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
  name:String,
  position:String,
  #[serde(rename = "DOB")]
  dob:String,
  nationality:String,
  #[serde(rename = "Kit Number")]
  kit:u8
}
// anyhow 特征和Result特征兼容，导入anyhow的result就可以
//
pub fn process_csv(input:&str,output: &str) -> Result<()> {
    // 读取 fs reader
    // 从输入读取数据，? 表示返回错误处理
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record:Player = result?;
        ret.push(record);
    }

    // ? 非空处理
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}