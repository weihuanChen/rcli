use std::path::Path;

use clap::Parser;
#[derive(Debug,Parser)]
#[command(name = "rcli", version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV,or convert CSV to other formats")]
    Csv(CsvOptions),
}

#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short,long,value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short,long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 输入命令解析
/** 函数输入 fileName 借用的str
 * 函数返回 正确的文件名或错误信息
 */
fn verify_input_file(flie_name:&str) -> Result<String,&'static str> {
    if Path::new(flie_name).exists() {
      return Ok(flie_name.into())
    }else{
      return Err("File does not exist")
    }
}