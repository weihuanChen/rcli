use std::{fmt, path::Path, str::FromStr};

use anyhow::Error;
use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "rcli", version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV,or convert CSV to other formats")]
    Csv(CsvOptions),
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOptions),
}
// 输出格式支持 json yaml

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    // 输出格式 value_parser格式转换
    #[arg(long,value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}
#[derive(Debug, Parser)]
pub struct GenPassOptions {
    #[arg(long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
fn parse_format(formart: &str) -> Result<OutputFormat, anyhow::Error> {
    formart.parse()
}
/**
 * 输入
 */
impl From<OutputFormat> for &'static str {
    fn from(formart: OutputFormat) -> Self {
        match formart {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
/**
 * 为每一个OutputFormat实现FromStr特征
 */
impl FromStr for OutputFormat {
    type Err = Error;
    // Self 为OutputFormat
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format")),
        }
    }
}
// 输入命令解析
/** 函数输入 fileName 借用的str
 * 函数返回 正确的文件名或错误信息
 */
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
