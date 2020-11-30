// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CssSection;
use crate::StyleProvider;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct CssProvider(Object<ffi::GtkCssProvider, ffi::GtkCssProviderClass>) @implements StyleProvider;

    match fn {
        get_type => || ffi::gtk_css_provider_get_type(),
    }
}

impl CssProvider {
    pub fn new() -> CssProvider {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_css_provider_new()) }
    }

    #[cfg_attr(feature = "v3_24", deprecated)]
    pub fn get_default() -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_css_provider_get_default()) }
    }

    pub fn get_named(name: &str, variant: Option<&str>) -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_named(
                name.to_glib_none().0,
                variant.to_glib_none().0,
            ))
        }
    }
}

impl Default for CssProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CssProvider {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&CssProviderExt::to_str(self))
    }
}

pub const NONE_CSS_PROVIDER: Option<&CssProvider> = None;

pub trait CssProviderExt: 'static {
    fn load_from_data(&self, data: &[u8]) -> Result<(), glib::Error>;

    fn load_from_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error>;

    fn load_from_path(&self, path: &str) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn load_from_resource(&self, resource_path: &str);

    fn to_str(&self) -> glib::GString;

    fn connect_parsing_error<F: Fn(&Self, &CssSection, &glib::Error) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<CssProvider>> CssProviderExt for O {
    fn load_from_data(&self, data: &[u8]) -> Result<(), glib::Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_data(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_from_file<P: IsA<gio::File>>(&self, file: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_file(
                self.as_ref().to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_from_path(&self, path: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn load_from_resource(&self, resource_path: &str) {
        unsafe {
            ffi::gtk_css_provider_load_from_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_css_provider_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_parsing_error<F: Fn(&Self, &CssSection, &glib::Error) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn parsing_error_trampoline<
            P,
            F: Fn(&P, &CssSection, &glib::Error) + 'static,
        >(
            this: *mut ffi::GtkCssProvider,
            section: *mut ffi::GtkCssSection,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CssProvider>,
        {
            let f: &F = &*(f as *const F);
            f(
                &CssProvider::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(section),
                &from_glib_borrow(error),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"parsing-error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    parsing_error_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CssProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CssProvider")
    }
}
