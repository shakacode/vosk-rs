use crate::ffi;
use std::path::Path;
use std::ffi::CString;

pub struct SpeakerModel {
    pub(crate) inner: *mut ffi::VoskSpkModel
}

impl SpeakerModel {
    pub fn new(root: &Path) -> Self {
        let root = unsafe { CString::from_vec_unchecked(root.to_string_lossy().as_bytes().to_vec()) };

        Self {
            inner: unsafe { ffi::vosk_spk_model_new(root.as_c_str().as_ptr()) }
        }
    }
}

impl Drop for SpeakerModel {
    fn drop(&mut self) {
        unsafe { ffi::vosk_spk_model_free(self.inner) }
    }
}