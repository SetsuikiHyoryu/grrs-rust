use std::io::Write;

/// 查找匹配内容
///
/// 从给定的内容里找出包含模式的行，并写入到输出里。
///
/// # 参数
///
/// `content` - 内容
/// `pattern` - 模式
/// `write` - 输出源（需要实现 [`Write`]）
///
/// [`Write`]: std::io::Write
pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{line}").unwrap();
        }
    }
}

#[test]
fn find_a_match() {
    let content = "Suzuran wearing white tights.\nAnd than she getup from bed.";
    let pattern = "white tights";

    let mut result = Vec::new();
    find_matches(content, pattern, &mut result);

    // “b 使得它被当作 byte 字节，即其类型为 &[u8] 而非 &str。”
    // See: https://suibianxiedianer.github.io/rust-cli-book-zh_CN/tutorial/testing_zh.html#%E7%BC%96%E5%86%99%E5%8F%AF%E6%B5%8B%E8%AF%95%E7%9A%84%E4%BB%A3%E7%A0%81
    assert_eq!(result, b"Suzuran wearing white tights.\n")
}
