#[cfg(test)]
#[cfg(feature = "io")]
use thepkg::iofn;

//confirm file exists
#[test]
#[cfg(feature = "io")]
fn confirm_file_exists() {
    let file_path = "./tests/testqr.jpg";
    let _ = iofn::file_exists(file_path);
}
