// Copyright (c) Microsoft. All rights reserved.

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug)]
pub struct KeysRawError(pub(crate) sys::AZIOT_KEYS_RC);

impl std::fmt::Display for KeysRawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            sys::AZIOT_KEYS_RC_ERR_INVALID_PARAMETER => {
                f.write_str("AZIOT_KEYS_RC_ERR_INVALID_PARAMETER")
            }
            sys::AZIOT_KEYS_RC_ERR_EXTERNAL => f.write_str("AZIOT_KEYS_RC_ERR_EXTERNAL"),
            err => write!(f, "0x{err:08x}"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Keys {
    V2_0_0_0 {
        set_parameter: unsafe extern "C" fn(
            name: *const std::os::raw::c_char,
            value: *const std::os::raw::c_char,
        ) -> sys::AZIOT_KEYS_RC,

        create_key_pair_if_not_exists: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            preferred_algorithms: *const std::os::raw::c_char,
        ) -> sys::AZIOT_KEYS_RC,

        load_key_pair: unsafe extern "C" fn(id: *const std::os::raw::c_char) -> sys::AZIOT_KEYS_RC,

        get_key_pair_parameter: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            r#type: sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE,
            value: *mut std::os::raw::c_uchar,
            value_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        create_key_if_not_exists: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            usage: sys::AZIOT_KEYS_KEY_USAGE,
        ) -> sys::AZIOT_KEYS_RC,

        load_key: unsafe extern "C" fn(id: *const std::os::raw::c_char) -> sys::AZIOT_KEYS_RC,

        import_key: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            bytes: *const u8,
            bytes_len: usize,
            usage: sys::AZIOT_KEYS_KEY_USAGE,
        ) -> sys::AZIOT_KEYS_RC,

        derive_key: unsafe extern "C" fn(
            base_id: *const std::os::raw::c_char,
            derivation_data: *const std::os::raw::c_uchar,
            derivation_data_len: usize,
            derived_key: *mut std::os::raw::c_uchar,
            derived_key_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        sign: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            digest: *const std::os::raw::c_uchar,
            digest_len: usize,
            signature: *mut std::os::raw::c_uchar,
            signature_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        verify: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            digest: *const std::os::raw::c_uchar,
            digest_len: usize,
            signature: *const std::os::raw::c_uchar,
            signature_len: usize,
            ok: *mut std::os::raw::c_int,
        ) -> sys::AZIOT_KEYS_RC,

        encrypt: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            plaintext: *const std::os::raw::c_uchar,
            plaintext_len: usize,
            ciphertext: *mut std::os::raw::c_uchar,
            ciphertext_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        decrypt: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            ciphertext: *const std::os::raw::c_uchar,
            ciphertext_len: usize,
            plaintext: *mut std::os::raw::c_uchar,
            plaintext_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,
    },

    V2_1_0_0 {
        set_parameter: unsafe extern "C" fn(
            name: *const std::os::raw::c_char,
            value: *const std::os::raw::c_char,
        ) -> sys::AZIOT_KEYS_RC,

        create_key_pair_if_not_exists: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            preferred_algorithms: *const std::os::raw::c_char,
        ) -> sys::AZIOT_KEYS_RC,

        move_key_pair: unsafe extern "C" fn(
            from: *const std::os::raw::c_char,
            to: *const std::os::raw::c_char,
        ) -> sys::AZIOT_KEYS_RC,

        load_key_pair: unsafe extern "C" fn(id: *const std::os::raw::c_char) -> sys::AZIOT_KEYS_RC,

        get_key_pair_parameter: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            r#type: sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE,
            value: *mut std::os::raw::c_uchar,
            value_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        delete_key_pair:
            unsafe extern "C" fn(id: *const std::os::raw::c_char) -> sys::AZIOT_KEYS_RC,

        create_key_if_not_exists: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            usage: sys::AZIOT_KEYS_KEY_USAGE,
        ) -> sys::AZIOT_KEYS_RC,

        load_key: unsafe extern "C" fn(id: *const std::os::raw::c_char) -> sys::AZIOT_KEYS_RC,

        import_key: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            bytes: *const u8,
            bytes_len: usize,
            usage: sys::AZIOT_KEYS_KEY_USAGE,
        ) -> sys::AZIOT_KEYS_RC,

        delete_key: unsafe extern "C" fn(id: *const std::os::raw::c_char) -> sys::AZIOT_KEYS_RC,

        derive_key: unsafe extern "C" fn(
            base_id: *const std::os::raw::c_char,
            derivation_data: *const std::os::raw::c_uchar,
            derivation_data_len: usize,
            derived_key: *mut std::os::raw::c_uchar,
            derived_key_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        sign: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            digest: *const std::os::raw::c_uchar,
            digest_len: usize,
            signature: *mut std::os::raw::c_uchar,
            signature_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        verify: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            digest: *const std::os::raw::c_uchar,
            digest_len: usize,
            signature: *const std::os::raw::c_uchar,
            signature_len: usize,
            ok: *mut std::os::raw::c_int,
        ) -> sys::AZIOT_KEYS_RC,

        encrypt: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            plaintext: *const std::os::raw::c_uchar,
            plaintext_len: usize,
            ciphertext: *mut std::os::raw::c_uchar,
            ciphertext_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,

        decrypt: unsafe extern "C" fn(
            id: *const std::os::raw::c_char,
            mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
            parameters: *const std::ffi::c_void,
            ciphertext: *const std::os::raw::c_uchar,
            ciphertext_len: usize,
            plaintext: *mut std::os::raw::c_uchar,
            plaintext_len: *mut usize,
        ) -> sys::AZIOT_KEYS_RC,
    },
}

