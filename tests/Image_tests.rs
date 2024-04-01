#[cfg(test)]

#[cfg(feature = "qr")]
use thepkg::imagefn;

//from url wrapper function
#[cfg(feature = "qr")]
#[test]
fn from_remote_pass(){
    let url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";

    let (orig_cutoff,new_cutoff) = (600,300);
    let res = imagefn::from_remote(url,orig_cutoff,new_cutoff);

    assert_eq!(res,"http://cblink.je/app-install-display-nl");
}

//save image wrapper function
// #[cfg(feature = "qr")]
// #[test]
// fn save_img_pass(){
//     let img_url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
//     imagefn::save_img(img_url,"Example.jpg",image::ImageFormat::Jpeg);
// }

//local image DNE
#[cfg(feature = "qr")]
#[test]
#[should_panic]
fn local_import_fail(){
    _ = imagefn::image_import("./tests");
}

//url image DNE
#[cfg(feature = "qr")]
#[test]
#[should_panic]
fn url_import_fail(){
    let res = imagefn::image_from_url("https://github.com/");
    res.unwrap();
}

//test local wrapper function (local image DNE)
#[cfg(feature = "qr")]
#[test]
#[should_panic]
fn from_local_fail(){
    _ = imagefn::image_import("./tests");
}

//test local wrapper function (url image DNE)
#[cfg(feature = "qr")]
#[test]
#[should_panic]
fn from_url_fail(){
    let res = imagefn::image_from_url("https://github.com/");
    res.unwrap();
}

#[cfg(feature = "qr")]
#[test]
#[should_panic]
fn save_img_fail(){
    let img_url = "https://github.com/";
    imagefn::save_img(img_url,"./tests/Example.jpg",image::ImageFormat::Jpeg);
}

#[cfg(feature = "qr")]
#[test]
#[should_panic]
fn save_img_fail1(){
    let img_url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    imagefn::save_img(img_url,"./tests",image::ImageFormat::Jpeg);
}