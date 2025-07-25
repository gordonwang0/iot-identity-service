// Copyright (c) Microsoft. All rights reserved.

use std::sync::{LazyLock, Mutex, RwLock};

use aziot_keys_common::PreloadedKeyLocation;

static HOMEDIR_PATH: LazyLock<RwLock<Option<std::path::PathBuf>>> = LazyLock::new(Default::default);

static PKCS11_LIB_PATH: LazyLock<RwLock<Option<std::path::PathBuf>>> =
    LazyLock::new(Default::default);
static PKCS11_BASE_SLOT: LazyLock<RwLock<Option<pkcs11::Uri>>> = LazyLock::new(Default::default);

static PRELOADED_KEYS: LazyLock<RwLock<std::collections::BTreeMap<String, PreloadedKeyLocation>>> =
    LazyLock::new(Default::default);

static PKCS11_BASE_SLOT_SESSION: LazyLock<Mutex<Option<std::sync::Arc<pkcs11::Session>>>> =
    LazyLock::new(Default::default);

pub(crate) unsafe fn get_function_list(
    version: crate::AZIOT_KEYS_VERSION,
    pfunction_list: *mut *const crate::function_list::AZIOT_KEYS_FUNCTION_LIST,
) -> crate::AZIOT_KEYS_RC {
    // Ignore the error from `try_init`. The error indicates a global logger was already set,
    // which is because `get_function_list` is being called a second time. That's fine.
    let _ = logger::try_init();

    crate::r#catch(|| {
        static AZIOT_KEYS_FUNCTION_LIST_2_0_0_0:
            crate::function_list::v2_0_0_0::AZIOT_KEYS_FUNCTION_LIST_2_0_0_0 =
            crate::function_list::v2_0_0_0::AZIOT_KEYS_FUNCTION_LIST_2_0_0_0 {
                base: crate::function_list::AZIOT_KEYS_FUNCTION_LIST {
                    version: crate::function_list::v2_0_0_0::AZIOT_KEYS_VERSION_2_0_0_0,
                },

                set_parameter,
                create_key_pair_if_not_exists: crate::key_pair::create_key_pair_if_not_exists,
                load_key_pair: crate::key_pair::load_key_pair,
                get_key_pair_parameter: crate::key_pair::get_key_pair_parameter,
                create_key_if_not_exists: crate::key::create_key_if_not_exists,
                load_key: crate::key::load_key,
                import_key: crate::key::import_key,
                derive_key: crate::key::derive_key,
                sign,
                verify,
                encrypt,
                decrypt,
            };

        static AZIOT_KEYS_FUNCTION_LIST_2_1_0_0:
            crate::function_list::v2_1_0_0::AZIOT_KEYS_FUNCTION_LIST_2_1_0_0 =
            crate::function_list::v2_1_0_0::AZIOT_KEYS_FUNCTION_LIST_2_1_0_0 {
                base: crate::function_list::AZIOT_KEYS_FUNCTION_LIST {
                    version: crate::function_list::v2_1_0_0::AZIOT_KEYS_VERSION_2_1_0_0,
                },

                set_parameter,
                create_key_pair_if_not_exists: crate::key_pair::create_key_pair_if_not_exists,
                move_key_pair: crate::key_pair::move_key_pair,
                load_key_pair: crate::key_pair::load_key_pair,
                get_key_pair_parameter: crate::key_pair::get_key_pair_parameter,
                delete_key_pair: crate::key_pair::delete_key_pair,
                create_key_if_not_exists: crate::key::create_key_if_not_exists,
                load_key: crate::key::load_key,
                import_key: crate::key::import_key,
                delete_key: crate::key::delete_key,
                derive_key: crate::key::derive_key,
                sign,
                verify,
                encrypt,
                decrypt,
            };

        match version {
            crate::function_list::v2_0_0_0::AZIOT_KEYS_VERSION_2_0_0_0 => {
                let mut function_list_out = std::ptr::NonNull::new(pfunction_list)
                    .ok_or_else(|| err_invalid_parameter("pfunction_list", "expected non-NULL"))?;
                *function_list_out.as_mut() =
                    std::ptr::addr_of!(AZIOT_KEYS_FUNCTION_LIST_2_0_0_0).cast();
                Ok(())
            }

            crate::function_list::v2_1_0_0::AZIOT_KEYS_VERSION_2_1_0_0 => {
                let mut function_list_out = std::ptr::NonNull::new(pfunction_list)
                    .ok_or_else(|| err_invalid_parameter("pfunction_list", "expected non-NULL"))?;
                *function_list_out.as_mut() =
                    std::ptr::addr_of!(AZIOT_KEYS_FUNCTION_LIST_2_1_0_0).cast();
                Ok(())
            }

            _ => Err(err_invalid_parameter("version", "unsupported version")),
        }
    })
}

