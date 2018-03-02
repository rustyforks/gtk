// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use CellRenderer;
use CellRendererAccelMode;
use CellRendererText;
use TreePath;
use ffi;
use gdk;
use gdk_ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CellRendererAccel(Object<ffi::GtkCellRendererAccel, ffi::GtkCellRendererAccelClass>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_accel_get_type(),
    }
}

impl CellRendererAccel {
    pub fn new() -> CellRendererAccel {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_accel_new()).downcast_unchecked()
        }
    }
}

impl Default for CellRendererAccel {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CellRendererAccelExt {
    fn get_property_accel_key(&self) -> u32;

    fn set_property_accel_key(&self, accel_key: u32);

    fn get_property_accel_mode(&self) -> CellRendererAccelMode;

    fn set_property_accel_mode(&self, accel_mode: CellRendererAccelMode);

    fn get_property_accel_mods(&self) -> gdk::ModifierType;

    fn set_property_accel_mods(&self, accel_mods: gdk::ModifierType);

    fn get_property_keycode(&self) -> u32;

    fn set_property_keycode(&self, keycode: u32);

    fn connect_accel_cleared<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_accel_edited<F: Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_mods_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_keycode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererAccel> + IsA<glib::object::Object>> CellRendererAccelExt for O {
    fn get_property_accel_key(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accel-key".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accel_key(&self, accel_key: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accel-key".to_glib_none().0, Value::from(&accel_key).to_glib_none().0);
        }
    }

    fn get_property_accel_mode(&self) -> CellRendererAccelMode {
        unsafe {
            let mut value = Value::from_type(<CellRendererAccelMode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accel-mode".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accel_mode(&self, accel_mode: CellRendererAccelMode) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accel-mode".to_glib_none().0, Value::from(&accel_mode).to_glib_none().0);
        }
    }

    fn get_property_accel_mods(&self) -> gdk::ModifierType {
        unsafe {
            let mut value = Value::from_type(<gdk::ModifierType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accel-mods".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accel_mods(&self, accel_mods: gdk::ModifierType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accel-mods".to_glib_none().0, Value::from(&accel_mods).to_glib_none().0);
        }
    }

    fn get_property_keycode(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "keycode".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_keycode(&self, keycode: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "keycode".to_glib_none().0, Value::from(&keycode).to_glib_none().0);
        }
    }

    fn connect_accel_cleared<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accel-cleared",
                transmute(accel_cleared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_accel_edited<F: Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accel-edited",
                transmute(accel_edited_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accel_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accel-key",
                transmute(notify_accel_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accel_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accel-mode",
                transmute(notify_accel_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accel_mods_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accel-mods",
                transmute(notify_accel_mods_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_keycode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::keycode",
                transmute(notify_keycode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn accel_cleared_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, path_string: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &&(Fn(&P, TreePath) + 'static) = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&CellRendererAccel::from_glib_borrow(this).downcast_unchecked(), path)
}

unsafe extern "C" fn accel_edited_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, path_string: *mut libc::c_char, accel_key: libc::c_uint, accel_mods: gdk_ffi::GdkModifierType, hardware_keycode: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &&(Fn(&P, TreePath, u32, gdk::ModifierType, u32) + 'static) = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&CellRendererAccel::from_glib_borrow(this).downcast_unchecked(), path, accel_key, from_glib(accel_mods), hardware_keycode)
}

unsafe extern "C" fn notify_accel_key_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererAccel::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accel_mode_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererAccel::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accel_mods_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererAccel::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_keycode_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererAccel::from_glib_borrow(this).downcast_unchecked())
}
