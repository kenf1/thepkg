use image::{self,DynamicImage,GenericImageView,ImageFormat};
use reqwest::blocking::get;
use image::imageops::CatmullRom; //resize algorithm
use bardecoder;

//import image from local path
pub fn image_import(image_path: &str) -> DynamicImage{
    let temp_img = image::open(image_path.to_string()).unwrap();
    return temp_img;
}

//import image from url (need to unwrap outside function)
pub fn image_from_url(url: &str) -> Result<DynamicImage,Box<dyn std::error::Error>>{
    let img_bytes = get(url)?
        .bytes()?
        .to_vec();
    let image = image::load_from_memory(&img_bytes)?;
    Ok(image)
}

//determine image dimensions & resize if any >= 600 pixels
pub fn image_dimensions(image: DynamicImage,orig_cutoff: u32,new_cutoff: u32) -> DynamicImage{
    //set check & resize cutoffs 
    let (orig_cutoff,new_dim) = (orig_cutoff,new_cutoff);

    let (width,height) = image.dimensions();
    println!("Image dimensions are: {} pixels in width x {} pixels in height.",width,height);

    /*
        resize image if width &/or height >= 600 pixels, else return original
            - maintains orig aspect ratio
            - uses CatmullRom filter (https://docs.rs/image/latest/image/imageops/enum.FilterType.html)
    */
    if (width >= orig_cutoff) | (height >= orig_cutoff){
        println!("Image width and/or height >= {} pixels -> will be resized, maintaining original aspect ratio.",orig_cutoff);

        let new_image = DynamicImage::resize(&image,new_dim,new_dim,CatmullRom);
        println!("Resized image dimensions: {:?}",new_image.dimensions());
        
        return new_image;
    }else{
        return image;
    }
}

//decode image
pub fn image_decode(image: DynamicImage){
    //default decoder
    let decoder = bardecoder::default_decoder();

    //print results
    let results = decoder.decode(&image);
    for result in results{
        println!("QR Code content: \n {}",result.unwrap());
    }
}

//wrapper function for local image
pub fn from_local(image_path: &str,orig_cutoff: u32,new_cutoff: u32){
    //import image & resize if nec
    let qrcode = image_import(image_path);
    let tidy = image_dimensions(qrcode,orig_cutoff,new_cutoff);
    
    //print qr code contents
    image_decode(tidy);
}

//wrapper function for remote image (same structure as from_local)
pub fn from_remote(url: &str,orig_cutoff: u32,new_cutoff: u32){
    //import image & resize if nec
    let qrcode = image_from_url(url);
    let tidy = image_dimensions(qrcode.unwrap(),orig_cutoff,new_cutoff);
    image_decode(tidy);
}

//save image from url (keep original dimensions + aspect ratio)
pub fn save_img(url: &str,path: &str,extension: ImageFormat){
    let image = image_from_url(url);
    image
        .unwrap()
        .save_with_format(path,extension)
        .unwrap();
}

//save image from url (resize image while maintaining original aspect ratio)
pub fn save_img_resized(url: &str,orig_cutoff: u32,new_cutoff: u32,path: &str,extension: ImageFormat){
    let image = image_from_url(url);
    
    let resized = image_dimensions(image.unwrap(),orig_cutoff,new_cutoff);
    resized
        .save_with_format(path,extension)
        .unwrap();
}