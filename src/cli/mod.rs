mod csv;
mod genpass;
mod base64;
use std::path::Path;

use clap::Parser;

use self::{csv::CsvOptions,genpass::GenPassOptions};
pub use self::csv::OutputFormat;
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
// 输入命令解析
/** 函数输入 fileName 借用的str
 * 函数返回 正确的文件名或错误信息
 */
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
  if filename == "-" || Path::new(filename).exists() {
      Ok(filename.into())
  } else {
      Err("File does not exist")
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}