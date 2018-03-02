// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use TextBuffer;
use ffi;
use gdk;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TargetList(Shared<ffi::GtkTargetList>);

    match fn {
        ref => |ptr| ffi::gtk_target_list_ref(ptr),
        unref => |ptr| ffi::gtk_target_list_unref(ptr),
        get_type => || ffi::gtk_target_list_get_type(),
    }
}

impl TargetList {
    pub fn add(&self, target: &gdk::Atom, flags: u32, info: u32) {
        unsafe {
            ffi::gtk_target_list_add(self.to_glib_none().0, target.to_glib_none().0, flags, info);
        }
    }

    pub fn add_image_targets(&self, info: u32, writable: bool) {
        unsafe {
            ffi::gtk_target_list_add_image_targets(self.to_glib_none().0, info, writable.to_glib());
        }
    }

    pub fn add_rich_text_targets(&self, info: u32, deserializable: bool, buffer: &TextBuffer) {
        unsafe {
            ffi::gtk_target_list_add_rich_text_targets(self.to_glib_none().0, info, deserializable.to_glib(), buffer.to_glib_none().0);
        }
    }

    pub fn add_text_targets(&self, info: u32) {
        unsafe {
            ffi::gtk_target_list_add_text_targets(self.to_glib_none().0, info);
        }
    }

    pub fn add_uri_targets(&self, info: u32) {
        unsafe {
            ffi::gtk_target_list_add_uri_targets(self.to_glib_none().0, info);
        }
    }

    pub fn find(&self, target: &gdk::Atom) -> Option<u32> {
        unsafe {
            let mut info = mem::uninitialized();
            let ret = from_glib(ffi::gtk_target_list_find(self.to_glib_none().0, target.to_glib_none().0, &mut info));
            if ret { Some(info) } else { None }
        }
    }

    pub fn remove(&self, target: &gdk::Atom) {
        unsafe {
            ffi::gtk_target_list_remove(self.to_glib_none().0, target.to_glib_none().0);
        }
    }
}