pub(crate) unsafe extern "C" fn set_parameter(
    name: *const std::os::raw::c_char,
    value: *const std::os::raw::c_char,
) -> crate::AZIOT_KEYS_RC {
    crate::r#catch(|| {
        let name = name
            .as_ref()
            .ok_or_else(|| err_invalid_parameter("name", "expected non-NULL"))?;
        let name = std::ffi::CStr::from_ptr(name);
        let name = name
            .to_str()
            .map_err(|err| err_invalid_parameter("name", err))?;

        match name {
            "homedir_path" => {
                let value = value
                    .as_ref()
                    .ok_or_else(|| err_invalid_parameter("value", "expected non-NULL"))?;
                let value = std::ffi::CStr::from_ptr(value);
                let value = value
                    .to_str()
                    .map_err(|err| err_invalid_parameter("value", err))?;
                let value: std::path::PathBuf = value.into();

                let mut guard = HOMEDIR_PATH.write().expect("fatal RwLock failure");
                *guard = Some(value);
            }

            "pkcs11_lib_path" => {
                let value = value
                    .as_ref()
                    .ok_or_else(|| err_invalid_parameter("value", "expected non-NULL"))?;
                let value = std::ffi::CStr::from_ptr(value);
                let value = value
                    .to_str()
                    .map_err(|err| err_invalid_parameter("value", err))?;
                let value: std::path::PathBuf = value.into();

                let mut guard = PKCS11_LIB_PATH.write().expect("fatal RwLock failure");
                *guard = Some(value);
            }

            "pkcs11_base_slot" => {
                let value = value
                    .as_ref()
                    .ok_or_else(|| err_invalid_parameter("value", "expected non-NULL"))?;
                let value = std::ffi::CStr::from_ptr(value);
                let value = value
                    .to_str()
                    .map_err(|err| err_invalid_parameter("value", err))?;
                let value = value
                    .parse()
                    .map_err(|err| err_invalid_parameter("value", err))?;

                let mut guard = PKCS11_BASE_SLOT.write().expect("fatal RwLock failure");
                *guard = Some(value);
            }

            name if name.starts_with("preloaded_key:") => {
                let key_id = &name["preloaded_key:".len()..];
                if key_id.is_empty() {
                    return Err(err_invalid_parameter("name", "key ID is empty"));
                }

                let value = value
                    .as_ref()
                    .ok_or_else(|| err_invalid_parameter("value", "expected non-NULL"))?;
                let value = std::ffi::CStr::from_ptr(value);
                let value = value
                    .to_str()
                    .map_err(|err| err_invalid_parameter("value", err))?;
                let value: PreloadedKeyLocation = value
                    .parse()
                    .map_err(|err| err_invalid_parameter("value", err))?;

                let mut guard = PRELOADED_KEYS.write().expect("fatal RwLock failure");
                guard.insert(key_id.to_owned(), value);
            }

            _ => return Err(err_invalid_parameter("name", "unrecognized value")),
        }

        {
            let pkcs11_lib_path = PKCS11_LIB_PATH.read().expect("fatal RwLock failure");
            let pkcs11_base_slot = PKCS11_BASE_SLOT.read().expect("fatal RwLock failure");

            if let (Some(pkcs11_lib_path), Some(pkcs11_base_slot)) =
                (&*pkcs11_lib_path, &*pkcs11_base_slot)
            {
                // Pre-emptively open a session to the base slot. This makes it faster to use it in the future,
                // since we won't have to log in again.

                let pkcs11_context =
                    pkcs11::Context::load(pkcs11_lib_path.clone()).map_err(err_external)?;
                let pkcs11_slot = pkcs11_context
                    .find_slot(&pkcs11_base_slot.slot_identifier)
                    .map_err(err_external)?;
                let pkcs11_session = pkcs11_context
                    .open_session(pkcs11_slot, pkcs11_base_slot.pin.clone())
                    .map_err(err_external)?;

                let mut pkcs11_base_slot_session = PKCS11_BASE_SLOT_SESSION
                    .lock()
                    .expect("fatal RwLock failure");
                let pkcs11_base_slot_session = &mut *pkcs11_base_slot_session;
                *pkcs11_base_slot_session = Some(pkcs11_session);
            }
        }

        Ok(())
    })
}

