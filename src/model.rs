use std::path::Path;
use std::ffi::{CStr, CString};

use crate::ffi;
use crate::session::{VoskSession, VoskSessionConfig};

pub struct VoskModel {
    pub(crate) inner: *mut ffi::VoskModel,
}

impl VoskModel {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let root = unsafe { CString::from_vec_unchecked(root.as_ref().to_string_lossy().as_bytes().to_vec()) };

        Self {
            inner: unsafe { ffi::vosk_model_new(root.as_c_str().as_ptr()) },
        }
    }

    #[inline]
    pub fn create_session(&self, cfg: VoskSessionConfig) -> VoskSession {
        VoskSession::new(self.inner, cfg)
    }

    #[inline]
    pub fn feed(&self, sess: &mut VoskSession, data: &[i16]) -> bool {
        unsafe { ffi::vosk_recognizer_accept_waveform_s(sess.inner, data.as_ptr(), data.len() as _) == 1 }
    }

    #[inline]
    pub fn get_result(&self, sess: &mut VoskSession) -> crate::TranscriptionResult {
        let cstr = unsafe { CStr::from_ptr(ffi::vosk_recognizer_result(sess.inner)) };

        serde_json::from_str(cstr.to_str().unwrap()).unwrap()
    }

    #[inline]
    pub fn get_partial_result(&self, sess: &mut VoskSession) -> crate::TranscriptionPartialResult {
        let cstr = unsafe { CStr::from_ptr(ffi::vosk_recognizer_partial_result(sess.inner)) };

        serde_json::from_str(cstr.to_str().unwrap()).unwrap()
    }

    #[inline]
    pub fn get_final_result(&self, sess: &mut VoskSession) -> crate::TranscriptionResult {
        let cstr = unsafe { CStr::from_ptr(ffi::vosk_recognizer_final_result(sess.inner)) };

        serde_json::from_str(cstr.to_str().unwrap()).unwrap()
    }
}

impl Drop for VoskModel {
    fn drop(&mut self) {
        unsafe { ffi::vosk_model_free(self.inner) }
    }
}