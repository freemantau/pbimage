# pbimage
Image type conversion, scaling, modification in Powerbuilder
> more information:[pbni-rs](https://crates.io/crates/pbni-rs)
* Build
```rust
cargo build --release
# Copy \target\i686-pc-windows-msvc\release\pbimage.dll file to the pb project directory
```

# Conversion
> **BMP,JPEG,GIF,PNG,ICO,TIFF,WEBP**
* File
```vbscript
nrs_tp_image nimage
nimage = create nrs_tp_image
if nimage.open("pic.jpg") then
    //Convert jpg images to png format,save to file;
    nimage.save("pic.png")
end if
```
* Blob
```vbscript
blob bimg
nrs_tp_image nimage
nimage = create nrs_tp_image
if nimage.open("pic.jpg") then
    //Convert jpg images to png format,data to blob;
    bimg = nimage.as_bytes(enum.IMAGEFORMAT_PNG))
end if
```

# Scaling
* Thumbnail
```vbscript
nrs_tp_image nimage
nimage = create nrs_tp_image
if nimage.open("pic.jpg") then
    //Generate 200 * 200 thumbnail and convert to png
    nimage.thumbnail(200,200).save("pic.png")
end if
```
* Resize
```vbscript
nrs_tp_image nimage
nimage = create nrs_tp_image
if nimage.open("pic.jpg") then
    //Resize to 200 * 200 with FILTERTYPE_NEAREST and convert to png
    nimage.resize(200,200,enum.FILTERTYPE_NEAREST).save("pic.png")
end if
``` 
# Modify
| Function Name      | Description |
| ----------- | ----------- |
| public function ulong height()     | height       |
| public function ulong width()   | width        |
| public function nrs_tp_image grayscale()  | grayscale        |
| public subroutine invert()  | invert        |
| public function nrs_tp_image blur(readonly real sigma)  | blur        |
| public function nrs_tp_image unsharpen(readonly real sigma,readonly long threshold)  | unsharpen        |
| public function nrs_tp_image adjust_contrast(readonly real c)  | adjust_contrast        |
| public function nrs_tp_image brighten(readonly long v)  | brighten        |
| public function nrs_tp_image huerotate(readonly long v)  | huerotate        |
| public function nrs_tp_image flipv()  | flipv        |
| public function nrs_tp_image fliph()  | fliph        |
| public function nrs_tp_image rotate90()  | rotate90        |
| public function nrs_tp_image rotate180()  | rotate180        |
| public function nrs_tp_image rotate270()  | rotate270        |
# Chaining Style
```vbscript
nrs_tp_image nimage
nimage = create nrs_tp_image
if nimage.open("pic.jpg") then
    //Chaining Style
    nimage.thumbnail(200,200).grayscale().blur(2.0).flipv().save("pic.png")
end if
```

# More...
```vbscript
public function nrs_tp_image resize(readonly ulong nwidth,readonly ulong nheight,readonly uint filtertype)
public function nrs_tp_image resize_exact(readonly ulong nwidth,readonly ulong nheight,readonly uint filtertype)
public function nrs_tp_image resize_to_fill(readonly ulong nwidth,readonly ulong nheight,readonly uint filtertype)
public function nrs_tp_image thumbnail(readonly ulong nwidth,readonly ulong nheight)
public function nrs_tp_image thumbnail_exact(readonly ulong nwidth,readonly ulong nheight)
public function uint color()
```

