use super::pixoo::{Image, Pixoo};
use std::collections::HashMap;
use super::vec2::Vec2;
use divoom::*;
//use png_pong::{PngImage};

pub struct PixooCam {
    images: Vec<Image>,
}

impl PixooCam {

    pub fn new() -> Self {
        PixooCam {
            images: Vec::new(),
         }
     }

    pub fn render_image(&mut self,hash:&HashMap<Vec2, char>,w:usize,h:usize)  {
     
        let image = Self::render_buff(hash, w, h).unwrap();
        self.images.push(image.clone());

        //pix.add_image(&image);
        //pix.save_all();
        
    }

    pub async fn save_all(&mut self) {


//                let divoom = DivoomServiceClient::new();
//        let devices = divoom.get_same_lan_devices().await.unwrap(); // Use unwrap to handle the Result
//        devices.iter().for_each(|x| 
//            println!("{:?}", x )
//    //      res = x.device_private_ip.to_string();
//        );
//
//        return ;


        let mut pix = Pixoo::new(64,64);
        let mut frame = 0;

        for img in &self.images 
        {
            let mut img = img.clone();
            let s = format!("iter {}", frame);
            img.draw_text(s, 1, 1,  255, 0, 0);
            pix.add_image(&img);
            if frame > 10 {
                break;
            }
            frame += 1;
        }
        pix.save_all();

        //pix.pixoo_test_text().await; 
        pix.load_to_pixoo("192.168.8.113".to_string()).await;
        //Pixoo::find().await;



    }

    fn render_buff(hash:&HashMap<Vec2, char>,ww:usize,hh:usize) -> Result<Image, String> {

        let w = 64;
        let h = 64;
        let mut buff = vec![vec![0usize; w]; h];
        let mut cnt = vec![vec![0usize; w]; h];

        for (pos, ch) in hash.iter() {
            let x = (((pos.x as f32)*64.0f32)/(ww as f32)).trunc() as usize;
            let y = (((pos.y as f32)*64.0f32)/(hh as f32)).trunc() as usize;
            if x < w && y < h 
            {
                buff[y][x]+=*ch as usize;
                cnt[y][x]+=1;
            }
        }

        for y in 0..h {
            for x in 0..w {
                if cnt[y][x]>0 {
                    buff[y][x] = (buff[y][x] as f32/cnt[y][x] as f32) as usize;
                }
            }
        }
        let max_val = buff.iter().flat_map(|row| row.iter()).cloned().max().unwrap_or(1);

        let mut image = Image::new(w , h );
        for y in 0..h {
            for x in 0..w {                
                let t = buff[y][x] as f32 / max_val as f32;

                //let r = (v * 140.0f32).min(255.0f32) as u8;
                //let g = (v * 120.0f32).min(255.0f32) as u8;
                //let b = (v * 110.0f32).min(255.0f32) as u8;

                let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
                let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
                let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
           
                image.set_pixel(x as i32, y as i32, r, g, b);
                //((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            }
        }
        
        

        Ok(image)
    }

}