// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Vec2;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    pub struct Point(BoxedInline<ffi::graphene_point_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_point_get_type(), ptr as *mut _) as *mut ffi::graphene_point_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_point_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_point_get_type(),
    }
}

impl Point {
    #[doc(alias = "graphene_point_distance")]
    pub fn distance(&self, b: &Point) -> (f32, f32, f32) {
        unsafe {
            let mut d_x = mem::MaybeUninit::uninit();
            let mut d_y = mem::MaybeUninit::uninit();
            let ret = ffi::graphene_point_distance(
                self.to_glib_none().0,
                b.to_glib_none().0,
                d_x.as_mut_ptr(),
                d_y.as_mut_ptr(),
            );
            (ret, d_x.assume_init(), d_y.assume_init())
        }
    }

    #[cfg(feature = "v1_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_12")))]
    #[doc(alias = "graphene_point_distance_squared")]
    pub fn distance_squared(&self, b: &Point) -> f32 {
        unsafe { ffi::graphene_point_distance_squared(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_point_equal")]
    fn equal(&self, b: &Point) -> bool {
        unsafe { ffi::graphene_point_equal(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_point_interpolate")]
    #[must_use]
    pub fn interpolate(&self, b: &Point, factor: f64) -> Point {
        unsafe {
            let mut res = Point::uninitialized();
            ffi::graphene_point_interpolate(
                self.to_glib_none().0,
                b.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_point_near")]
    pub fn near(&self, b: &Point, epsilon: f32) -> bool {
        unsafe { ffi::graphene_point_near(self.to_glib_none().0, b.to_glib_none().0, epsilon) }
    }

    #[doc(alias = "graphene_point_to_vec2")]
    pub fn to_vec2(&self) -> Vec2 {
        unsafe {
            let mut v = Vec2::uninitialized();
            ffi::graphene_point_to_vec2(self.to_glib_none().0, v.to_glib_none_mut().0);
            v
        }
    }

    #[doc(alias = "graphene_point_zero")]
    pub fn zero() -> Point {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_point_zero()) }
    }
}

impl PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Point {}
