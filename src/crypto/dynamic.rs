//!
//! Wrapping for dynamic Crypto Backend Implementation for XmlSec Crypto Interface
//!
use crate::bindings;


/// Supported digesting and signing methods as specified by the XML standard.
#[allow(missing_docs)]
pub enum XmlSecSignatureMethod
{
    Aes128Cbc,
    Aes192Cbc,
    Aes256Cbc,
    // Aes128Gcm,
    // Aes192Gcm,
    // Aes256Gcm,
    KWAes128,
    KWAes192,
    KWAes256,
    Des3Cbc,
    KWDes3,
    DsaSha1,
    DsaSha256,
    EcdsaSha1,
    EcdsaSha224,
    EcdsaSha256,
    EcdsaSha384,
    EcdsaSha512,
    HmacMd5,
    HmacRipemd160,
    HmacSha1,
    HmacSha224,
    HmacSha256,
    HmacSha384,
    HmacSha512,
    Md5,
    Ripemd160,
    RsaMd5,
    RsaRipemd160,
    RsaSha1,
    RsaSha224,
    RsaSha256,
    RsaSha384,
    RsaSha512,
    RsaPkcs1,
    RsaOaep,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}


impl XmlSecSignatureMethod
{
    /// Returns the resource pointer for the corresponding digesting/signing resource
    pub fn to_method(&self) -> bindings::xmlSecTransformId
    {
        match self
        {
            Self::Aes128Cbc     => unsafe { bindings::xmlSecTransformAes128CbcGetKlass() },
            Self::Aes192Cbc     => unsafe { bindings::xmlSecTransformAes192CbcGetKlass() },
            Self::Aes256Cbc     => unsafe { bindings::xmlSecTransformAes256CbcGetKlass() },
            // Self::Aes128Gcm     => unsafe { bindings::xmlSecTransformAes128GcmGetKlass() },
            // Self::Aes192Gcm     => unsafe { bindings::xmlSecTransformAes192GcmGetKlass() },
            // Self::Aes256Gcm     => unsafe { bindings::xmlSecTransformAes256GcmGetKlass() },
            Self::KWAes128      => unsafe { bindings::xmlSecTransformKWAes128GetKlass() },
            Self::KWAes192      => unsafe { bindings::xmlSecTransformKWAes192GetKlass() },
            Self::KWAes256      => unsafe { bindings::xmlSecTransformKWAes256GetKlass() },
            Self::Des3Cbc       => unsafe { bindings::xmlSecTransformDes3CbcGetKlass() },
            Self::KWDes3        => unsafe { bindings::xmlSecTransformKWDes3GetKlass() },
            Self::DsaSha1       => unsafe { bindings::xmlSecTransformDsaSha1GetKlass() },
            Self::DsaSha256     => unsafe { bindings::xmlSecTransformDsaSha256GetKlass() },
            Self::EcdsaSha1     => unsafe { bindings::xmlSecTransformEcdsaSha1GetKlass() },
            Self::EcdsaSha224   => unsafe { bindings::xmlSecTransformEcdsaSha224GetKlass() },
            Self::EcdsaSha256   => unsafe { bindings::xmlSecTransformEcdsaSha256GetKlass() },
            Self::EcdsaSha384   => unsafe { bindings::xmlSecTransformEcdsaSha384GetKlass() },
            Self::EcdsaSha512   => unsafe { bindings::xmlSecTransformEcdsaSha512GetKlass() },
            Self::HmacMd5       => unsafe { bindings::xmlSecTransformHmacMd5GetKlass() },
            Self::HmacRipemd160 => unsafe { bindings::xmlSecTransformHmacRipemd160GetKlass() },
            Self::HmacSha1      => unsafe { bindings::xmlSecTransformHmacSha1GetKlass() },
            Self::HmacSha224    => unsafe { bindings::xmlSecTransformHmacSha224GetKlass() },
            Self::HmacSha256    => unsafe { bindings::xmlSecTransformHmacSha256GetKlass() },
            Self::HmacSha384    => unsafe { bindings::xmlSecTransformHmacSha384GetKlass() },
            Self::HmacSha512    => unsafe { bindings::xmlSecTransformHmacSha512GetKlass() },
            Self::Md5           => unsafe { bindings::xmlSecTransformMd5GetKlass() },
            Self::Ripemd160     => unsafe { bindings::xmlSecTransformRipemd160GetKlass() },
            Self::RsaMd5        => unsafe { bindings::xmlSecTransformRsaMd5GetKlass() },
            Self::RsaRipemd160  => unsafe { bindings::xmlSecTransformRsaRipemd160GetKlass() },
            Self::RsaSha1       => unsafe { bindings::xmlSecTransformRsaSha1GetKlass() },
            Self::RsaSha224     => unsafe { bindings::xmlSecTransformRsaSha224GetKlass() },
            Self::RsaSha256     => unsafe { bindings::xmlSecTransformRsaSha256GetKlass() },
            Self::RsaSha384     => unsafe { bindings::xmlSecTransformRsaSha384GetKlass() },
            Self::RsaSha512     => unsafe { bindings::xmlSecTransformRsaSha512GetKlass() },
            Self::RsaPkcs1      => unsafe { bindings::xmlSecTransformRsaPkcs1GetKlass() },
            Self::RsaOaep       => unsafe { bindings::xmlSecTransformRsaOaepGetKlass() },
            Self::Sha1          => unsafe { bindings::xmlSecTransformSha1GetKlass() },
            Self::Sha224        => unsafe { bindings::xmlSecTransformSha224GetKlass() },
            Self::Sha256        => unsafe { bindings::xmlSecTransformSha256GetKlass() },
            Self::Sha384        => unsafe { bindings::xmlSecTransformSha384GetKlass() },
            Self::Sha512        => unsafe { bindings::xmlSecTransformSha512GetKlass() },
        }
    }
}
