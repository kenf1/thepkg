use image::{DynamicImage,GenericImageView,ImageFormat};
use reqwest::blocking::get;
use image::imageops::CatmullRom; //resize algorithm
use bardecoder;

//import image from local path (allows error handling)
pub fn image_import(image_path: &str) -> 
Result<DynamicImage,Box<dyn std::error::Error>>{
    let temp_img = image::open(image_path)?;
    Ok(temp_img)
}

//import image from url (allows error handling)
pub fn image_from_url(url: &str) -> 
Result<DynamicImage,Box<dyn std::error::Error>>{
    let img_bytes = get(url)?
        .bytes()?
        .to_vec();

    let image = image::load_from_memory(&img_bytes)?;
    Ok(image)
}

//determine image dimensions & resize if any >= 600 pixels
pub fn image_dimensions(
    image: DynamicImage,orig_cutoff: u32,new_cutoff: u32
) -> DynamicImage{
    let (width,height) = image.dimensions();
    println!("Image dimensions are: {} pixels in width x {} pixels in height.", width, height);

    if width >= orig_cutoff || height >= orig_cutoff {
        println!("Image width and/or height >= {} pixels -> will be resized, maintaining original aspect ratio.",orig_cutoff);

        let new_image = DynamicImage::resize(&image,new_cutoff,new_cutoff,CatmullRom);
        println!("Resized image dimensions: {:?}", new_image.dimensions());
        new_image
    }else{
        image
    }
}

//decode image
pub fn image_decode(image: DynamicImage) -> 
Result<String,Box<dyn std::error::Error>>{
    let decoder = bardecoder::default_decoder();

    let decoded_items: Vec<String> = decoder
        .decode(&image)
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();
    Ok(decoded_items.join(""))
}

//wrapper function for local image
pub fn from_local(image_path: &str,orig_cutoff: u32,new_cutoff: u32) -> 
Result<String,Box<dyn std::error::Error>>{
    let qrcode = image_import(image_path)?;

    //resize if nec
    let tidy = image_dimensions(qrcode,orig_cutoff,new_cutoff);

    let res = image_decode(tidy)?;
    Ok(res)
}

//wrapper function for remote image (same structure as from_local)
pub fn from_remote(url: &str, orig_cutoff: u32, new_cutoff: u32) -> Result<String, Box<dyn std::error::Error>> {
    let qrcode = image_from_url(url)?;

    //resize if nec
    let tidy = image_dimensions(qrcode, orig_cutoff, new_cutoff);

    let res = image_decode(tidy)?;
    Ok(res)
}

//save image from url (keep original dimensions + aspect ratio)
pub fn save_img(url: &str,path: &str,extension: ImageFormat) -> 
Result<(),Box<dyn std::error::Error>>{
    let image = image_from_url(url)?;
    image.save_with_format(path,extension)?;
    Ok(())
}

//save image from url (resize image while maintaining original aspect ratio)
pub fn save_img_resized(
    url: &str,orig_cutoff: u32,new_cutoff: u32,
    path: &str,extension: ImageFormat
) -> Result<(),Box<dyn std::error::Error>>{
    let image = image_from_url(url)?;

    //resize image
    let resized = image_dimensions(image,orig_cutoff,new_cutoff);
    resized.save_with_format(path,extension)?;
    Ok(())
}