impl Keys {
    pub(crate) fn new() -> Result<Self, LoadLibraryError> {
        unsafe {
            let mut function_list: *const sys::AZIOT_KEYS_FUNCTION_LIST = std::ptr::null_mut();
            keys_ok(sys::aziot_keys_get_function_list(
                sys::AZIOT_KEYS_VERSION_2_1_0_0,
                &raw mut function_list,
            ))
            .or_else(|_| {
                keys_ok(sys::aziot_keys_get_function_list(
                    sys::AZIOT_KEYS_VERSION_2_0_0_0,
                    &raw mut function_list,
                ))
            })
            .map_err(LoadLibraryError::GetFunctionList)?;

            let api_version = (*function_list).version;
            let result = match api_version {
                sys::AZIOT_KEYS_VERSION_2_0_0_0 => {
                    // AZIOT_KEYS_FUNCTION_LIST has looser alignment than AZIOT_KEYS_FUNCTION_LIST_2_0_0_0, but the pointer comes from the library itself,
                    // so it will be correctly aligned already.
                    #[allow(clippy::cast_ptr_alignment)]
                    let function_list =
                        function_list.cast::<sys::AZIOT_KEYS_FUNCTION_LIST_2_0_0_0>();

                    Keys::V2_0_0_0 {
                        set_parameter: (*function_list)
                            .set_parameter
                            .ok_or(LoadLibraryError::MissingFunction("set_parameter"))?,

                        create_key_pair_if_not_exists: (*function_list)
                            .create_key_pair_if_not_exists
                            .ok_or(LoadLibraryError::MissingFunction(
                                "create_key_pair_if_not_exists",
                            ))?,

                        load_key_pair: (*function_list)
                            .load_key_pair
                            .ok_or(LoadLibraryError::MissingFunction("load_key_pair"))?,

                        get_key_pair_parameter: (*function_list)
                            .get_key_pair_parameter
                            .ok_or(LoadLibraryError::MissingFunction("get_key_pair_parameter"))?,

                        create_key_if_not_exists: (*function_list).create_key_if_not_exists.ok_or(
                            LoadLibraryError::MissingFunction("create_key_if_not_exists"),
                        )?,

                        load_key: (*function_list)
                            .load_key
                            .ok_or(LoadLibraryError::MissingFunction("load_key"))?,

                        import_key: (*function_list)
                            .import_key
                            .ok_or(LoadLibraryError::MissingFunction("import_key"))?,

                        derive_key: (*function_list)
                            .derive_key
                            .ok_or(LoadLibraryError::MissingFunction("derive_key"))?,

                        sign: (*function_list)
                            .sign
                            .ok_or(LoadLibraryError::MissingFunction("sign"))?,

                        verify: (*function_list)
                            .verify
                            .ok_or(LoadLibraryError::MissingFunction("verify"))?,

                        encrypt: (*function_list)
                            .encrypt
                            .ok_or(LoadLibraryError::MissingFunction("encrypt"))?,

                        decrypt: (*function_list)
                            .decrypt
                            .ok_or(LoadLibraryError::MissingFunction("decrypt"))?,
                    }
                }

                sys::AZIOT_KEYS_VERSION_2_1_0_0 => {
                    // AZIOT_KEYS_FUNCTION_LIST has looser alignment than AZIOT_KEYS_FUNCTION_LIST_2_1_0_0, but the pointer comes from the library itself,
                    // so it will be correctly aligned already.
                    #[allow(clippy::cast_ptr_alignment)]
                    let function_list =
                        function_list.cast::<sys::AZIOT_KEYS_FUNCTION_LIST_2_1_0_0>();

                    Keys::V2_1_0_0 {
                        set_parameter: (*function_list)
                            .set_parameter
                            .ok_or(LoadLibraryError::MissingFunction("set_parameter"))?,

                        create_key_pair_if_not_exists: (*function_list)
                            .create_key_pair_if_not_exists
                            .ok_or(LoadLibraryError::MissingFunction(
                                "create_key_pair_if_not_exists",
                            ))?,

                        move_key_pair: (*function_list)
                            .move_key_pair
                            .ok_or(LoadLibraryError::MissingFunction("move_key_pair"))?,

                        load_key_pair: (*function_list)
                            .load_key_pair
                            .ok_or(LoadLibraryError::MissingFunction("load_key_pair"))?,

                        get_key_pair_parameter: (*function_list)
                            .get_key_pair_parameter
                            .ok_or(LoadLibraryError::MissingFunction("get_key_pair_parameter"))?,

                        delete_key_pair: (*function_list)
                            .delete_key_pair
                            .ok_or(LoadLibraryError::MissingFunction("delete_key_pair"))?,

                        create_key_if_not_exists: (*function_list).create_key_if_not_exists.ok_or(
                            LoadLibraryError::MissingFunction("create_key_if_not_exists"),
                        )?,

                        load_key: (*function_list)
                            .load_key
                            .ok_or(LoadLibraryError::MissingFunction("load_key"))?,

                        import_key: (*function_list)
                            .import_key
                            .ok_or(LoadLibraryError::MissingFunction("import_key"))?,

                        delete_key: (*function_list)
                            .delete_key
                            .ok_or(LoadLibraryError::MissingFunction("delete_key"))?,

                        derive_key: (*function_list)
                            .derive_key
                            .ok_or(LoadLibraryError::MissingFunction("derive_key"))?,

                        sign: (*function_list)
                            .sign
                            .ok_or(LoadLibraryError::MissingFunction("sign"))?,

                        verify: (*function_list)
                            .verify
                            .ok_or(LoadLibraryError::MissingFunction("verify"))?,

                        encrypt: (*function_list)
                            .encrypt
                            .ok_or(LoadLibraryError::MissingFunction("encrypt"))?,

                        decrypt: (*function_list)
                            .decrypt
                            .ok_or(LoadLibraryError::MissingFunction("decrypt"))?,
                    }
                }

                api_version => return Err(LoadLibraryError::UnsupportedApiVersion(api_version)),
            };

            log::info!("Loaded libaziot-keys with version 0x{api_version:08x}");

            Ok(result)
        }
    }
}

