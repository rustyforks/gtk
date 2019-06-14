// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_sys;
use std::ffi::CStr;
use PadActionType;

#[derive(Debug, Clone)]
pub struct PadActionEntry {
    pub(crate) type_: PadActionType,
    pub(crate) index: i32,
    pub(crate) mode: i32,
    pub(crate) label: String,
    pub(crate) action_name: String,
}

impl PadActionEntry {
    pub fn new(
        type_: PadActionType,
        index: i32,
        mode: i32,
        label: &str,
        action_name: &str,
    ) -> PadActionEntry {
        assert_initialized_main_thread!();
        PadActionEntry {
            type_,
            index,
            mode,
            label: label.to_owned(),
            action_name: action_name.to_owned(),
        }
    }

    pub fn get_type(&self) -> PadActionType {
        self.type_
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn get_mode(&self) -> i32 {
        self.mode
    }

    pub fn get_label(&self) -> Option<&str> {
        Some(&self.label)
    }

    pub fn get_action_name(&self) -> Option<&str> {
        Some(&self.action_name)
    }
}
