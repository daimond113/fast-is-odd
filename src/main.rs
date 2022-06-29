use std::env;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::Write;

fn env_val_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(
        env::var_os(key)
            .unwrap_or(OsString::from(default.to_string()))
            .into_string()
            .unwrap(),
    )
}

fn main() {
    let base_file = r#"/**
  Very fast is-odd implementation.
  * @author daimond113
  * @param {number} n - The number to check.
  * @returns True if the number is odd, false otherwise.
*/
function isOdd(n) {
    if (n === -9007199254740991) {
        return false;
    }"#;
    //
    let mut cjs_file = File::create("dist/index.js").unwrap();
    cjs_file.write(base_file.as_bytes()).unwrap();
    let start: i128 = env_val_or_default("START_NUMBER", "-9007199254740991")
        .parse()
        .unwrap();
    let end: i128 = env_val_or_default("END_NUMBER", "9007199254740991")
        .parse()
        .unwrap();

    for i in start..=end {
        cjs_file
            .write(
                format!(
                    "
    else if (n === {}) {{
        return {}
    }}",
                    i,
                    i & 1 == 1,
                )
                .as_bytes(),
            )
            .unwrap();
    }
    cjs_file
        .write(
            b"
    else {
        return false;
    }",
        )
        .unwrap();
    cjs_file
        .write(
            b"
}\n",
        )
        .unwrap();
    fs::copy("dist/index.js", "dist/index.mjs").unwrap();
    let mut mjs_file = OpenOptions::new()
        .append(true)
        .open("dist/index.mjs")
        .unwrap();
    mjs_file
        .write(
            b"export default isOdd
export { isOdd }",
        )
        .unwrap();
    cjs_file.write(b"module.exports = isOdd").unwrap();
}
