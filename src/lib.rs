use pbni::*;
use image::imageops::FilterType;
use image::{io::Reader, ColorType, DynamicImage, ImageResult,ImageFormat};
use std::io::{Cursor, ErrorKind};
struct ImageObject {
    session: Session,
    ctx: ContextObject,
    img: ImageResult<DynamicImage>,
}

impl ImageObject {
    fn context_mut(&mut self) -> &mut ContextObject {
        &mut self.ctx
    }
}
const fn convfilter(filter: &u16) -> FilterType {
    match filter {
        0 => FilterType::Nearest,
        1 => FilterType::Triangle,
        2 => FilterType::CatmullRom,
        3 => FilterType::Gaussian,
        4 => FilterType::Lanczos3,
        _ => FilterType::Nearest,
    }
}
const fn convfmt(imgfmt:&u16)->ImageFormat{
    match imgfmt {
        0=>{ImageFormat::Png},
        1=>{ImageFormat::Jpeg},
        2=>{ImageFormat::Gif},
        3=>{ImageFormat::WebP},
        4=>{ImageFormat::Pnm},
        5=>{ImageFormat::Tiff},
        6=>{ImageFormat::Tga},
        7=>{ImageFormat::Dds},
        8=>{ImageFormat::Bmp},
        9=>{ImageFormat::Ico},
        10=>{ImageFormat::Hdr},
        11=>{ImageFormat::OpenExr},
        12=>{ImageFormat::Farbfeld},
        13=>{ImageFormat::Avif},
        _=>{ImageFormat::Png},
    }
}

#[nonvisualobject(name = "nrs_tp_image")]
impl ImageObject {
    #[constructor]
    fn new(session: Session, ctx: ContextObject) -> ImageObject {
        ImageObject {
            session,
            ctx,
            img: Ok(DynamicImage::default()),
        }
    }
    #[method]
    fn load(&mut self, filebuff: Vec<u8>) -> bool {
        if let Ok(f) = Reader::new(Cursor::new(&filebuff)).with_guessed_format() {
            self.img = f.decode();
        } else {
            return false;
        }
        self.img.is_ok()
    }
    #[method]
    fn open(&mut self, filepath: String) -> bool {
        if let Ok(r) = Reader::open(&filepath) {
            self.img = r.decode();
        } else {
            return false;
        }
        self.img.is_ok()
    }
    #[method]
    fn save(&mut self, savepath: String) -> bool {
        if let Ok(b) = &self.img {
            return b.save(&savepath).is_ok();
        } else {
            return false;
        }
    }
    #[method]
    fn as_bytes(&mut self,imgtype:u16) -> Vec<u8> {
        let mut rt:Vec<u8> = vec![];
        if let Ok(b) = &self.img {
            let _ =  b.write_to(&mut Cursor::new(&mut rt), convfmt(&imgtype));
        }
        rt
    }
    #[method]
    fn height(&mut self) -> u32 {
        if let Ok(m) = &self.img {
            m.height()
        } else {
            0
        }
    }
    #[method]
    fn width(&mut self) -> u32 {
        if let Ok(m) = &self.img {
            m.width()
        } else {
            0
        }
    }
    #[method]
    fn resize(&mut self, nwidth: u32, nheight: u32, filtertype: u16) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.resize(nwidth, nheight, convfilter(&filtertype)));
        }
        obj
    }
    #[method]
    fn resize_exact(&mut self, nwidth: u32, nheight: u32, filtertype: u16) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.resize_exact(nwidth, nheight, convfilter(&filtertype)));
        }
        obj
    }
    #[method]
    fn resize_to_fill(&mut self, nwidth: u32, nheight: u32, filtertype: u16) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.resize_to_fill(nwidth, nheight, convfilter(&filtertype)));
        }
        obj
    }
    #[method]
    fn thumbnail(&mut self, nwidth: u32, nheight: u32) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.thumbnail(nwidth, nheight));
        }
        obj
    }
    #[method]
    fn thumbnail_exact(&mut self, nwidth: u32, nheight: u32) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.thumbnail_exact(nwidth, nheight));
        }
        obj
    }
    #[method]
    fn color(&mut self) -> u16 {
        if let Ok(m) = &self.img {
            match m.color() {
                ColorType::L8 => 1,
                ColorType::La8 => 2,
                ColorType::Rgb8 => 3,
                ColorType::Rgba8 => 4,
                ColorType::L16 => 5,
                ColorType::La16 => 6,
                ColorType::Rgb16 => 7,
                ColorType::Rgba16 => 8,
                ColorType::Rgb32F => 9,
                ColorType::Rgba32F => 10,
                _ => 1024,
            }
        } else {
            0
        }
    }
    #[method]
    fn grayscale(&mut self) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.grayscale());
        }
        obj
    }
    #[method]
    fn invert(&mut self) {
        if let Ok(im) = &mut self.img {
            im.invert();
        }
    }
    #[method]
    fn blur(&mut self, sigma: f32) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.blur(sigma));
        }
        obj
    }
    #[method]
    fn unsharpen(&mut self, sigma: f32, threshold: i32) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.unsharpen(sigma, threshold));
        }
        obj
    }
    #[method]
    fn adjust_contrast(&mut self, c: f32) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.adjust_contrast(c));
        }
        obj
    }
    #[method]
    fn brighten(&mut self, v: i32) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.brighten(v));
        }
        obj
    }
    #[method]
    fn huerotate(&mut self, v: i32) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.huerotate(v));
        }
        obj
    }
    #[method]
    fn flipv(&mut self) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.flipv());
        }
        obj
    }
    #[method]
    fn fliph(&mut self) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.fliph());
        }
        obj
    }
    #[method]
    fn rotate90(&mut self) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.rotate90());
        }
        obj
    }
    #[method]
    fn rotate180(&mut self) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.rotate180());
        }
        obj
    }
    #[method]
    fn rotate270(&mut self) -> Object {
        let mut obj = self.session.new_object("nrs_tp_image").unwrap();
        let s = unsafe { obj.get_native_mut::<ImageObject>().unwrap() };
        if let Ok(im) = &self.img {
            s.img = Ok(im.rotate270());
        }
        obj
    }
}
