// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Object;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Table(Interface<ffi::AtkTable>);

    match fn {
        get_type => || ffi::atk_table_get_type(),
    }
}

pub const NONE_TABLE: Option<&Table> = None;

pub trait TableExt: 'static {
    fn add_column_selection(&self, column: i32) -> bool;

    fn add_row_selection(&self, row: i32) -> bool;

    fn get_caption(&self) -> Option<Object>;

    fn get_column_at_index(&self, index_: i32) -> i32;

    fn get_column_description(&self, column: i32) -> Option<GString>;

    fn get_column_extent_at(&self, row: i32, column: i32) -> i32;

    fn get_column_header(&self, column: i32) -> Option<Object>;

    fn get_index_at(&self, row: i32, column: i32) -> i32;

    fn get_n_columns(&self) -> i32;

    fn get_n_rows(&self) -> i32;

    fn get_row_at_index(&self, index_: i32) -> i32;

    fn get_row_description(&self, row: i32) -> Option<GString>;

    fn get_row_extent_at(&self, row: i32, column: i32) -> i32;

    fn get_row_header(&self, row: i32) -> Option<Object>;

    fn get_summary(&self) -> Option<Object>;

    fn is_column_selected(&self, column: i32) -> bool;

    fn is_row_selected(&self, row: i32) -> bool;

    fn is_selected(&self, row: i32, column: i32) -> bool;

    fn ref_at(&self, row: i32, column: i32) -> Option<Object>;

    fn remove_column_selection(&self, column: i32) -> bool;

    fn remove_row_selection(&self, row: i32) -> bool;

    fn set_caption<P: IsA<Object>>(&self, caption: &P);

    fn set_column_description(&self, column: i32, description: &str);

    fn set_column_header<P: IsA<Object>>(&self, column: i32, header: &P);

    fn set_row_description(&self, row: i32, description: &str);

    fn set_row_header<P: IsA<Object>>(&self, row: i32, header: &P);

    fn set_summary<P: IsA<Object>>(&self, accessible: &P);

    fn connect_column_deleted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_column_inserted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_column_reordered<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_model_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_deleted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_inserted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_reordered<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Table>> TableExt for O {
    fn add_column_selection(&self, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_add_column_selection(self.as_ref().to_glib_none().0, column))
        }
    }

    fn add_row_selection(&self, row: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_add_row_selection(self.as_ref().to_glib_none().0, row))
        }
    }

    fn get_caption(&self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_table_get_caption(self.as_ref().to_glib_none().0))
        }
    }

    fn get_column_at_index(&self, index_: i32) -> i32 {
        unsafe {
            ffi::atk_table_get_column_at_index(self.as_ref().to_glib_none().0, index_)
        }
    }

    fn get_column_description(&self, column: i32) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::atk_table_get_column_description(self.as_ref().to_glib_none().0, column))
        }
    }

    fn get_column_extent_at(&self, row: i32, column: i32) -> i32 {
        unsafe {
            ffi::atk_table_get_column_extent_at(self.as_ref().to_glib_none().0, row, column)
        }
    }

    fn get_column_header(&self, column: i32) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_table_get_column_header(self.as_ref().to_glib_none().0, column))
        }
    }

    fn get_index_at(&self, row: i32, column: i32) -> i32 {
        unsafe {
            ffi::atk_table_get_index_at(self.as_ref().to_glib_none().0, row, column)
        }
    }

    fn get_n_columns(&self) -> i32 {
        unsafe {
            ffi::atk_table_get_n_columns(self.as_ref().to_glib_none().0)
        }
    }

    fn get_n_rows(&self) -> i32 {
        unsafe {
            ffi::atk_table_get_n_rows(self.as_ref().to_glib_none().0)
        }
    }

    fn get_row_at_index(&self, index_: i32) -> i32 {
        unsafe {
            ffi::atk_table_get_row_at_index(self.as_ref().to_glib_none().0, index_)
        }
    }

    fn get_row_description(&self, row: i32) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::atk_table_get_row_description(self.as_ref().to_glib_none().0, row))
        }
    }

    fn get_row_extent_at(&self, row: i32, column: i32) -> i32 {
        unsafe {
            ffi::atk_table_get_row_extent_at(self.as_ref().to_glib_none().0, row, column)
        }
    }

    fn get_row_header(&self, row: i32) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_table_get_row_header(self.as_ref().to_glib_none().0, row))
        }
    }

    fn get_summary(&self) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_table_get_summary(self.as_ref().to_glib_none().0))
        }
    }

    fn is_column_selected(&self, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_is_column_selected(self.as_ref().to_glib_none().0, column))
        }
    }

    fn is_row_selected(&self, row: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_is_row_selected(self.as_ref().to_glib_none().0, row))
        }
    }

    fn is_selected(&self, row: i32, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_is_selected(self.as_ref().to_glib_none().0, row, column))
        }
    }

    fn ref_at(&self, row: i32, column: i32) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_table_ref_at(self.as_ref().to_glib_none().0, row, column))
        }
    }

    fn remove_column_selection(&self, column: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_remove_column_selection(self.as_ref().to_glib_none().0, column))
        }
    }

    fn remove_row_selection(&self, row: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_table_remove_row_selection(self.as_ref().to_glib_none().0, row))
        }
    }

    fn set_caption<P: IsA<Object>>(&self, caption: &P) {
        unsafe {
            ffi::atk_table_set_caption(self.as_ref().to_glib_none().0, caption.as_ref().to_glib_none().0);
        }
    }

    fn set_column_description(&self, column: i32, description: &str) {
        unsafe {
            ffi::atk_table_set_column_description(self.as_ref().to_glib_none().0, column, description.to_glib_none().0);
        }
    }

    fn set_column_header<P: IsA<Object>>(&self, column: i32, header: &P) {
        unsafe {
            ffi::atk_table_set_column_header(self.as_ref().to_glib_none().0, column, header.as_ref().to_glib_none().0);
        }
    }

    fn set_row_description(&self, row: i32, description: &str) {
        unsafe {
            ffi::atk_table_set_row_description(self.as_ref().to_glib_none().0, row, description.to_glib_none().0);
        }
    }

    fn set_row_header<P: IsA<Object>>(&self, row: i32, header: &P) {
        unsafe {
            ffi::atk_table_set_row_header(self.as_ref().to_glib_none().0, row, header.as_ref().to_glib_none().0);
        }
    }

    fn set_summary<P: IsA<Object>>(&self, accessible: &P) {
        unsafe {
            ffi::atk_table_set_summary(self.as_ref().to_glib_none().0, accessible.as_ref().to_glib_none().0);
        }
    }

    fn connect_column_deleted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"column-deleted\0".as_ptr() as *const _,
                Some(transmute(column_deleted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_column_inserted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"column-inserted\0".as_ptr() as *const _,
                Some(transmute(column_inserted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_column_reordered<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"column-reordered\0".as_ptr() as *const _,
                Some(transmute(column_reordered_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_model_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"model-changed\0".as_ptr() as *const _,
                Some(transmute(model_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_row_deleted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-deleted\0".as_ptr() as *const _,
                Some(transmute(row_deleted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_row_inserted<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-inserted\0".as_ptr() as *const _,
                Some(transmute(row_inserted_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_row_reordered<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-reordered\0".as_ptr() as *const _,
                Some(transmute(row_reordered_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn column_deleted_trampoline<P, F: Fn(&P, i32, i32) + 'static>(this: *mut ffi::AtkTable, arg1: libc::c_int, arg2: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Table> {
    let f: &F = transmute(f);
    f(&Table::from_glib_borrow(this).unsafe_cast(), arg1, arg2)
}

unsafe extern "C" fn column_inserted_trampoline<P, F: Fn(&P, i32, i32) + 'static>(this: *mut ffi::AtkTable, arg1: libc::c_int, arg2: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Table> {
    let f: &F = transmute(f);
    f(&Table::from_glib_borrow(this).unsafe_cast(), arg1, arg2)
}

unsafe extern "C" fn column_reordered_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::AtkTable, f: glib_ffi::gpointer)
where P: IsA<Table> {
    let f: &F = transmute(f);
    f(&Table::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn model_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::AtkTable, f: glib_ffi::gpointer)
where P: IsA<Table> {
    let f: &F = transmute(f);
    f(&Table::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn row_deleted_trampoline<P, F: Fn(&P, i32, i32) + 'static>(this: *mut ffi::AtkTable, arg1: libc::c_int, arg2: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Table> {
    let f: &F = transmute(f);
    f(&Table::from_glib_borrow(this).unsafe_cast(), arg1, arg2)
}

unsafe extern "C" fn row_inserted_trampoline<P, F: Fn(&P, i32, i32) + 'static>(this: *mut ffi::AtkTable, arg1: libc::c_int, arg2: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Table> {
    let f: &F = transmute(f);
    f(&Table::from_glib_borrow(this).unsafe_cast(), arg1, arg2)
}

unsafe extern "C" fn row_reordered_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::AtkTable, f: glib_ffi::gpointer)
where P: IsA<Table> {
    let f: &F = transmute(f);
    f(&Table::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table")
    }
}