#[derive(Debug)]
pub enum LoadLibraryError {
    GetFunctionList(KeysRawError),
    MissingFunction(&'static str),
    UnsupportedApiVersion(sys::AZIOT_KEYS_VERSION),
}

impl std::fmt::Display for LoadLibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadLibraryError::GetFunctionList(inner) => {
                write!(f, "could not get function list: {inner}")
            }
            LoadLibraryError::MissingFunction(name) => {
                write!(f, "library does not define {name}")
            }
            LoadLibraryError::UnsupportedApiVersion(api_version) => write!(
                f,
                "library exports API version 0x{api_version:08x} which is not supported"
            ),
        }
    }
}

impl std::error::Error for LoadLibraryError {}

impl Keys {
    pub(crate) fn set_parameter(
        &mut self,
        name: &std::ffi::CStr,
        value: &std::ffi::CStr,
    ) -> Result<(), SetLibraryParameterError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { set_parameter, .. } | Keys::V2_1_0_0 { set_parameter, .. } => {
                    keys_ok(set_parameter(name.as_ptr(), value.as_ptr())).map_err(|err| {
                        SetLibraryParameterError {
                            name: name.to_string_lossy().into_owned(),
                            err,
                        }
                    })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct SetLibraryParameterError {
    name: String,
    err: KeysRawError,
}

impl std::fmt::Display for SetLibraryParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "could not set {} parameter on library: {}",
            self.name, self.err
        )
    }
}

