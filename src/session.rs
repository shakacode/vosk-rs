use crate::ffi;
use std::ffi::CString;
use std::path::{PathBuf};

pub struct VoskSessionConfigBuilder {
    spk_root: Option<CString>,
    grammar: Option<CString>,
    freq: f32,
}

impl VoskSessionConfigBuilder {
    fn new() -> Self {
        VoskSessionConfigBuilder {
            spk_root: None,
            grammar: None,
            freq: 16000.0,
        }
    }

    pub fn spk_root<P: Into<PathBuf>>(&mut self, root: P) -> &mut Self {
        self.spk_root = Some(unsafe { CString::from_vec_unchecked(root.into().to_string_lossy().as_bytes().to_vec()) });
        self
    }

    pub fn sampling_freq(&mut self, freq: f32) -> &mut Self {
        self.freq = freq;
        self
    }

    pub fn grammar<G: AsRef<str>>(&mut self, grammar: G) -> &mut Self {
        self.grammar = Some(CString::new(grammar.as_ref()).unwrap());
        self
    }

    pub fn finish(&mut self) -> VoskSessionConfig {
        VoskSessionConfig {
            spk_root: core::mem::take(&mut self.spk_root),
            grammar: core::mem::take(&mut self.grammar),
            freq: self.freq,
        }
    }
}

pub struct VoskSessionConfig {
    spk_root: Option<CString>,
    grammar: Option<CString>,
    freq: f32,
}

impl Default for VoskSessionConfig {
    fn default() -> Self {
        Self {
            spk_root: None,
            grammar: None,
            freq: 16000.0
        }
    }
}

impl VoskSessionConfig {
    pub fn builder() -> VoskSessionConfigBuilder {
        VoskSessionConfigBuilder::new()
    }

    #[inline]
    pub fn set_spk_root(&mut self, root: CString) {
        self.spk_root = Some(root);
    }

    #[inline]
    pub fn set_grammar(&mut self, grammar: CString) {
        self.grammar = Some(grammar);
    }

    #[inline]
    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq;
    }
}

pub struct VoskSession {
    pub(crate) inner: *mut ffi::VoskRecognizer
}

impl VoskSession {
    pub(crate) fn new(model: *const ffi::VoskModel, cfg: VoskSessionConfig) -> Self {
        if let Some(_cfg) = &cfg.spk_root {
            unimplemented!()
            // VoskSession {
            //     inner: ffi::KaldiRecognizer::new1(model as *mut ffi::Model,  cfg.freq)
            // }
        } else if let Some(_grammar) = &cfg.grammar {
            unimplemented!()
            // VoskSession {
            //     inner: unsafe { ffi::KaldiRecognizer::new2(model as *mut ffi::Model, cfg.freq, grammar.as_c_str().as_ptr()) }
            // }
        } else {
            VoskSession {
                inner: unsafe { ffi::vosk_recognizer_new(model as *mut ffi::VoskModel, cfg.freq) }
            }
        }
    }
}

impl Drop for VoskSession {
    fn drop(&mut self) {
        unsafe { ffi::vosk_recognizer_free(self.inner) }
    }
}
