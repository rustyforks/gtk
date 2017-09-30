// This file was generated by gir (0fe730d) from gir-files (db49619)
// DO NOT EDIT

use CellArea;
use CellLayout;
use CellRenderer;
use Container;
use IconViewDropPosition;
use MovementStep;
use Orientation;
use Scrollable;
use SelectionMode;
use Tooltip;
use TreeIter;
use TreeModel;
use TreePath;
use Widget;
use cairo;
use ffi;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use gdk;
use glib;
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
    pub struct IconView(Object<ffi::GtkIconView>): Container, Widget, CellLayout, Scrollable;

    match fn {
        get_type => || ffi::gtk_icon_view_get_type(),
    }
}

impl IconView {
    pub fn new() -> IconView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_area<P: IsA<CellArea>>(area: &P) -> IconView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new_with_area(area.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_model<P: IsA<TreeModel>>(model: &P) -> IconView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_icon_view_new_with_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for IconView {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IconViewExt {
    fn convert_widget_to_bin_window_coords(&self, wx: i32, wy: i32) -> (i32, i32);

    fn create_drag_icon(&self, path: &TreePath) -> Option<cairo::Surface>;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_activate_on_single_click(&self) -> bool;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_cell_rect<'a, P: IsA<CellRenderer> + 'a, Q: Into<Option<&'a P>>>(&self, path: &TreePath, cell: Q) -> Option<gdk::Rectangle>;

    fn get_column_spacing(&self) -> i32;

    fn get_columns(&self) -> i32;

    fn get_cursor(&self) -> Option<(TreePath, CellRenderer)>;

    fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32) -> Option<(TreePath, IconViewDropPosition)>;

    fn get_drag_dest_item(&self) -> (TreePath, IconViewDropPosition);

    fn get_item_at_pos(&self, x: i32, y: i32) -> Option<(TreePath, CellRenderer)>;

    fn get_item_column(&self, path: &TreePath) -> i32;

    fn get_item_orientation(&self) -> Orientation;

    fn get_item_padding(&self) -> i32;

    fn get_item_row(&self, path: &TreePath) -> i32;

    fn get_item_width(&self) -> i32;

    fn get_margin(&self) -> i32;

    fn get_markup_column(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    fn get_path_at_pos(&self, x: i32, y: i32) -> Option<TreePath>;

    fn get_pixbuf_column(&self) -> i32;

    fn get_reorderable(&self) -> bool;

    fn get_row_spacing(&self) -> i32;

    fn get_selected_items(&self) -> Vec<TreePath>;

    fn get_selection_mode(&self) -> SelectionMode;

    fn get_spacing(&self) -> i32;

    fn get_text_column(&self) -> i32;

    fn get_tooltip_column(&self) -> i32;

    fn get_tooltip_context(&self, x: &mut i32, y: &mut i32, keyboard_tip: bool) -> Option<(TreeModel, TreePath, TreeIter)>;

    fn get_visible_range(&self) -> Option<(TreePath, TreePath)>;

    fn item_activated(&self, path: &TreePath);

    fn path_is_selected(&self, path: &TreePath) -> bool;

    fn scroll_to_path(&self, path: &TreePath, use_align: bool, row_align: f32, col_align: f32);

    fn select_all(&self);

    fn select_path(&self, path: &TreePath);

    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/IconViewForeachFunc, data: P);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_activate_on_single_click(&self, single: bool);

    fn set_column_spacing(&self, column_spacing: i32);

    fn set_columns(&self, columns: i32);

    fn set_cursor<'a, P: IsA<CellRenderer> + 'a, Q: Into<Option<&'a P>>>(&self, path: &TreePath, cell: Q, start_editing: bool);

    fn set_drag_dest_item<'a, P: Into<Option<&'a TreePath>>>(&self, path: P, pos: IconViewDropPosition);

    fn set_item_orientation(&self, orientation: Orientation);

    fn set_item_padding(&self, item_padding: i32);

    fn set_item_width(&self, item_width: i32);

    fn set_margin(&self, margin: i32);

    fn set_markup_column(&self, column: i32);

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q);

    fn set_pixbuf_column(&self, column: i32);

    fn set_reorderable(&self, reorderable: bool);

    fn set_row_spacing(&self, row_spacing: i32);

    fn set_selection_mode(&self, mode: SelectionMode);

    fn set_spacing(&self, spacing: i32);

    fn set_text_column(&self, column: i32);

    fn set_tooltip_cell<'a, P: IsA<CellRenderer> + 'a, Q: Into<Option<&'a P>>>(&self, tooltip: &Tooltip, path: &TreePath, cell: Q);

    fn set_tooltip_column(&self, column: i32);

    fn set_tooltip_item(&self, tooltip: &Tooltip, path: &TreePath);

    fn unselect_all(&self);

    fn unselect_path(&self, path: &TreePath);

    fn unset_model_drag_dest(&self);

    fn unset_model_drag_source(&self);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn connect_activate_cursor_item<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_cursor_item(&self) -> bool;

    fn connect_item_activated<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_cursor(&self, step: MovementStep, count: i32) -> bool;

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_all(&self);

    fn connect_select_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_cursor_item(&self);

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_toggle_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_cursor_item(&self);

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_unselect_all(&self);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_columns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_markup_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IconView> + IsA<glib::object::Object> + glib::object::ObjectExt> IconViewExt for O {
    fn convert_widget_to_bin_window_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        unsafe {
            let mut bx = mem::uninitialized();
            let mut by = mem::uninitialized();
            ffi::gtk_icon_view_convert_widget_to_bin_window_coords(self.to_glib_none().0, wx, wy, &mut bx, &mut by);
            (bx, by)
        }
    }

    fn create_drag_icon(&self, path: &TreePath) -> Option<cairo::Surface> {
        unsafe {
            from_glib_full(ffi::gtk_icon_view_create_drag_icon(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_cell_rect<'a, P: IsA<CellRenderer> + 'a, Q: Into<Option<&'a P>>>(&self, path: &TreePath, cell: Q) -> Option<gdk::Rectangle> {
        let cell = cell.into();
        let cell = cell.to_glib_none();
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_icon_view_get_cell_rect(self.to_glib_none().0, mut_override(path.to_glib_none().0), cell.0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    fn get_column_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_column_spacing(self.to_glib_none().0)
        }
    }

    fn get_columns(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_columns(self.to_glib_none().0)
        }
    }

    fn get_cursor(&self) -> Option<(TreePath, CellRenderer)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut cell = ptr::null_mut();
            let ret = from_glib(ffi::gtk_icon_view_get_cursor(self.to_glib_none().0, &mut path, &mut cell));
            if ret { Some((from_glib_full(path), from_glib_none(cell))) } else { None }
        }
    }

    fn get_dest_item_at_pos(&self, drag_x: i32, drag_y: i32) -> Option<(TreePath, IconViewDropPosition)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            let ret = from_glib(ffi::gtk_icon_view_get_dest_item_at_pos(self.to_glib_none().0, drag_x, drag_y, &mut path, &mut pos));
            if ret { Some((from_glib_full(path), from_glib(pos))) } else { None }
        }
    }

    fn get_drag_dest_item(&self) -> (TreePath, IconViewDropPosition) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            ffi::gtk_icon_view_get_drag_dest_item(self.to_glib_none().0, &mut path, &mut pos);
            (from_glib_full(path), from_glib(pos))
        }
    }

    fn get_item_at_pos(&self, x: i32, y: i32) -> Option<(TreePath, CellRenderer)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut cell = ptr::null_mut();
            let ret = from_glib(ffi::gtk_icon_view_get_item_at_pos(self.to_glib_none().0, x, y, &mut path, &mut cell));
            if ret { Some((from_glib_full(path), from_glib_full(cell))) } else { None }
        }
    }

    fn get_item_column(&self, path: &TreePath) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_column(self.to_glib_none().0, mut_override(path.to_glib_none().0))
        }
    }

    fn get_item_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_item_orientation(self.to_glib_none().0))
        }
    }

    fn get_item_padding(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_padding(self.to_glib_none().0)
        }
    }

    fn get_item_row(&self, path: &TreePath) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_row(self.to_glib_none().0, mut_override(path.to_glib_none().0))
        }
    }

    fn get_item_width(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_item_width(self.to_glib_none().0)
        }
    }

    fn get_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_margin(self.to_glib_none().0)
        }
    }

    fn get_markup_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_markup_column(self.to_glib_none().0)
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_icon_view_get_model(self.to_glib_none().0))
        }
    }

    fn get_path_at_pos(&self, x: i32, y: i32) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_icon_view_get_path_at_pos(self.to_glib_none().0, x, y))
        }
    }

    fn get_pixbuf_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_pixbuf_column(self.to_glib_none().0)
        }
    }

    fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_reorderable(self.to_glib_none().0))
        }
    }

    fn get_row_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_row_spacing(self.to_glib_none().0)
        }
    }

    fn get_selected_items(&self) -> Vec<TreePath> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_icon_view_get_selected_items(self.to_glib_none().0))
        }
    }

    fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_icon_view_get_selection_mode(self.to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_spacing(self.to_glib_none().0)
        }
    }

    fn get_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_text_column(self.to_glib_none().0)
        }
    }

    fn get_tooltip_column(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_view_get_tooltip_column(self.to_glib_none().0)
        }
    }

    fn get_tooltip_context(&self, x: &mut i32, y: &mut i32, keyboard_tip: bool) -> Option<(TreeModel, TreePath, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut path = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_icon_view_get_tooltip_context(self.to_glib_none().0, x, y, keyboard_tip.to_glib(), &mut model, &mut path, iter.to_glib_none_mut().0));
            if ret { Some((from_glib_none(model), from_glib_full(path), iter)) } else { None }
        }
    }

    fn get_visible_range(&self) -> Option<(TreePath, TreePath)> {
        unsafe {
            let mut start_path = ptr::null_mut();
            let mut end_path = ptr::null_mut();
            let ret = from_glib(ffi::gtk_icon_view_get_visible_range(self.to_glib_none().0, &mut start_path, &mut end_path));
            if ret { Some((from_glib_full(start_path), from_glib_full(end_path))) } else { None }
        }
    }

    fn item_activated(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_item_activated(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_view_path_is_selected(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    fn scroll_to_path(&self, path: &TreePath, use_align: bool, row_align: f32, col_align: f32) {
        unsafe {
            ffi::gtk_icon_view_scroll_to_path(self.to_glib_none().0, mut_override(path.to_glib_none().0), use_align.to_glib(), row_align, col_align);
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_icon_view_select_all(self.to_glib_none().0);
        }
    }

    fn select_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_select_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/IconViewForeachFunc, data: P) {
    //    unsafe { TODO: call ffi::gtk_icon_view_selected_foreach() }
    //}

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_icon_view_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    fn set_column_spacing(&self, column_spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_column_spacing(self.to_glib_none().0, column_spacing);
        }
    }

    fn set_columns(&self, columns: i32) {
        unsafe {
            ffi::gtk_icon_view_set_columns(self.to_glib_none().0, columns);
        }
    }

    fn set_cursor<'a, P: IsA<CellRenderer> + 'a, Q: Into<Option<&'a P>>>(&self, path: &TreePath, cell: Q, start_editing: bool) {
        let cell = cell.into();
        let cell = cell.to_glib_none();
        unsafe {
            ffi::gtk_icon_view_set_cursor(self.to_glib_none().0, mut_override(path.to_glib_none().0), cell.0, start_editing.to_glib());
        }
    }

    fn set_drag_dest_item<'a, P: Into<Option<&'a TreePath>>>(&self, path: P, pos: IconViewDropPosition) {
        let path = path.into();
        unsafe {
            ffi::gtk_icon_view_set_drag_dest_item(self.to_glib_none().0, mut_override(path.to_glib_none().0), pos.to_glib());
        }
    }

    fn set_item_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_icon_view_set_item_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    fn set_item_padding(&self, item_padding: i32) {
        unsafe {
            ffi::gtk_icon_view_set_item_padding(self.to_glib_none().0, item_padding);
        }
    }

    fn set_item_width(&self, item_width: i32) {
        unsafe {
            ffi::gtk_icon_view_set_item_width(self.to_glib_none().0, item_width);
        }
    }

    fn set_margin(&self, margin: i32) {
        unsafe {
            ffi::gtk_icon_view_set_margin(self.to_glib_none().0, margin);
        }
    }

    fn set_markup_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_markup_column(self.to_glib_none().0, column);
        }
    }

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q) {
        let model = model.into();
        let model = model.to_glib_none();
        unsafe {
            ffi::gtk_icon_view_set_model(self.to_glib_none().0, model.0);
        }
    }

    fn set_pixbuf_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_pixbuf_column(self.to_glib_none().0, column);
        }
    }

    fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_icon_view_set_reorderable(self.to_glib_none().0, reorderable.to_glib());
        }
    }

    fn set_row_spacing(&self, row_spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_row_spacing(self.to_glib_none().0, row_spacing);
        }
    }

    fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_icon_view_set_selection_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_icon_view_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_text_column(self.to_glib_none().0, column);
        }
    }

    fn set_tooltip_cell<'a, P: IsA<CellRenderer> + 'a, Q: Into<Option<&'a P>>>(&self, tooltip: &Tooltip, path: &TreePath, cell: Q) {
        let cell = cell.into();
        let cell = cell.to_glib_none();
        unsafe {
            ffi::gtk_icon_view_set_tooltip_cell(self.to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0), cell.0);
        }
    }

    fn set_tooltip_column(&self, column: i32) {
        unsafe {
            ffi::gtk_icon_view_set_tooltip_column(self.to_glib_none().0, column);
        }
    }

    fn set_tooltip_item(&self, tooltip: &Tooltip, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_set_tooltip_item(self.to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_icon_view_unselect_all(self.to_glib_none().0);
        }
    }

    fn unselect_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_icon_view_unselect_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn unset_model_drag_dest(&self) {
        unsafe {
            ffi::gtk_icon_view_unset_model_drag_dest(self.to_glib_none().0);
        }
    }

    fn unset_model_drag_source(&self) {
        unsafe {
            ffi::gtk_icon_view_unset_model_drag_source(self.to_glib_none().0);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        let mut value = Value::from(None::<&CellArea>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cell-area".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_activate_cursor_item<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-cursor-item",
                transmute(activate_cursor_item_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_activate_cursor_item(&self) -> bool {
        let res = self.emit("activate-cursor-item", &[]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_item_activated<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "item-activated",
                transmute(item_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, MovementStep, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_cursor(&self, step: MovementStep, count: i32) -> bool {
        let res = self.emit("move-cursor", &[&step, &count]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-all",
                transmute(select_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_select_all(&self) {
        let _ = self.emit("select-all", &[]).unwrap();
    }

    fn connect_select_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-cursor-item",
                transmute(select_cursor_item_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_select_cursor_item(&self) {
        let _ = self.emit("select-cursor-item", &[]).unwrap();
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_toggle_cursor_item<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-cursor-item",
                transmute(toggle_cursor_item_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_toggle_cursor_item(&self) {
        let _ = self.emit("toggle-cursor-item", &[]).unwrap();
    }

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unselect-all",
                transmute(unselect_all_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_unselect_all(&self) {
        let _ = self.emit("unselect-all", &[]).unwrap();
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::activate-on-single-click",
                transmute(notify_activate_on_single_click_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-area",
                transmute(notify_cell_area_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::column-spacing",
                transmute(notify_column_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_columns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::columns",
                transmute(notify_columns_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_item_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::item-orientation",
                transmute(notify_item_orientation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_item_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::item-padding",
                transmute(notify_item_padding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_item_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::item-width",
                transmute(notify_item_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_margin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::margin",
                transmute(notify_margin_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_markup_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::markup-column",
                transmute(notify_markup_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::model",
                transmute(notify_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf-column",
                transmute(notify_pixbuf_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_reorderable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::reorderable",
                transmute(notify_reorderable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::row-spacing",
                transmute(notify_row_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::selection-mode",
                transmute(notify_selection_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::spacing",
                transmute(notify_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text-column",
                transmute(notify_text_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tooltip_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tooltip-column",
                transmute(notify_tooltip_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_cursor_item_trampoline<P>(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn item_activated_trampoline<P>(this: *mut ffi::GtkIconView, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P, &TreePath) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(path))
}

unsafe extern "C" fn move_cursor_trampoline<P>(this: *mut ffi::GtkIconView, step: ffi::GtkMovementStep, count: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P, MovementStep, i32) -> bool + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked(), from_glib(step), count).to_glib()
}

unsafe extern "C" fn select_all_trampoline<P>(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn select_cursor_item_trampoline<P>(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn selection_changed_trampoline<P>(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn toggle_cursor_item_trampoline<P>(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn unselect_all_trampoline<P>(this: *mut ffi::GtkIconView, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_8", feature = "dox"))]
unsafe extern "C" fn notify_activate_on_single_click_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_area_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_column_spacing_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_columns_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_item_orientation_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_item_padding_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_item_width_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_margin_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_markup_column_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_model_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_column_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_reorderable_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_row_spacing_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_selection_mode_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_spacing_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_column_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tooltip_column_trampoline<P>(this: *mut ffi::GtkIconView, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IconView> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconView::from_glib_borrow(this).downcast_unchecked())
}
