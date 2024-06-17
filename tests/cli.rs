use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::predicate;
use std::{error::Error, process::Command};

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("./test/file/doesnt/exist");

    cmd.assert()
        .failure()
        // NOTE: 这里的预期字符串需要根据真实的系统调整
        .stderr(predicate::str::contains("指定されたパスが見つかりません"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
