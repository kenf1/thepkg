#[cfg(test)]
#[cfg(feature = "qr")]
use thepkg::imagefn;

//decode from local image
#[test]
#[cfg(feature = "qr")]
fn from_local_pass() {
    let file_path = "./tests/testqr.jpg";
    let (orig_cutoff, new_cutoff) = (600, 300);

    //check if success
    let result = imagefn::from_local(file_path, orig_cutoff, new_cutoff);
    assert!(result.is_ok());

    //confirm result
    let decoded_string = result.unwrap();
    assert_eq!(
        decoded_string,
        "http://bw-winelist-website-prod.s3-website-us-west-2.amazonaws.com/winelist-demo/"
    );
}

#[test]
#[cfg(feature = "qr")]
fn from_local_fail() {
    let file_path = "./tests/testqr";
    let (orig_cutoff, new_cutoff) = (600, 300);

    let res = imagefn::from_local(file_path, orig_cutoff, new_cutoff);
    assert!(res.is_err(), "No such file or directory (os error 2)");

    let error_message = res.unwrap_err().to_string();
    assert_eq!(error_message, "No such file or directory (os error 2)");
}

//decode from url
#[test]
#[cfg(feature = "qr")]
fn from_remote_pass() {
    let url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    let (orig_cutoff, new_cutoff) = (600, 300);

    let res = imagefn::from_remote(url, orig_cutoff, new_cutoff);
    assert!(res.is_ok(), "Expected an ok result but got {res:?}");

    let decoded_string = res.unwrap();
    assert_eq!(decoded_string, "http://cblink.je/app-install-display-nl");
}

#[test]
#[cfg(feature = "qr")]
fn from_remote_fail() {
    let url = "dne.com";
    let (orig_cutoff, new_cutoff) = (600, 300);

    let res = imagefn::from_remote(url, orig_cutoff, new_cutoff);
    assert!(res.is_err(), "Expected an error but got {res:?}");

    let error_message = res.unwrap_err().to_string();
    assert_eq!(error_message, "builder error");
}

//save image from url
#[test]
#[cfg(feature = "qr")]
fn save_img_fail() {
    let img_url = "https://github.com/";
    let result = imagefn::save_img(
        img_url,
        "./tests/Example.jpg",
        image::ImageFormat::Jpeg,
    );

    assert!(result.is_err(), "Expected an error but got {result:?}");

    //will get 1 of 2 errors dep on whether cargo clean was run
    assert!(matches!(
        result.err().unwrap().to_string(),
        s if s == "error sending request for url (https://github.com/)" ||
        s == "The image format could not be determined"
    ));
}

//takes too long in Actions CI/CD, passes on local
#[test]
#[cfg(feature = "archive")]
fn save_img_fail1() {
    let img_url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    let result =
        imagefn::save_img(img_url, "./tests", image::ImageFormat::Jpeg);

    assert!(result.is_err(), "Expected an error but got {:?}", result);

    //will get 1 of 2 errors dep on whether cargo clean was run
    assert!(matches!(
        result.err().unwrap().to_string(),
        s if s == "error sending request for url (https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true)" ||
        s == "Is a directory (os error 21)"
    ));
}
