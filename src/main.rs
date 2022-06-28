use std::fs::{self, File};
use std::io::Write;

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
    }
"#;
    //
    let mut cjs_file = File::create("dist/index.js").unwrap();
    cjs_file.write(base_file.as_bytes()).unwrap();
    let start: i128 = -9007199254740991;
    let end: i128 = 9007199254740991;

    for i in start..=end {
        cjs_file
            .write(
                format!(
                    "else if (n === {}) {{
            return {}            
}}\n",
                    i,
                    i % 2 == 0
                )
                .as_bytes(),
            )
            .unwrap();
    }
    cjs_file.write(b"}").unwrap();
    fs::copy("dist/index.js", "dist/index.mjs").unwrap();
    let mut mjs_file = File::create("dist/index.mjs").unwrap();
    mjs_file
        .write(
            b"export default isOdd
export { isOdd }",
        )
        .unwrap();
    cjs_file.write(b"module.exports = isOdd").unwrap();
}
