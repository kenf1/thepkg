#[cfg(test)]

#[cfg(feature = "qr")]
use thepkg::imagefn;

//decode from local image
#[cfg(feature = "qr")]
#[test]
fn from_local_pass(){
    let file_path = "./tests/testqr.jpg";
    let (orig_cutoff,new_cutoff) = (600,300);

    let res = imagefn::from_local(file_path,orig_cutoff,new_cutoff);
    assert_eq!(res,"http://bw-winelist-website-prod.s3-website-us-west-2.amazonaws.com/winelist-demo/");
}

#[cfg(feature = "qr")]
#[test]
fn from_local_fail(){
    let file_path = "./tests/testqr";
    let (orig_cutoff,new_cutoff) = (600,300);

    let res = imagefn::from_local(file_path,orig_cutoff,new_cutoff);
    assert_eq!(res,"Error: unable to decode QR code");
}

//decode from url
#[cfg(feature = "qr")]
#[test]
fn from_remote_pass(){
    let url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    let (orig_cutoff,new_cutoff) = (600,300);

    let res = imagefn::from_remote(url,orig_cutoff,new_cutoff);
    assert_eq!(res,"http://cblink.je/app-install-display-nl");
}

#[cfg(feature = "qr")]
#[test]
fn from_remote_fail(){
    let url = "dne.com";
    let (orig_cutoff,new_cutoff) = (600,300);

    let res = imagefn::from_remote(url,orig_cutoff,new_cutoff);
    assert_eq!(res,"Error: unable to decode QR code");
}

//save image from url
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