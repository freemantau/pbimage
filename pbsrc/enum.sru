$PBExportHeader$enum.sru
forward
global type enum from nonvisualobject
end type
end forward

global type enum from nonvisualobject
end type
global enum enum

type variables
//image filtertype
CONSTANT UINT FILTERTYPE_Nearest = 0
CONSTANT UINT FILTERTYPE_Triangle = 1
CONSTANT UINT FILTERTYPE_CatmullRom = 2
CONSTANT UINT FILTERTYPE_Gaussian = 3
CONSTANT UINT FILTERTYPE_Lanczos3 = 4



CONSTANT UINT        ImageFormat_Png = 0
CONSTANT UINT        ImageFormat_Jpeg = 1
CONSTANT UINT        ImageFormat_Gif = 2
CONSTANT UINT        ImageFormat_WebP = 3
CONSTANT UINT        ImageFormat_Pnm = 4
CONSTANT UINT        ImageFormat_Tiff = 5
CONSTANT UINT        ImageFormat_Tga = 6
CONSTANT UINT        ImageFormat_Dds = 7
CONSTANT UINT        ImageFormat_Bmp = 8
CONSTANT UINT        ImageFormat_Ico = 9
CONSTANT UINT        ImageFormat_Hdr = 10
CONSTANT UINT        ImageFormat_OpenExr = 11
CONSTANT UINT        ImageFormat_Farbfeld = 12
CONSTANT UINT        ImageFormat_Avif = 13
end variables
on enum.create
call super::create
TriggerEvent( this, "constructor" )
end on

on enum.destroy
TriggerEvent( this, "destructor" )
call super::destroy
end on

