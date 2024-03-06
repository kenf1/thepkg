#[cfg(test)]

use thepkg::imagefn;

//local image DNE
#[test]
#[should_panic]
fn local_import_fail(){
    _ = imagefn::image_import("./tests");
}

//url image DNE
#[test]
#[should_panic]
fn url_import_fail(){
    let res = imagefn::image_from_url("https://github.com/");
    res.unwrap();
}

//test local wrapper function (local image DNE)
#[test]
#[should_panic]
fn from_local_fail(){
    _ = imagefn::image_import("./tests");
}

//test local wrapper function (url image DNE)
#[test]
#[should_panic]
fn from_url_fail(){
    let res = imagefn::image_from_url("https://github.com/");
    res.unwrap();
}

#[test]
#[should_panic]
fn save_img_fail(){
    let img_url = "https://github.com/";
    imagefn::save_img(img_url,"./tests/Example.jpg",image::ImageFormat::Jpeg);
}

#[test]
#[should_panic]
fn save_img_fail1(){
    let img_url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    imagefn::save_img(img_url,"./tests",image::ImageFormat::Jpeg);
}