impl std::error::Error for SetLibraryParameterError {}

impl Keys {
    pub(crate) fn create_key_pair_if_not_exists(
        &mut self,
        id: &std::ffi::CStr,
        preferred_algorithms: Option<&std::ffi::CStr>,
    ) -> Result<(), CreateKeyPairIfNotExistsError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 {
                    create_key_pair_if_not_exists,
                    ..
                }
                | Keys::V2_1_0_0 {
                    create_key_pair_if_not_exists,
                    ..
                } => {
                    keys_ok(create_key_pair_if_not_exists(
                        id.as_ptr(),
                        preferred_algorithms.map_or(std::ptr::null(), std::ffi::CStr::as_ptr),
                    ))
                    .map_err(|err| CreateKeyPairIfNotExistsError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CreateKeyPairIfNotExistsError {
    pub err: KeysRawError,
}

impl std::fmt::Display for CreateKeyPairIfNotExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not create key pair: {}", self.err)
    }
}

impl std::error::Error for CreateKeyPairIfNotExistsError {}

impl Keys {
    pub(crate) fn load_key_pair(&mut self, id: &std::ffi::CStr) -> Result<(), LoadKeyPairError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { load_key_pair, .. } | Keys::V2_1_0_0 { load_key_pair, .. } => {
                    keys_ok(load_key_pair(id.as_ptr())).map_err(|err| LoadKeyPairError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct LoadKeyPairError {
    pub err: KeysRawError,
}

impl std::fmt::Display for LoadKeyPairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not load key pair: {}", self.err)
    }
}

impl std::error::Error for LoadKeyPairError {}

impl Keys {
    pub(crate) fn get_key_pair_public_parameter(
        &mut self,
        id: &std::ffi::CStr,
        parameter_name: &str,
    ) -> Result<String, GetKeyPairPublicParameterError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 {
                    get_key_pair_parameter,
                    ..
                }
                | Keys::V2_1_0_0 {
                    get_key_pair_parameter,
                    ..
                } => {
                    match parameter_name {
                        "algorithm" => {
                            let mut algorithm: sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_ALGORITHM = 0;
                            let mut algorithm_len = std::mem::size_of_val(&algorithm);

                            keys_ok(get_key_pair_parameter(
                                id.as_ptr(),
                                sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE_ALGORITHM,
                                std::ptr::addr_of_mut!(algorithm).cast(),
                                &raw mut algorithm_len,
                            ))
                            .map_err(|err| GetKeyPairPublicParameterError::Api { err })?;

                            if algorithm_len != std::mem::size_of_val(&algorithm) {
                                return Err(GetKeyPairPublicParameterError::UnrecognizedKeyAlgorithmLength { algorithm_len });
                            }

                            let algorithm =
                                match algorithm {
                                    sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_ALGORITHM_EC => {
                                        "ECDSA".to_owned()
                                    }
                                    sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_ALGORITHM_RSA => {
                                        "RSA".to_owned()
                                    }
                                    algorithm => return Err(
                                        GetKeyPairPublicParameterError::UnrecognizedKeyAlgorithm {
                                            algorithm,
                                        },
                                    ),
                                };
                            Ok(algorithm)
                        }

                        parameter_name => {
                            // These are all byte-buf parameters, so they can be handled identically.

                            let parameter_type = match parameter_name {
                                "ec-curve-oid" => {
                                    sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE_EC_CURVE_OID
                                }
                                "ec-point" => sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE_EC_POINT,
                                "rsa-modulus" => {
                                    sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE_RSA_MODULUS
                                }
                                "rsa-exponent" => {
                                    sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE_RSA_EXPONENT
                                }
                                _ => {
                                    return Err(GetKeyPairPublicParameterError::Api {
                                        err: KeysRawError(sys::AZIOT_KEYS_RC_ERR_INVALID_PARAMETER),
                                    })
                                }
                            };

                            let parameter_value = get_key_pair_parameter_byte_buf(
                                *get_key_pair_parameter,
                                id,
                                parameter_type,
                            )
                            .map_err(|err| GetKeyPairPublicParameterError::Api { err })?;
                            let engine = base64::engine::general_purpose::STANDARD;
                            let parameter_value = base64::Engine::encode(&engine, parameter_value);
                            Ok(parameter_value)
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum GetKeyPairPublicParameterError {
    Api {
        err: KeysRawError,
    },
    UnrecognizedKeyAlgorithm {
        algorithm: sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_ALGORITHM,
    },
    UnrecognizedKeyAlgorithmLength {
        algorithm_len: usize,
    },
}

impl std::fmt::Display for GetKeyPairPublicParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetKeyPairPublicParameterError::Api { err } => {
                write!(f, "could not get key pair parameter: {err}")
            }
            GetKeyPairPublicParameterError::UnrecognizedKeyAlgorithm { algorithm } => write!(
                f,
                "could not get key pair parameter: key has unknown algorithm {algorithm}"
            ),
            GetKeyPairPublicParameterError::UnrecognizedKeyAlgorithmLength { algorithm_len } => {
                write!(
                f,
                "could not get key pair parameter: key has unknown algorithm value of length {algorithm_len}"
            )
            }
        }
    }
}

impl std::error::Error for GetKeyPairPublicParameterError {}

impl Keys {
    pub(crate) fn move_key_pair(
        &mut self,
        from: &std::ffi::CStr,
        to: &std::ffi::CStr,
    ) -> Result<(), MoveKeyPairError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { .. } => {
                    // This version doesn't support moving key pairs, so treat it as a no-op.
                    Ok(())
                }

