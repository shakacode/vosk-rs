use std::path::Path;
use std::ffi::{CStr, CString};

use crate::ffi;
use crate::session::{VoskSession, VoskSessionConfig};

pub struct VoskModel {
    pub(crate) inner: ffi::Model,
}

impl VoskModel {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let root = unsafe { CString::from_vec_unchecked(root.as_ref().to_string_lossy().as_bytes().to_vec()) };

        Self {
            inner: unsafe { ffi::Model::new(root.as_c_str().as_ptr()) },
        }
    }

    #[inline]
    pub fn create_session(&self, cfg: VoskSessionConfig) -> VoskSession {
        VoskSession::new(&self.inner, cfg)
    }

    #[inline]
    pub fn feed(&self, sess: &mut VoskSession, data: &[i16]) -> bool {
        unsafe { ffi::KaldiRecognizer_AcceptWaveform1(&mut sess.inner, data.as_ptr(), data.len() as _) }
    }

    #[inline]
    pub fn get_result(&self, sess: &mut VoskSession) -> String {
        let cstr = unsafe { CStr::from_ptr(ffi::KaldiRecognizer_Result(&mut sess.inner)) };

        cstr.to_string_lossy().to_string()
    }

    #[inline]
    pub fn get_partial_result(&self, sess: &mut VoskSession) -> String {
        let cstr = unsafe { CStr::from_ptr(ffi::KaldiRecognizer_PartialResult(&mut sess.inner)) };

        cstr.to_string_lossy().to_string()
    }

    #[inline]
    pub fn get_final_result(&self, sess: &mut VoskSession) -> String {
        let cstr = unsafe { CStr::from_ptr(ffi::KaldiRecognizer_FinalResult(&mut sess.inner)) };

        cstr.to_string_lossy().to_string()
    }
}

impl Drop for VoskModel {
    fn drop(&mut self) {
        unsafe { self.inner.destruct() }
    }
}