pub(crate) unsafe extern "C" fn sign(
    id: *const std::os::raw::c_char,
    mechanism: crate::AZIOT_KEYS_SIGN_MECHANISM,
    parameters: *const std::ffi::c_void,
    digest: *const std::os::raw::c_uchar,
    digest_len: usize,
    signature: *mut std::os::raw::c_uchar,
    signature_len: *mut usize,
) -> crate::AZIOT_KEYS_RC {
    crate::r#catch(|| {
        let id = {
            if id.is_null() {
                return Err(err_invalid_parameter("id", "expected non-NULL"));
            }
            let id = std::ffi::CStr::from_ptr(id);
            let id = id
                .to_str()
                .map_err(|err| err_invalid_parameter("id", err))?;
            id
        };

        let digest = if digest.is_null() {
            return Err(err_invalid_parameter("digest", "expected non-NULL"));
        } else {
            std::slice::from_raw_parts(digest, digest_len)
        };

        let mut signature_len_out = std::ptr::NonNull::new(signature_len)
            .ok_or_else(|| err_invalid_parameter("signature_len", "expected non-NULL"))?;

        let locations = Location::of(id)?;

        let (expected_signature_len, expected_signature) = match mechanism {
            crate::AZIOT_KEYS_SIGN_MECHANISM_ECDSA => {
                crate::key_pair::sign(&locations, mechanism, parameters, digest)?
            }

            crate::AZIOT_KEYS_SIGN_MECHANISM_HMAC_SHA256
            | crate::AZIOT_KEYS_SIGN_MECHANISM_DERIVED => {
                crate::key::sign(&locations, mechanism, parameters, digest)?
            }

            _ => return Err(err_invalid_parameter("mechanism", "unrecognized value")),
        };

        let actual_signature_len = *signature_len_out.as_ref();

        *signature_len_out.as_mut() = expected_signature_len;

        if !signature.is_null() {
            let expected_signature_len = expected_signature.len();

            if actual_signature_len < expected_signature_len {
                return Err(err_invalid_parameter("signature", "insufficient size"));
            }

            let signature_out = std::slice::from_raw_parts_mut(signature, actual_signature_len);

            signature_out[..expected_signature_len].copy_from_slice(&expected_signature);
            *signature_len_out.as_mut() = expected_signature_len;
        }

        Ok(())
    })
}

pub(crate) unsafe extern "C" fn verify(
    id: *const std::os::raw::c_char,
    mechanism: crate::AZIOT_KEYS_SIGN_MECHANISM,
    _parameters: *const std::ffi::c_void, // Currently unused, but may be used in the future
    digest: *const std::os::raw::c_uchar,
    digest_len: usize,
    signature: *const std::os::raw::c_uchar,
    signature_len: usize,
    ok: *mut std::os::raw::c_int,
) -> crate::AZIOT_KEYS_RC {
    crate::r#catch(|| {
        let id = {
            if id.is_null() {
                return Err(err_invalid_parameter("id", "expected non-NULL"));
            }
            let id = std::ffi::CStr::from_ptr(id);
            let id = id
                .to_str()
                .map_err(|err| err_invalid_parameter("id", err))?;
            id
        };

        let digest = if digest.is_null() {
            return Err(err_invalid_parameter("digest", "expected non-NULL"));
        } else {
            std::slice::from_raw_parts(digest, digest_len)
        };

        let signature = if signature.is_null() {
            return Err(err_invalid_parameter("signature", "expected non-NULL"));
        } else {
            std::slice::from_raw_parts(signature, signature_len)
        };

        let mut ok_out = std::ptr::NonNull::new(ok)
            .ok_or_else(|| err_invalid_parameter("ok", "expected non-NULL"))?;

        let locations = Location::of(id)?;

        #[allow(clippy::match_same_arms)]
        let ok = match mechanism {
            // Verify is not supported for asymmetric keys.
            // Clients can verify signatures themselves from the public parameters of the key pair.
            crate::AZIOT_KEYS_SIGN_MECHANISM_ECDSA => {
                return Err(err_invalid_parameter("mechanism", "unrecognized value"))
            }

            crate::AZIOT_KEYS_SIGN_MECHANISM_HMAC_SHA256 => {
                crate::key::verify(&locations, digest, signature)?
            }

            _ => return Err(err_invalid_parameter("mechanism", "unrecognized value")),
        };

        *ok_out.as_mut() = ok.into();

        Ok(())
    })
}

