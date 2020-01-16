use std::marker::PhantomData;
use std::path::Path;

use AddressParser;
use LanguageClassifier;
use NormalizeOptions;
use sys;
use traits::{ToC, ToRust};

pub struct Core {
    inner: PhantomData<u32>,
}

impl Core {
    pub fn setup() -> Option<Core> {
        if unsafe { sys::libpostal_setup() }.to_rust() {
            Some(Core { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn setup_datadir<P: AsRef<Path>>(datadir: P) -> Option<Core> {
        let datadir = datadir.as_ref();
        let c = datadir.to_c();
        if unsafe { sys::libpostal_setup_datadir(c.as_ptr()) }.to_rust() {
            Some(Core { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn setup_parser(&self) -> Option<AddressParser> {
        if unsafe { sys::libpostal_setup_parser() }.to_rust() {
            Some(AddressParser { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn setup_parser_datadir<P: AsRef<Path>>(&self, datadir: P) -> Option<AddressParser> {
        let datadir = datadir.as_ref();
        let c = datadir.to_c();
        if unsafe { sys::libpostal_setup_parser_datadir(c.as_ptr()) }.to_rust() {
            Some(AddressParser { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn setup_language_classifier(&self) -> Option<LanguageClassifier> {
        if unsafe { sys::libpostal_setup_language_classifier() }.to_rust() {
            Some(LanguageClassifier { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn setup_language_classifier_datadir<P: AsRef<Path>>(
        &self,
        datadir: P,
    ) -> Option<LanguageClassifier> {
        let datadir = datadir.as_ref();
        let c = datadir.to_c();
        if unsafe { sys::libpostal_setup_language_classifier_datadir(c.as_ptr()) }.to_rust() {
            Some(LanguageClassifier { inner: PhantomData })
        } else {
            None
        }
    }

    pub fn get_default_options(&self) -> NormalizeOptions {
        unsafe { sys::libpostal_get_default_options() }.to_rust()
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe { sys::libpostal_teardown() }
    }
}
