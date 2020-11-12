// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use std::fmt;
use AxisFlags;
use DeviceToolType;

glib_wrapper! {
    pub struct DeviceTool(Object<gdk_sys::GdkDeviceTool>);

    match fn {
        get_type => || gdk_sys::gdk_device_tool_get_type(),
    }
}

impl DeviceTool {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_hardware_id(&self) -> u64 {
        unsafe { gdk_sys::gdk_device_tool_get_hardware_id(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_serial(&self) -> u64 {
        unsafe { gdk_sys::gdk_device_tool_get_serial(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_tool_type(&self) -> DeviceToolType {
        unsafe {
            from_glib(gdk_sys::gdk_device_tool_get_tool_type(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_property_axes(&self) -> AxisFlags {
        unsafe {
            let mut value = Value::from_type(<AxisFlags as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"axes\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `axes` getter")
                .unwrap()
        }
    }

    pub fn get_property_hardware_id(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"hardware-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `hardware-id` getter")
                .unwrap()
        }
    }

    pub fn get_property_serial(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"serial\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `serial` getter")
                .unwrap()
        }
    }

    pub fn get_property_tool_type(&self) -> DeviceToolType {
        unsafe {
            let mut value = Value::from_type(<DeviceToolType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"tool-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tool-type` getter")
                .unwrap()
        }
    }
}

impl fmt::Display for DeviceTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceTool")
    }
}