pub(crate) unsafe extern "C" fn encrypt(
    id: *const std::os::raw::c_char,
    mechanism: crate::AZIOT_KEYS_ENCRYPT_MECHANISM,
    parameters: *const std::ffi::c_void,
    plaintext: *const std::os::raw::c_uchar,
    plaintext_len: usize,
    ciphertext: *mut std::os::raw::c_uchar,
    ciphertext_len: *mut usize,
) -> crate::AZIOT_KEYS_RC {
    crate::r#catch(|| {
        let id = {
            if id.is_null() {
                return Err(err_invalid_parameter("id", "expected non-NULL"));
            }
            let id = std::ffi::CStr::from_ptr(id);
            let id = id
                .to_str()
                .map_err(|err| err_invalid_parameter("id", err))?;
            id
        };

        let plaintext = if plaintext.is_null() {
            return Err(err_invalid_parameter("plaintext", "expected non-NULL"));
        } else {
            std::slice::from_raw_parts(plaintext, plaintext_len)
        };

        let mut ciphertext_len_out = std::ptr::NonNull::new(ciphertext_len)
            .ok_or_else(|| err_invalid_parameter("ciphertext_len", "expected non-NULL"))?;

        let locations = Location::of(id)?;

        let (expected_ciphertext_len, expected_ciphertext) = match mechanism {
            crate::AZIOT_KEYS_ENCRYPT_MECHANISM_AEAD
            | crate::AZIOT_KEYS_ENCRYPT_MECHANISM_DERIVED => {
                crate::key::encrypt(&locations, mechanism, parameters, plaintext)?
            }

            crate::AZIOT_KEYS_ENCRYPT_MECHANISM_RSA_PKCS1
            | crate::AZIOT_KEYS_ENCRYPT_MECHANISM_RSA_NO_PADDING => {
                crate::key_pair::encrypt(&locations, mechanism, parameters, plaintext)?
            }

            _ => return Err(err_invalid_parameter("mechanism", "unrecognized value")),
        };

        let actual_ciphertext_len = *ciphertext_len_out.as_ref();

        *ciphertext_len_out.as_mut() = expected_ciphertext_len;

        if !ciphertext.is_null() {
            let expected_ciphertext_len = expected_ciphertext.len();

            if actual_ciphertext_len < expected_ciphertext_len {
                return Err(err_invalid_parameter("ciphertext", "insufficient size"));
            }

            let ciphertext_out = std::slice::from_raw_parts_mut(ciphertext, actual_ciphertext_len);

            ciphertext_out[..expected_ciphertext_len].copy_from_slice(&expected_ciphertext);
            *ciphertext_len_out.as_mut() = expected_ciphertext_len;
        }

        Ok(())
    })
}

pub(crate) unsafe extern "C" fn decrypt(
    id: *const std::os::raw::c_char,
    mechanism: crate::AZIOT_KEYS_ENCRYPT_MECHANISM,
    parameters: *const std::ffi::c_void,
    ciphertext: *const std::os::raw::c_uchar,
    ciphertext_len: usize,
    plaintext: *mut std::os::raw::c_uchar,
    plaintext_len: *mut usize,
) -> crate::AZIOT_KEYS_RC {
    crate::r#catch(|| {
        let id = {
            if id.is_null() {
                return Err(err_invalid_parameter("id", "expected non-NULL"));
            }
            let id = std::ffi::CStr::from_ptr(id);
            let id = id
                .to_str()
                .map_err(|err| err_invalid_parameter("id", err))?;
            id
        };

        let ciphertext = if ciphertext.is_null() {
            return Err(err_invalid_parameter("ciphertext", "expected non-NULL"));
        } else {
            std::slice::from_raw_parts(ciphertext, ciphertext_len)
        };

        let mut plaintext_len_out = std::ptr::NonNull::new(plaintext_len)
            .ok_or_else(|| err_invalid_parameter("plaintext_len", "expected non-NULL"))?;

        let locations = Location::of(id)?;

        let (expected_plaintext_len, expected_plaintext) = match mechanism {
            crate::AZIOT_KEYS_ENCRYPT_MECHANISM_AEAD
            | crate::AZIOT_KEYS_ENCRYPT_MECHANISM_DERIVED => {
                crate::key::decrypt(&locations, mechanism, parameters, ciphertext)?
            }

            _ => return Err(err_invalid_parameter("mechanism", "unrecognized value")),
        };

        let actual_plaintext_len = *plaintext_len_out.as_ref();

        *plaintext_len_out.as_mut() = expected_plaintext_len;

        if !plaintext.is_null() {
            let expected_plaintext_len = expected_plaintext.len();

            if actual_plaintext_len < expected_plaintext_len {
                return Err(err_invalid_parameter("plaintext", "insufficient size"));
            }

            let plaintext_out = std::slice::from_raw_parts_mut(plaintext, actual_plaintext_len);

            plaintext_out[..expected_plaintext_len].copy_from_slice(&expected_plaintext);
            *plaintext_len_out.as_mut() = expected_plaintext_len;
        }

        Ok(())
    })
}

