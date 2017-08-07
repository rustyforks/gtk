// This file was generated by gir (3294959) from gir-files (0bcaef9)
// DO NOT EDIT

use Bin;
use CellEditable;
use CellLayout;
use ComboBox;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ComboBoxText(Object<ffi::GtkComboBoxText>): ComboBox, Bin, Container, Widget, CellEditable, CellLayout;

    match fn {
        get_type => || ffi::gtk_combo_box_text_get_type(),
    }
}

impl ComboBoxText {
    pub fn new() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_text_new()).downcast_unchecked()
        }
    }

    pub fn new_with_entry() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_text_new_with_entry()).downcast_unchecked()
        }
    }
}

impl Default for ComboBoxText {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ComboBoxTextExt {
    fn append<'a, P: Into<Option<&'a str>>>(&self, id: P, text: &str);

    fn append_text(&self, text: &str);

    fn get_active_text(&self) -> Option<String>;

    fn insert<'a, P: Into<Option<&'a str>>>(&self, position: i32, id: P, text: &str);

    fn insert_text(&self, position: i32, text: &str);

    fn prepend<'a, P: Into<Option<&'a str>>>(&self, id: P, text: &str);

    fn prepend_text(&self, text: &str);

    fn remove(&self, position: i32);

    fn remove_all(&self);
}

impl<O: IsA<ComboBoxText>> ComboBoxTextExt for O {
    fn append<'a, P: Into<Option<&'a str>>>(&self, id: P, text: &str) {
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            ffi::gtk_combo_box_text_append(self.to_glib_none().0, id.0, text.to_glib_none().0);
        }
    }

    fn append_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn get_active_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_combo_box_text_get_active_text(self.to_glib_none().0))
        }
    }

    fn insert<'a, P: Into<Option<&'a str>>>(&self, position: i32, id: P, text: &str) {
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            ffi::gtk_combo_box_text_insert(self.to_glib_none().0, position, id.0, text.to_glib_none().0);
        }
    }

    fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert_text(self.to_glib_none().0, position, text.to_glib_none().0);
        }
    }

    fn prepend<'a, P: Into<Option<&'a str>>>(&self, id: P, text: &str) {
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            ffi::gtk_combo_box_text_prepend(self.to_glib_none().0, id.0, text.to_glib_none().0);
        }
    }

    fn prepend_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn remove(&self, position: i32) {
        unsafe {
            ffi::gtk_combo_box_text_remove(self.to_glib_none().0, position);
        }
    }

    fn remove_all(&self) {
        unsafe {
            ffi::gtk_combo_box_text_remove_all(self.to_glib_none().0);
        }
    }
}
