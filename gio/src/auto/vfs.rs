// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
use std::boxed::Box as Box_;
use std::fmt;
use File;

glib_wrapper! {
    pub struct Vfs(Object<gio_sys::GVfs, gio_sys::GVfsClass>);

    match fn {
        get_type => || gio_sys::g_vfs_get_type(),
    }
}

impl Vfs {
    pub fn get_default() -> Option<Vfs> {
        unsafe { from_glib_none(gio_sys::g_vfs_get_default()) }
    }

    pub fn get_local() -> Option<Vfs> {
        unsafe { from_glib_none(gio_sys::g_vfs_get_local()) }
    }
}

unsafe impl Send for Vfs {}
unsafe impl Sync for Vfs {}

pub const NONE_VFS: Option<&Vfs> = None;

pub trait VfsExt: 'static {
    fn get_file_for_path(&self, path: &str) -> Option<File>;

    fn get_file_for_uri(&self, uri: &str) -> Option<File>;

    fn get_supported_uri_schemes(&self) -> Vec<GString>;

    fn is_active(&self) -> bool;

    fn parse_name(&self, parse_name: &str) -> Option<File>;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn register_uri_scheme(
        &self,
        scheme: &str,
        uri_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
        parse_name_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
    ) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn unregister_uri_scheme(&self, scheme: &str) -> bool;
}

impl<O: IsA<Vfs>> VfsExt for O {
    fn get_file_for_path(&self, path: &str) -> Option<File> {
        unsafe {
            from_glib_full(gio_sys::g_vfs_get_file_for_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    fn get_file_for_uri(&self, uri: &str) -> Option<File> {
        unsafe {
            from_glib_full(gio_sys::g_vfs_get_file_for_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn get_supported_uri_schemes(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gio_sys::g_vfs_get_supported_uri_schemes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe { from_glib(gio_sys::g_vfs_is_active(self.as_ref().to_glib_none().0)) }
    }

    fn parse_name(&self, parse_name: &str) -> Option<File> {
        unsafe {
            from_glib_full(gio_sys::g_vfs_parse_name(
                self.as_ref().to_glib_none().0,
                parse_name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn register_uri_scheme(
        &self,
        scheme: &str,
        uri_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
        parse_name_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
    ) -> bool {
        let uri_func_data: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            Box_::new(uri_func);
        unsafe extern "C" fn uri_func_func(
            vfs: *mut gio_sys::GVfs,
            identifier: *const libc::c_char,
            user_data: glib_sys::gpointer,
        ) -> *mut gio_sys::GFile {
            let vfs = from_glib_borrow(vfs);
            let identifier: Borrowed<GString> = from_glib_borrow(identifier);
            let callback: &Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&vfs, identifier.as_str())
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib_full()
        }
        let uri_func = if uri_func_data.is_some() {
            Some(uri_func_func as _)
        } else {
            None
        };
        let parse_name_func_data: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            Box_::new(parse_name_func);
        unsafe extern "C" fn parse_name_func_func(
            vfs: *mut gio_sys::GVfs,
            identifier: *const libc::c_char,
            user_data: glib_sys::gpointer,
        ) -> *mut gio_sys::GFile {
            let vfs = from_glib_borrow(vfs);
            let identifier: Borrowed<GString> = from_glib_borrow(identifier);
            let callback: &Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&vfs, identifier.as_str())
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib_full()
        }
        let parse_name_func = if parse_name_func_data.is_some() {
            Some(parse_name_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn uri_destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(uri_destroy_func as _);
        unsafe extern "C" fn parse_name_destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call7 = Some(parse_name_destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            uri_func_data;
        let super_callback1: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            parse_name_func_data;
        unsafe {
            from_glib(gio_sys::g_vfs_register_uri_scheme(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
                uri_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
                parse_name_func,
                Box_::into_raw(super_callback1) as *mut _,
                destroy_call7,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn unregister_uri_scheme(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_vfs_unregister_uri_scheme(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Vfs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vfs")
    }
}