#[derive(Debug)]
pub(crate) enum Location {
    Filesystem(std::path::PathBuf),
    Pkcs11 {
        lib_path: std::path::PathBuf,
        uri: pkcs11::Uri,
    },
}

impl Location {
    pub(crate) fn of(id: &str) -> Result<Vec<Self>, crate::AZIOT_KEYS_RC> {
        let homedir_path_guard = HOMEDIR_PATH.read().expect("fatal RwLock failure");
        let homedir_path = homedir_path_guard.as_ref();

        let pkcs11_lib_path_guard = PKCS11_LIB_PATH.read().expect("fatal RwLock failure");
        let pkcs11_lib_path = pkcs11_lib_path_guard.as_ref();

        let pkcs11_base_slot_guard = PKCS11_BASE_SLOT.read().expect("fatal RwLock failure");
        let pkcs11_base_slot = pkcs11_base_slot_guard.as_ref();

        let preloaded_keys_guard = PRELOADED_KEYS.read().expect("fatal RwLock failure");
        let preloaded_keys = &*preloaded_keys_guard;

        let mut locations = vec![];

        match (preloaded_keys.get(id), pkcs11_lib_path) {
            (Some(PreloadedKeyLocation::Filesystem { path }), _) => {
                locations.push(Location::Filesystem(path.clone()));
            }

            (Some(PreloadedKeyLocation::Pkcs11 { uri }), Some(pkcs11_lib_path)) => {
                locations.push(Location::Pkcs11 {
                    lib_path: pkcs11_lib_path.clone(),
                    uri: uri.clone(),
                });
            }

            (Some(PreloadedKeyLocation::Pkcs11 { .. }), None) => {
                return Err(err_invalid_parameter(
                    "id",
                    "pre-loaded key requires PKCS#11 lib path to be configured",
                ))
            }

            _ => (),
        }

        if locations.is_empty() {
            // Prefer to use PKCS#11 before filesystem if configured so
            if let (Some(pkcs11_lib_path), Some(pkcs11_base_slot)) =
                (pkcs11_lib_path, pkcs11_base_slot)
            {
                let mut uri = pkcs11_base_slot.clone();
                uri.object_label = Some(id.to_owned());
                locations.push(Location::Pkcs11 {
                    lib_path: pkcs11_lib_path.clone(),
                    uri,
                });
            }

            if let Some(homedir_path) = homedir_path {
                let mut path = homedir_path.clone();

                path.push("keys");

                if !path.exists() {
                    let () = std::fs::create_dir_all(&path).map_err(err_external)?;
                }

                let id_sanitized: String = id.chars().filter(char::is_ascii_alphanumeric).collect();

                let hash =
                    openssl::hash::hash(openssl::hash::MessageDigest::sha256(), id.as_bytes())?;
                let hash = hex::encode(hash);
                path.push(format!("{id_sanitized}-{hash}.key"));

                locations.push(Location::Filesystem(path));
            }
        }

        if locations.is_empty() {
            // No way to create keys
            return Err(err_invalid_parameter("id", "no way to create keys"));
        }

        Ok(locations)
    }
}

impl From<openssl::error::Error> for crate::AZIOT_KEYS_RC {
    fn from(err: openssl::error::Error) -> Self {
        err_external(err)
    }
}

impl From<openssl::error::ErrorStack> for crate::AZIOT_KEYS_RC {
    fn from(err: openssl::error::ErrorStack) -> Self {
        err_external(err)
    }
}

impl From<openssl2::Error> for crate::AZIOT_KEYS_RC {
    fn from(err: openssl2::Error) -> Self {
        err_external(err)
    }
}

pub(crate) fn err_external<E>(err: E) -> crate::AZIOT_KEYS_RC
where
    E: std::fmt::Display,
{
    log::error!("{err}");
    crate::AZIOT_KEYS_RC_ERR_EXTERNAL
}

pub(crate) fn err_invalid_parameter<E>(name: &str, err: E) -> crate::AZIOT_KEYS_RC
where
    E: std::fmt::Display,
{
    log::error!("invalid parameter {name:?}: {err}");
    crate::AZIOT_KEYS_RC_ERR_INVALID_PARAMETER
}
