

use clap::Parser;
use std::{fmt, str::FromStr};
use super::verify_input_file;
#[derive(Debug, Parser)]
// enum 可以只实现其中一组
pub enum Base64SubCommand {
    #[command(name = "encode" ,about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode" ,about = "Decode a base64 string")]
    Decode,
}

#[derive(Debug, Parser)]
// struct 需要全部实现
pub struct Base64EncodeOpts {
  #[arg(short, long,value_parser = verify_input_file, default_value = "-")]
  pub input: String,

  #[arg(long,value_parser = parse_base64_format, default_value = "standard")]
  pub format:Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
  #[arg(short, long,value_parser = verify_input_file, default_value = "-")]
  pub input: String,


  #[arg(long,value_parser = parse_base64_format, default_value = "standard")]
  pub format:Base64Format,

}
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
  // 需要实现impl fromstr 转换成对应字符串，才能解析
  format.parse()
}

// 给Base64Format 实现FromStr trait
// 实现from_str方法，将字符串转换成Base64Format枚举类型
// 序列化，字符串转base64Format枚举类型
impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::Error::msg(format!("Invalid base64 format: {}", s))),
        }
    }
}
// 反序列化
// base64Format枚举类型转字符串
impl From<Base64Format> for &'static str {
  fn from(format: Base64Format) -> Self {
      match format {
          Base64Format::Standard => "standard",
          Base64Format::UrlSafe => "urlsafe",
      }
  }
}

impl fmt::Display for Base64Format {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", Into::<&str>::into(*self))
  }
}