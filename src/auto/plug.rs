// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use Component;
use Object;

glib_wrapper! {
    pub struct Plug(Object<atk_sys::AtkPlug, atk_sys::AtkPlugClass, PlugClass>) @extends Object, @implements Component;

    match fn {
        get_type => || atk_sys::atk_plug_get_type(),
    }
}

impl Plug {
    pub fn new() -> Plug {
        assert_initialized_main_thread!();
        unsafe { Object::from_glib_full(atk_sys::atk_plug_new()).unsafe_cast() }
    }
}

impl Default for Plug {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PLUG: Option<&Plug> = None;

pub trait AtkPlugExt: 'static {
    fn get_id(&self) -> Option<GString>;
}

impl<O: IsA<Plug>> AtkPlugExt for O {
    fn get_id(&self) -> Option<GString> {
        unsafe { from_glib_full(atk_sys::atk_plug_get_id(self.as_ref().to_glib_none().0)) }
    }
}

impl fmt::Display for Plug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plug")
    }
}