                Keys::V2_1_0_0 { move_key_pair, .. } => {
                    keys_ok(move_key_pair(from.as_ptr(), to.as_ptr()))
                        .map_err(|err| MoveKeyPairError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MoveKeyPairError {
    pub err: KeysRawError,
}

impl std::fmt::Display for MoveKeyPairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not move key pair: {}", self.err)
    }
}

impl std::error::Error for MoveKeyPairError {}

impl Keys {
    pub(crate) fn delete_key_pair(
        &mut self,
        id: &std::ffi::CStr,
    ) -> Result<(), DeleteKeyPairError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { .. } => {
                    // This version doesn't support deleting key pairs, so treat it as a no-op.
                    Ok(())
                }

                Keys::V2_1_0_0 {
                    delete_key_pair, ..
                } => {
                    keys_ok(delete_key_pair(id.as_ptr()))
                        .map_err(|err| DeleteKeyPairError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct DeleteKeyPairError {
    pub err: KeysRawError,
}

impl std::fmt::Display for DeleteKeyPairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not delete key pair: {}", self.err)
    }
}

impl std::error::Error for DeleteKeyPairError {}

impl Keys {
    pub(crate) fn create_key_if_not_exists(
        &mut self,
        id: &std::ffi::CStr,
        usage: sys::AZIOT_KEYS_KEY_USAGE,
    ) -> Result<(), CreateKeyIfNotExistsError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 {
                    create_key_if_not_exists,
                    ..
                }
                | Keys::V2_1_0_0 {
                    create_key_if_not_exists,
                    ..
                } => {
                    keys_ok(create_key_if_not_exists(id.as_ptr(), usage))
                        .map_err(|err| CreateKeyIfNotExistsError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CreateKeyIfNotExistsError {
    pub err: KeysRawError,
}

impl std::fmt::Display for CreateKeyIfNotExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not create key: {}", self.err)
    }
}

impl std::error::Error for CreateKeyIfNotExistsError {}

impl Keys {
    pub(crate) fn load_key(&mut self, id: &std::ffi::CStr) -> Result<(), LoadKeyError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { load_key, .. } | Keys::V2_1_0_0 { load_key, .. } => {
                    keys_ok(load_key(id.as_ptr())).map_err(|err| LoadKeyError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct LoadKeyError {
    pub err: KeysRawError,
}

impl std::fmt::Display for LoadKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not load key: {}", self.err)
    }
}

impl std::error::Error for LoadKeyError {}

impl Keys {
    pub(crate) fn import_key(
        &mut self,
        id: &std::ffi::CStr,
        bytes: &[u8],
        usage: sys::AZIOT_KEYS_KEY_USAGE,
    ) -> Result<(), ImportKeyError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { import_key, .. } | Keys::V2_1_0_0 { import_key, .. } => {
                    keys_ok(import_key(id.as_ptr(), bytes.as_ptr(), bytes.len(), usage))
                        .map_err(|err| ImportKeyError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ImportKeyError {
    pub err: KeysRawError,
}

impl std::fmt::Display for ImportKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not import key: {}", self.err)
    }
}

impl std::error::Error for ImportKeyError {}

impl Keys {
    pub(crate) fn delete_key(&mut self, id: &std::ffi::CStr) -> Result<(), DeleteKeyError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { .. } => {
                    // This version doesn't support deleting keys, so treat it as a no-op.
                    Ok(())
                }

