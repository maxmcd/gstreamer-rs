// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RTSPThreadPool(Object<ffi::GstRTSPThreadPool, ffi::GstRTSPThreadPoolClass>);

    match fn {
        get_type => || ffi::gst_rtsp_thread_pool_get_type(),
    }
}

impl RTSPThreadPool {
    pub fn new() -> RTSPThreadPool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_thread_pool_new())
        }
    }

    pub fn cleanup() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_rtsp_thread_pool_cleanup();
        }
    }
}

impl Default for RTSPThreadPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPThreadPool {}
unsafe impl Sync for RTSPThreadPool {}

pub trait RTSPThreadPoolExt {
    fn get_max_threads(&self) -> i32;

    //fn get_thread(&self, type_: RTSPThreadType, ctx: &RTSPContext) -> /*Ignored*/Option<RTSPThread>;

    fn set_max_threads(&self, max_threads: i32);

    fn connect_property_max_threads_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPThreadPool> + IsA<glib::object::Object>> RTSPThreadPoolExt for O {
    fn get_max_threads(&self) -> i32 {
        unsafe {
            ffi::gst_rtsp_thread_pool_get_max_threads(self.to_glib_none().0)
        }
    }

    //fn get_thread(&self, type_: RTSPThreadType, ctx: &RTSPContext) -> /*Ignored*/Option<RTSPThread> {
    //    unsafe { TODO: call ffi::gst_rtsp_thread_pool_get_thread() }
    //}

    fn set_max_threads(&self, max_threads: i32) {
        unsafe {
            ffi::gst_rtsp_thread_pool_set_max_threads(self.to_glib_none().0, max_threads);
        }
    }

    fn connect_property_max_threads_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-threads",
                transmute(notify_max_threads_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_max_threads_trampoline<P>(this: *mut ffi::GstRTSPThreadPool, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPThreadPool> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPThreadPool::from_glib_borrow(this).downcast_unchecked())
}
