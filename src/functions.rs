extern crate image;

use image::{DynamicImage, GenericImageView, GenericImage,  RgbaImage,  imageops, Pixel, Rgba};

pub fn split_into_chunks(mut img: &mut DynamicImage, horiz: u32, vert: u32) -> Result<Vec<Vec<DynamicImage>>, ()> {
    let (width, height) = img.dimensions();
    let pixel_width = (width / horiz) as u32;
    let pixel_height = (height / vert) as u32;
    //Adds all internal subimage chunks (chunks that are full (not edges)) 
    let mut ret: Vec<Vec<DynamicImage>> = Vec::new();
    for  v in 0..vert {
        let mut row: Vec<DynamicImage> = Vec::new();        
        for h in 0..horiz {
            let mut curwidth = pixel_width;
            let mut curheight = pixel_height;
            if h == horiz-1 {
                curwidth = width - (pixel_width*(horiz-1));
            }
            if v == vert-1 {
                curheight = height- (pixel_height*(vert-1));
            }
            let mut chunk = imageops::crop( img, h*pixel_width, v*pixel_height, curwidth, curheight).to_image();
            
            // for i in h*pixel_width..curwidth {
            //     for j in v*pixel_height..curheight{
            //         chunk.put_pixel(i, j, img.get_pixel(i, j));
            //     }
            // }
             
        row.push(DynamicImage::ImageRgba8(chunk));
        }
        ret.push(row);        
    }
    return Ok(ret);
}

pub fn encrypt(mut img: image::DynamicImage) -> image::DynamicImage{
    let (width, height) = img.dimensions();
    for i in 0..width{
        for j in 0..height{
            if i%2 == 0 && i<width/2{
                // let inv_pixel = img.get_pixel(width - i - 1, height - j - 1);
                let inv_pixel = img.get_pixel(width - i - 2, j);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel);
                img.put_pixel(width - i - 2, j, pixel);
            }
            else if j%2 == 0 && j < height/2{
                let inv_pixel = img.get_pixel(i, height - j - 2);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel); 
                img.put_pixel(i, height - j - 2, pixel); 
            }
            else{
                let mut pixel = img.get_pixel(i, j);
                let tmp = pixel.0[0];
                pixel.0[0] = pixel.0[1];
                pixel.0[1] = pixel.0[2];
                pixel.0[2] = tmp;
                img.put_pixel(i, j, pixel);
            }
        }
    }
    img
}

pub fn decrypt(mut img: image::DynamicImage) -> image::DynamicImage{
    let (width, height) = img.dimensions();
    for i in 0..width{
        for j in 0..height{
            if i%2 == 0 && i<width/2{
                // let inv_pixel = img.get_pixel(width - i - 1, height - j - 1);
                let inv_pixel = img.get_pixel(width - i - 2, j);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel);
                img.put_pixel(width - i - 2, j, pixel);
            }
            else if j%2 == 0 && j < height/2{
                let inv_pixel = img.get_pixel(i, height - j - 2);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel); 
                img.put_pixel(i, height - j - 2, pixel); 
            }
            else{
                let mut pixel = img.get_pixel(i, j);
                let tmp = pixel.0[2];
                pixel.0[2] = pixel.0[1];
                pixel.0[1] = pixel.0[0]; 
                pixel.0[0] = tmp;
                img.put_pixel(i, j, pixel);
            }
        }
    }
    img
}

pub fn dec(bitSequence: Vec<i32>) -> i32{
    let mut decimal = 0;
    for bit in bitSequence{
        decimal = decimal * 2 + (bit as i32);
    }
    return decimal
}

pub fn genHenonMap( dimensionX: u32, dimensionY: u32, key: [f64; 2]) -> Vec<Vec<i32>>{
    let mut x: f64 = key[0];
    let mut y: f64 = key[1];
    let seqSize = dimensionX*dimensionY*8;
    let mut bitSequence = Vec::new();
    let mut byteArray = Vec::new();
    let mut TImageMatrix = Vec::new();
    for i in 0..seqSize{
        let xN = y + 1.0 - 1.4 * x*x;
        let yN = 0.3 * x;

        x = xN;
        y = yN;
        
        let mut bit = 0;

        if xN<= 0.4 {
            bit = 0
        }
        else {
            bit = 1
        }

        bitSequence.push(bit);

        if i % 8 == 7 {
            let decimal = dec(bitSequence);
            byteArray.push(decimal);
        bitSequence = Vec::new();
        }

        let byteArraySize = dimensionY*dimensionX*8;
        if i % byteArraySize == byteArraySize - 1{
            TImageMatrix.push(byteArray);
        }
        byteArray = Vec::new();
    }
    return TImageMatrix;
}

pub fn henonEncrypt(img: image::RgbaImage, key: [f64; 2]) -> image::DynamicImage {
    let imageMatrix = img;
    let dimensionX = imageMatrix.width();
    let dimensionY = imageMatrix.height();
    
    let transformationMatrix = genHenonMap(dimensionX, dimensionY, key);

    let mut resultantMatrix: RgbaImage = RgbaImage::new(dimensionX, dimensionY);

    for i in 0 as usize..dimensionX as usize{
        let mut row: Vec<Rgba<u8>> = Vec::new();
        for j in 0 as usize..dimensionY as usize{
            let mult = transformationMatrix[i][j];
            let channels = imageMatrix.get_pixel(i as u32, j as u32).channels();
            let pixel: Rgba<u8> = Pixel::from_channels(mult as u8 ^ channels[0], mult as u8 ^ channels[1], mult as u8 ^ channels[2], mult as u8 ^ channels[3]);

            row.push(pixel);
        }
        for a in 0..dimensionY as usize{
            resultantMatrix.put_pixel(i as u32, a as u32, row[a]);
        }
    }
    return DynamicImage::ImageRgba8(resultantMatrix);
}