                Keys::V2_1_0_0 { delete_key, .. } => {
                    keys_ok(delete_key(id.as_ptr())).map_err(|err| DeleteKeyError { err })?;

                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct DeleteKeyError {
    pub err: KeysRawError,
}

impl std::fmt::Display for DeleteKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not delete key: {}", self.err)
    }
}

impl std::error::Error for DeleteKeyError {}

impl Keys {
    pub(crate) fn derive_key(
        &mut self,
        base_id: &std::ffi::CStr,
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, DeriveKeyError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { derive_key, .. } | Keys::V2_1_0_0 { derive_key, .. } => {
                    let mut derived_key_len = 0;

                    keys_ok(derive_key(
                        base_id.as_ptr(),
                        derivation_data.as_ptr(),
                        derivation_data.len(),
                        std::ptr::null_mut(),
                        &raw mut derived_key_len,
                    ))
                    .map_err(|err| DeriveKeyError { err })?;

                    let mut derived_key = vec![0_u8; derived_key_len];

                    keys_ok(derive_key(
                        base_id.as_ptr(),
                        derivation_data.as_ptr(),
                        derivation_data.len(),
                        derived_key.as_mut_ptr(),
                        &raw mut derived_key_len,
                    ))
                    .map_err(|err| DeriveKeyError { err })?;

                    if derived_key_len > derived_key.len() {
                        // libaziot-keys scribbled past the end of the buffer. Crash as soon as possible.
                        std::process::abort();
                    }

                    derived_key.resize(derived_key_len, 0);

                    Ok(derived_key)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct DeriveKeyError {
    pub err: KeysRawError,
}

impl std::fmt::Display for DeriveKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not derive key: {}", self.err)
    }
}

impl std::error::Error for DeriveKeyError {}

impl Keys {
    pub(crate) fn sign(
        &mut self,
        id: &std::ffi::CStr,
        mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
        parameters: *const std::ffi::c_void,
        digest: &[u8],
    ) -> Result<Vec<u8>, SignError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { sign, .. } | Keys::V2_1_0_0 { sign, .. } => {
                    let mut signature_len = 0;

                    keys_ok(sign(
                        id.as_ptr(),
                        mechanism,
                        parameters,
                        digest.as_ptr(),
                        digest.len(),
                        std::ptr::null_mut(),
                        &raw mut signature_len,
                    ))
                    .map_err(|err| SignError { err })?;

                    let mut signature = vec![0_u8; signature_len];

                    keys_ok(sign(
                        id.as_ptr(),
                        mechanism,
                        parameters,
                        digest.as_ptr(),
                        digest.len(),
                        signature.as_mut_ptr(),
                        &raw mut signature_len,
                    ))
                    .map_err(|err| SignError { err })?;

                    if signature_len > signature.len() {
                        // libaziot-keys scribbled past the end of the buffer. Crash as soon as possible.
                        std::process::abort();
                    }

                    signature.resize(signature_len, 0);

                    Ok(signature)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct SignError {
    pub err: KeysRawError,
}

impl std::fmt::Display for SignError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not sign: {}", self.err)
    }
}

impl std::error::Error for SignError {}

impl Keys {
    pub(crate) fn verify(
        &mut self,
        id: &std::ffi::CStr,
        mechanism: sys::AZIOT_KEYS_SIGN_MECHANISM,
        parameters: *const std::ffi::c_void,
        digest: &[u8],
        signature: &[u8],
    ) -> Result<bool, VerifyError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { verify, .. } | Keys::V2_1_0_0 { verify, .. } => {
                    let mut ok = 0;

                    keys_ok(verify(
                        id.as_ptr(),
                        mechanism,
                        parameters,
                        digest.as_ptr(),
                        digest.len(),
                        signature.as_ptr(),
                        signature.len(),
                        &raw mut ok,
                    ))
                    .map_err(|err| VerifyError { err })?;

                    let ok = ok != 0;
                    Ok(ok)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct VerifyError {
    pub err: KeysRawError,
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not verify: {}", self.err)
    }
}

impl std::error::Error for VerifyError {}

impl Keys {
    pub(crate) fn encrypt(
        &mut self,
        id: &std::ffi::CStr,
        mechanism: sys::AZIOT_KEYS_ENCRYPT_MECHANISM,
        parameters: *const std::ffi::c_void,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, EncryptError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { encrypt, .. } | Keys::V2_1_0_0 { encrypt, .. } => {
                    let mut ciphertext_len = 0;

                    keys_ok(encrypt(
                        id.as_ptr(),
                        mechanism,
                        parameters,
                        plaintext.as_ptr(),
                        plaintext.len(),
                        std::ptr::null_mut(),
                        &raw mut ciphertext_len,
                    ))
                    .map_err(|err| EncryptError { err })?;

                    let mut ciphertext = vec![0_u8; ciphertext_len];

                    keys_ok(encrypt(
                        id.as_ptr(),
                        mechanism,
                        parameters,
                        plaintext.as_ptr(),
                        plaintext.len(),
                        ciphertext.as_mut_ptr(),
                        &raw mut ciphertext_len,
                    ))
                    .map_err(|err| EncryptError { err })?;

                    if ciphertext_len > ciphertext.len() {
                        // libaziot-keys scribbled past the end of the buffer. Crash as soon as possible.
                        std::process::abort();
                    }

                    ciphertext.resize(ciphertext_len, 0);

                    Ok(ciphertext)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct EncryptError {
    pub err: KeysRawError,
}

impl std::fmt::Display for EncryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not encrypt: {}", self.err)
    }
}

impl std::error::Error for EncryptError {}

impl Keys {
    pub(crate) fn decrypt(
        &mut self,
        id: &std::ffi::CStr,
        mechanism: sys::AZIOT_KEYS_ENCRYPT_MECHANISM,
        parameters: *const std::ffi::c_void,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, DecryptError> {
        unsafe {
            match self {
                Keys::V2_0_0_0 { decrypt, .. } | Keys::V2_1_0_0 { decrypt, .. } => {
                    let mut plaintext_len = 0;

                    keys_ok(decrypt(
                        id.as_ptr(),
                        mechanism,
                        parameters,
                        ciphertext.as_ptr(),
                        ciphertext.len(),
                        std::ptr::null_mut(),
                        &raw mut plaintext_len,
                    ))
                    .map_err(|err| DecryptError { err })?;

                    let mut plaintext = vec![0_u8; plaintext_len];

                    keys_ok(decrypt(
                        id.as_ptr(),
                        mechanism,
                        parameters,
                        ciphertext.as_ptr(),
                        ciphertext.len(),
                        plaintext.as_mut_ptr(),
                        &raw mut plaintext_len,
                    ))
                    .map_err(|err| DecryptError { err })?;

                    if plaintext_len > plaintext.len() {
                        // libaziot-keys scribbled past the end of the buffer. Crash as soon as possible.
                        std::process::abort();
                    }

                    plaintext.resize(plaintext_len, 0);

                    Ok(plaintext)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct DecryptError {
    pub err: KeysRawError,
}

impl std::fmt::Display for DecryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not decrypt: {}", self.err)
    }
}

impl std::error::Error for DecryptError {}

fn keys_ok(result: sys::AZIOT_KEYS_RC) -> Result<(), KeysRawError> {
    match result {
        sys::AZIOT_KEYS_RC_OK => Ok(()),
        err => Err(KeysRawError(err)),
    }
}

unsafe fn get_key_pair_parameter_byte_buf(
    get_key_pair_parameter: unsafe extern "C" fn(
        id: *const std::os::raw::c_char,
        r#type: sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE,
        value: *mut std::os::raw::c_uchar,
        value_len: *mut usize,
    ) -> sys::AZIOT_KEYS_RC,
    id: &std::ffi::CStr,
    r#type: sys::AZIOT_KEYS_KEY_PAIR_PARAMETER_TYPE,
) -> Result<Vec<u8>, KeysRawError> {
    let mut value_len: usize = 0;

    keys_ok(get_key_pair_parameter(
        id.as_ptr(),
        r#type,
        std::ptr::null_mut(),
        &raw mut value_len,
    ))?;

    let mut value = vec![0_u8; value_len];

    keys_ok(get_key_pair_parameter(
        id.as_ptr(),
        r#type,
        value.as_mut_ptr(),
        &raw mut value_len,
    ))?;

    if value_len > value.len() {
        // libaziot-keys scribbled past the end of the buffer. Crash as soon as possible.
        std::process::abort();
    }

    Ok(value)
}

pub(crate) mod sys {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        unused,
        clippy::borrow_as_ptr,
        clippy::too_many_lines,
        clippy::unreadable_literal,
        clippy::unseparated_literal_suffix,
        clippy::upper_case_acronyms
    )]
    // The tests generated by bindgen deref nullptr to test for size and alignment of the structs and fields.
    // While this is indeed UB, this is only done for tests, so allow it.
    //
    // Ref: https://github.com/rust-lang/rust-bindgen/issues/1651
    #![cfg_attr(test, allow(deref_nullptr))]

    use openssl_sys::EVP_PKEY;

    include!("keys.generated.rs");
}
