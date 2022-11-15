$PBExportHeader$nrs_tp_image.sru
forward
global type nrs_tp_image from nonvisualobject
end type
end forward

global type nrs_tp_image from nonvisualobject native "pbimage.dll"
public function boolean load(readonly blob filebuff)
public function boolean open(readonly string filepath)
public function boolean save(readonly string savepath)
public function blob as_bytes(readonly uint imageformat)
public function ulong height()
public function ulong width()
public function nrs_tp_image resize(readonly ulong nwidth,readonly ulong nheight,readonly uint filtertype)
public function nrs_tp_image resize_exact(readonly ulong nwidth,readonly ulong nheight,readonly uint filtertype)
public function nrs_tp_image resize_to_fill(readonly ulong nwidth,readonly ulong nheight,readonly uint filtertype)
public function nrs_tp_image thumbnail(readonly ulong nwidth,readonly ulong nheight)
public function nrs_tp_image thumbnail_exact(readonly ulong nwidth,readonly ulong nheight)
public function uint color()
public function nrs_tp_image grayscale()
public subroutine invert()
public function nrs_tp_image blur(readonly real sigma)
public function nrs_tp_image unsharpen(readonly real sigma,readonly long threshold)
public function nrs_tp_image adjust_contrast(readonly real c)
public function nrs_tp_image brighten(readonly long v)
public function nrs_tp_image huerotate(readonly long v)
public function nrs_tp_image flipv()
public function nrs_tp_image fliph()
public function nrs_tp_image rotate90()
public function nrs_tp_image rotate180()
public function nrs_tp_image rotate270()
end type
global nrs_tp_image nrs_tp_image

type variables
//
end variables

on nrs_tp_image.create
call super::create
TriggerEvent( this, "constructor" )
end on

on nrs_tp_image.destroy
TriggerEvent( this, "destructor" )
call super::destroy
end on

