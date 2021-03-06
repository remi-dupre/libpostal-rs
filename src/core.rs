use std::ffi::CString;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, Mutex};

use sys;
use traits::{ToC, ToRust};
use utils::ptr_to_rust;

use AddressParser;
use LanguageClassifier;
use NormalizeOptions;

static INIT_CORE: once_cell::sync::Lazy<Arc<Mutex<(usize, Option<CString>)>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new((0, None))));

pub struct Core {
    inner: PhantomData<u32>,
}

impl Drop for Core {
    fn drop(&mut self) {
        if let Ok(ref mut x) = INIT_CORE.lock() {
            if (**x).0 == 1 {
                unsafe { sys::libpostal_teardown() }
                (**x).1 = None;
            }
            (**x).0 -= 1;
        }
    }
}

impl Core {
    /// Initialize libpostal.
    ///
    /// NOTE: If you already initialized it, it'll mostly do nothing. However, if you already
    /// initialized it with a `datadir`, it'll be overwritten since `libpostal` handles it globally.
    pub fn setup() -> Option<Core> {
        if let Ok(ref mut x) = INIT_CORE.lock() {
            if unsafe { sys::libpostal_setup() }.to_rust() {
                (**x).0 += 1;
                (**x).1 = None;
                return Some(Core { inner: PhantomData });
            }
        }
        None
    }

    /// Initialize libpostal with a given `datadir`.
    ///
    /// NOTE: If you already initialized it, it'll mostly do nothing. However, the `datadir` will be
    /// overwritten with the new one since `libpostal` handles it globally.
    pub fn setup_datadir<P: AsRef<Path>>(datadir: P) -> Option<Core> {
        if let Ok(ref mut x) = INIT_CORE.lock() {
            let datadir = datadir.as_ref();
            let c = datadir.to_c();
            if unsafe { sys::libpostal_setup_datadir(c.as_ptr()) }.to_rust() {
                (**x).0 += 1;
                (**x).1 = Some(c);
                return Some(Core { inner: PhantomData });
            }
        }
        None
    }

    pub fn setup_parser<'a>(&'a self) -> Option<AddressParser<'a>> {
        AddressParser::new(self)
    }

    pub fn setup_parser_datadir<'a, P: AsRef<Path>>(
        &'a self,
        datadir: P,
    ) -> Option<AddressParser<'a>> {
        AddressParser::new_datadir(self, datadir)
    }

    pub fn setup_language_classifier<'a>(&'a self) -> Option<LanguageClassifier<'a>> {
        LanguageClassifier::new(self)
    }

    pub fn setup_language_classifier_datadir<'a, P: AsRef<Path>>(
        &'a self,
        datadir: P,
    ) -> Option<LanguageClassifier<'a>> {
        LanguageClassifier::new_datadir(self, datadir)
    }

    pub fn get_default_options(&self) -> NormalizeOptions {
        unsafe { sys::libpostal_get_default_options() }.to_rust()
    }

    pub fn expand_address(&self, input: &str, options: NormalizeOptions) -> Vec<String> {
        let input = input.to_c();
        let (_, options) = options.to_c();
        let mut size = 0;

        let ptr = unsafe { sys::libpostal_expand_address(input.as_ptr(), options, &mut size) };
        let ret = ptr_to_rust(ptr, size);
        // Apparently we have to free memory of a char** using THIS function so let's go...
        unsafe {
            sys::libpostal_expansion_array_destroy(ptr, size);
        }
        ret
    }

    pub fn expand_address_root(&self, input: &str, options: NormalizeOptions) -> Vec<String> {
        let input = input.to_c();
        let (_, options) = options.to_c();
        let mut size = 0;

        let ptr = unsafe { sys::libpostal_expand_address_root(input.as_ptr(), options, &mut size) };
        let ret = ptr_to_rust(ptr, size);
        // Apparently we have to free memory of a char** using THIS function so let's go...
        unsafe {
            sys::libpostal_expansion_array_destroy(ptr, size);
        }
        ret
    }
}
