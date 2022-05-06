//!
//! Crypto Backend Wrappings
//!

// TODO do proper selection for the backend depending on env var set for xmlsec crypto

// mod nss;
// pub use nss::XmlSecSignatureMethod;

// mod gcrypt;
// pub use gcrypt::XmlSecSignatureMethod;

// mod gnutls;
// pub use gnutls::XmlSecSignatureMethod;

#[cfg(xmlsec_static)]
mod openssl;
#[cfg(xmlsec_static)]
use openssl as backend;

#[cfg(xmlsec_dynamic)]
mod dynamic;
#[cfg(xmlsec_dynamic)]
use dynamic as backend;

pub use backend::XmlSecSignatureMethod;
