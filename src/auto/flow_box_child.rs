// This file was generated by gir (0fe730d) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FlowBoxChild(Object<ffi::GtkFlowBoxChild>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_flow_box_child_get_type(),
    }
}

impl FlowBoxChild {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    pub fn new() -> FlowBoxChild {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_flow_box_child_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
impl Default for FlowBoxChild {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FlowBoxChildExt {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn changed(&self);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_index(&self) -> i32;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn is_selected(&self) -> bool;

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);
}

impl<O: IsA<FlowBoxChild> + IsA<glib::object::Object> + glib::object::ObjectExt> FlowBoxChildExt for O {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn changed(&self) {
        unsafe {
            ffi::gtk_flow_box_child_changed(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_flow_box_child_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_child_is_selected(self.to_glib_none().0))
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_activate(&self) {
        let _ = self.emit("activate", &[]).unwrap();
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkFlowBoxChild, f: glib_ffi::gpointer)
where P: IsA<FlowBoxChild> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FlowBoxChild::from_glib_borrow(this).downcast_unchecked())
}
