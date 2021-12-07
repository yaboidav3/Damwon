extern crate image;

pub mod henon;
pub mod chunks;
pub mod thread;

use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage, RgbaImage, SubImage, imageops};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.


    /* Chunks tutorial */
    let mut img =  image::open("InputImages/Cat-PNG.png").unwrap();    

    let key = henon::Key {
        x: 0.1,
        y: 0.2,
        horizontal_chunks: 3,
        vertical_chunks: 3,
    };

    let vec : Vec<Vec<DynamicImage>> = chunks::split_into_chunks(&mut img, key.horizontal_chunks, key.vertical_chunks).unwrap();
    let mut fin: Vec<Vec<DynamicImage>>  = Vec::new();
    // prints out all chunks CREATES A LOT OF FILES
    for h in 0..key.horizontal_chunks as usize{
        let mut add: Vec<DynamicImage> = Vec::new();
        for v in 0..key.vertical_chunks as usize{
            let s =  "ImageBin/".to_string() + &*format!("{}{}", v, h) + ".png";
            let temp = henon::henonEncrypt(vec[v][h].to_rgba8(), &key);
            temp.save( s).unwrap();
            if fin.len() < v + 1{
                fin.push(Vec::new());
            }
            fin[v].push(temp);
        }
    }
    let s =  "ImageBin/combined.png";
    let d =  "ImageBin/decrypted.png";
    let x = chunks::combine_from_chunks(fin, key.horizontal_chunks, key.vertical_chunks).unwrap();
    x.save(s).unwrap();
    henon::henonDecrypt(x.to_rgba8(), &key).save(d).unwrap();
    /* Encrypt tutorial
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("bun.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
    let pixel = img.get_pixel(300, 300);
    // The color method returns the image's `ColorType`.
    println!("{:?}", pixel.0[0]);
    img = encrypt(img);
    img.save("encr.png").unwrap();
    img = decrypt(img);
    // img = encrypt(img);
    // Write the contents of this image to the Writer in PNG format.
    img.save("decr.png").unwrap();
    */
}


