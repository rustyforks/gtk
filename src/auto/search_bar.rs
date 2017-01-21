// This file was generated by gir (8a0b5e5) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
#[cfg(feature = "v3_10")]
use Entry;
use Widget;
use ffi;
#[cfg(feature = "v3_10")]
use gdk;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct SearchBar(Object<ffi::GtkSearchBar>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_search_bar_get_type(),
    }
}

impl SearchBar {
    #[cfg(feature = "v3_10")]
    pub fn new() -> SearchBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn connect_entry<T: IsA<Entry>>(&self, entry: &T) {
        unsafe {
            ffi::gtk_search_bar_connect_entry(self.to_glib_none().0, entry.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_search_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_search_mode(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_handle_event(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_search_mode(&self, search_mode: bool) {
        unsafe {
            ffi::gtk_search_bar_set_search_mode(self.to_glib_none().0, search_mode.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_show_close_button(&self, visible: bool) {
        unsafe {
            ffi::gtk_search_bar_set_show_close_button(self.to_glib_none().0, visible.to_glib());
        }
    }

    pub fn get_property_search_mode_enabled(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "search-mode-enabled".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_search_mode_enabled(&self, search_mode_enabled: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "search-mode-enabled".to_glib_none().0, Value::from(&search_mode_enabled).to_glib_none().0);
        }
    }

    pub fn get_property_show_close_button(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-close-button".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_show_close_button(&self, show_close_button: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-close-button".to_glib_none().0, Value::from(&show_close_button).to_glib_none().0);
        }
    }
}
