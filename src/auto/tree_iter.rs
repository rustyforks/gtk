// This file was generated by gir (8a0b5e5) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TreeIter(Boxed<ffi::GtkTreeIter>);

    match fn {
        copy => |ptr| ffi::gtk_tree_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_iter_free(ptr),
    }
}
