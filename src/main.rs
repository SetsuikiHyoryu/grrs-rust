use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
};
use structopt::StructOpt;

/// 在文件中检索一个模式，并显示包含这个模式的行。
#[derive(StructOpt, Debug)]
struct Cli {
    /// 检索用的模式
    pattern: String,

    /// 读取用的文件路径
    // PathBuf 为可跨平台路径类型，特性类似于 Sring。
    // See: https://suibianxiedianer.github.io/rust-cli-book-zh_CN/tutorial/cli-args_zh.html
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let content =
        File::open(&args.path).with_context(|| format!("Error reading `{:?}`", args.path,))?;
    let mut reader = BufReader::new(content);

    let mut buffer = String::new();
    let _ = reader.read_to_string(&mut buffer);

    // “由于 stdout 接收的是字节（而不是字符串）, 所以我们使用 std::io::Write 而不是 std::fmt::Write。”
    // See: https://suibianxiedianer.github.io/rust-cli-book-zh_CN/tutorial/testing_zh.html#%E7%BC%96%E5%86%99%E5%8F%AF%E6%B5%8B%E8%AF%95%E7%9A%84%E4%BB%A3%E7%A0%81
    grrs_hyoryu::find_matches(&buffer, &args.pattern, &mut io::stdout());

    Ok(())
}
