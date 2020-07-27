/**
 * Copyright (c) Microsoft. All rights reserved.
 *
 * This header specifies the API used for libaziot-keys. This library is used to create and load keys by the Azure IoT Keys Service.
 *
 *
 * # API conventions
 *
 * All functions return an `unsigned int` to indicate success or failure. See the [`KEYGEN_ERROR`] type's docs for details about these constants.
 *
 * The only function exported by this library is [`KEYGEN_get_function_list`]. Call this function to get the version of the API
 * that this library exports, as well as the function pointers to the key operations. See its docs for more details.
 *
 * All calls to [`KEYGEN_get_function_list`] or any function in [`KEYGEN_FUNCTION_LIST`] are serialized, ie a function will not be called
 * while another function is running. However, it is not guaranteed that all function calls will be made from the same operating system thread.
 * Thus, implementations do not need to worry about locking to prevent concurrent access, but should also not store data in thread-local storage.
 */

#include <stdint.h>

/**
 * Represents the version of the API exported by this library.
 */
typedef unsigned int KEYGEN_VERSION;

/**
 * The base struct of all of function lists.
 */
typedef struct {
    /**
     * The version of the API represented in this function list.
     *
     * The specific subtype of `KEYGEN_FUNCTION_LIST` can be determined by inspecting this value.
     */
    KEYGEN_VERSION version;
} KEYGEN_FUNCTION_LIST;

/**
 * Error type. This is a transparent wrapper around a `std::os::raw::c_uint` (`unsigned int`).
 *
 * Either `KEYGEN_SUCCESS` or one of the `KEYGEN_ERROR_*` constants.
 */
typedef unsigned int KEYGEN_ERROR;

typedef unsigned int KEYGEN_KEY_PAIR_PARAMETER_TYPE;

/**
 * Represents the mechanism used for a sign operation.
 */
typedef unsigned int KEYGEN_SIGN_MECHANISM;

/**
 * Represents the mechanism used for an encrypt operation.
 */
typedef unsigned int KEYGEN_ENCRYPT_MECHANISM;

/**
 * The specific implementation of [`KEYGEN_FUNCTION_LIST`] for API version 2.0.0.0
 */
typedef struct {
    /**
     * The value of `base.version` must be [`KEYGEN_VERSION_2_0_0_0`].
     */
    KEYGEN_FUNCTION_LIST base;
    /**
     * Set a parameter on this library.
     *
     * `name` must not be `NULL`.
     * `value` may be `NULL`.
     *
     * The caller may free the name string after this method returns. If the implementation needs to hold on to it, it must make a copy.
     *
     * The interpretation of names and values depends on the implementation.
     *
     * # Errors
     *
     * - `KEYGEN_ERROR_INVALID_PARAMETER`:
     *   - `name` is `NULL`.
     *   - `name` is not recognized by this implementation.
     *   - `value` is invalid.
     *
     * - `KEYGEN_ERROR_FATAL`
     */
    KEYGEN_ERROR (*set_parameter)(const char *name, const char *value);
    /**
     * Create or load a key identified by the specified `id`.
     *
     * - If a key with that ID exists, the key will be loaded from that URI and returned.
     * - If a key with that ID does not exist, a new key will be created. It will be saved such that it can be looked up later using that same ID.
     *
     * `preferred_algorithms` dictates the caller's preference for the key algorithm. It is a string with components separated by COLON U+003A `:`,
     * where each component specifies the name of an algorithm and will be attempted by the implementation in that order.
     * The valid components are `"ec-p256"` for secp256r1, `"rsa-2048"` for 2048-bit RSA, `"rsa-4096"` for 4096-bit RSA, and `"*"` which indicates
     * any algorithm of the implementation's choice. For example, the caller might use `"ec-p256:rsa-2048:*"` to indicate that it would like
     * the implementation to use secp256r1, else RSA-2048 if that fails, else any other algorithm of the implementation's choice if that also fails.
     *
     * If an implementation does not recognize a particular component as an algorithm, or is unable to use the algorithm to generate a key pair,
     * it should ignore that component and try the next one. If no components are left, the implementation should return an error.
     * It is allowed for the implementation to unable to generate a key pair even if the wildcard algorithm is specified.
     *
     * If `preferred_algorithms` is NULL, it should be interpreted the same as if it was `"*"`.
     *
     * The public key is written to `ppublic_key` and the private key to `pprivate_key`.
     * For keys generated by openssl in memory, the private and public components of a key live in the same `EVP_PKEY` value.
     * However keys loaded from engines do differentiate between the two, so separate `EVP_PKEY` values are required.
     * Even if the implementation generates keys in memory using openssl, it must copy the public parameters out of the key into a new `EVP_PKEY`
     * and set `ppublic_parameters` to that.
     *
     * # Errors
     *
     * - `KEYGEN_ERROR_INVALID_PARAMETER`:
     *   - `id` is NULL.
     *   - `ppublic_key` is `NULL`.
     *   - `pprivate_key` is `NULL`.
     *
     * - `KEYGEN_ERROR_EXTERNAL`
     */
    KEYGEN_ERROR (*create_key_pair_if_not_exists)(const char *id, const char *preferred_algorithms);
    KEYGEN_ERROR (*load_key_pair)(const char *id);
    /**
     * Gets the value of a parameter of the key identified by the specified `id`.
     *
     * `type_` must be set to one of the `KEYGEN_KEY_PAIR_PARAMETER_TYPE_*` constants.
     *
     * # Errors
     *
     * - `KEYGEN_ERROR_INVALID_PARAMETER`:
     *   - `id` is NULL.
     *   - The key specified by `id` does not exist.
     *   - `type_` is not a valid parameter type for the key specified by `id`.
     *
     * - `KEYGEN_ERROR_EXTERNAL`
     */
    KEYGEN_ERROR (*get_key_pair_parameter)(const char *id, KEYGEN_KEY_PAIR_PARAMETER_TYPE type_, unsigned char *value, uintptr_t *value_len);
    KEYGEN_ERROR (*create_key_if_not_exists)(const char *id, uintptr_t length);
    KEYGEN_ERROR (*import_key)(const char *id, const uint8_t *bytes, uintptr_t bytes_len);
    KEYGEN_ERROR (*sign)(const char *id, KEYGEN_SIGN_MECHANISM mechanism, const void *parameters, const unsigned char *digest, uintptr_t digest_len, unsigned char *signature, uintptr_t *signature_len);
    /**
     * Verifies the signature of the given digest using the key identified by the specified `id`.
     *
     * `mechanism` must be set to one of the `KEYGEN_SIGN_MECHANISM_*` constants.
     *
     * If the function returns `KEYGEN_SUCCESS`, then `ok` is set to 0 if the signature is invalid and non-zero if the signature is valid.
     *
     * # Errors
     *
     * - `KEYGEN_ERROR_INVALID_PARAMETER`:
     *   - `id` is NULL.
     *   - The key specified by `id` does not exist.
     *   - `mechanism` is not a valid parameter type for the key specified by `id`.
     *
     * - `KEYGEN_ERROR_EXTERNAL`
     */
    KEYGEN_ERROR (*verify)(const char *id, KEYGEN_SIGN_MECHANISM mechanism, const void *parameters, const unsigned char *digest, uintptr_t digest_len, const unsigned char *signature, uintptr_t signature_len, int *ok);
    KEYGEN_ERROR (*encrypt)(const char *id, KEYGEN_ENCRYPT_MECHANISM mechanism, const void *parameters, const unsigned char *plaintext, uintptr_t plaintext_len, unsigned char *ciphertext, uintptr_t *ciphertext_len);
    KEYGEN_ERROR (*decrypt)(const char *id, KEYGEN_ENCRYPT_MECHANISM mechanism, const void *parameters, const unsigned char *ciphertext, uintptr_t ciphertext_len, unsigned char *plaintext, uintptr_t *plaintext_len);
} KEYGEN_FUNCTION_LIST_2_0_0_0;

/**
 * Holds parameters for an encrypt operation with the [`KEYGEN_ENCRYPT_MECHANISM_AEAD`] mechanism.
 */
typedef struct {
    const unsigned char *iv;
    uintptr_t iv_len;
    const unsigned char *aad;
    uintptr_t aad_len;
} KEYGEN_ENCRYPT_AEAD_PARAMETERS;

typedef unsigned int KEYGEN_KEY_PAIR_PARAMETER_ALGORITHM;

/**
 * AEAD (eg AES-256-GCM)
 */
#define KEYGEN_ENCRYPT_MECHANISM_AEAD 1

/**
 * RSA PKCS1
 */
#define KEYGEN_ENCRYPT_MECHANISM_RSA_PKCS1 2

/**
 * The library encountered an error with an external resource, such as an I/O error or RPC error.
 */
#define KEYGEN_ERROR_EXTERNAL 3

/**
 * The library encountered an unrecoverable error. The process should exit as soon as possible.
 */
#define KEYGEN_ERROR_FATAL 1

/**
 * The operation failed because a parameter has an invalid value.
 */
#define KEYGEN_ERROR_INVALID_PARAMETER 2

#define KEYGEN_KEY_PAIR_PARAMETER_ALGORITHM_EC 1

#define KEYGEN_KEY_PAIR_PARAMETER_ALGORITHM_RSA 2

/**
 * Used as the parameter type with `get_key_pair_parameter` to get the key algorithm.
 *
 * The value returned by `get_key_pair_parameter` will be one of the `KEYGEN_KEY_PAIR_PARAMETER_ALGORITHM_*` constants.
 */
#define KEYGEN_KEY_PAIR_PARAMETER_TYPE_ALGORITHM 1

/**
 * Used as the parameter type with `get_key_pair_parameter` to get the curve OID of an EC key.
 *
 * The value returned by `get_key_pair_parameter` will be a byte buffer containing a DER-encoded OID.
 */
#define KEYGEN_KEY_PAIR_PARAMETER_TYPE_EC_CURVE_OID 2

/**
 * Used as the parameter type with `get_key_pair_parameter` to get the point of an EC key.
 *
 * The value returned by `get_key_pair_parameter` will be a byte buffer containing a DER-encoded octet string in RFC 5490 format.
 */
#define KEYGEN_KEY_PAIR_PARAMETER_TYPE_EC_POINT 3

/**
 * Used as the parameter type with `get_key_pair_parameter` to get the exponent of an RSA key.
 *
 * The value returned by `get_key_pair_parameter` will be a byte buffer holding a big-endian bignum.
 */
#define KEYGEN_KEY_PAIR_PARAMETER_TYPE_RSA_EXPONENT 5

/**
 * Used as the parameter type with `get_key_pair_parameter` to get the modulus of an RSA key.
 *
 * The value returned by `get_key_pair_parameter` will be a byte buffer holding a big-endian bignum.
 */
#define KEYGEN_KEY_PAIR_PARAMETER_TYPE_RSA_MODULUS 4

/**
 * ECDSA
 */
#define KEYGEN_SIGN_MECHANISM_ECDSA 1

/**
 * HMAC-SHA256
 */
#define KEYGEN_SIGN_MECHANISM_HMAC_SHA256 2

/**
 * The operation succeeded.
 */
#define KEYGEN_SUCCESS 0

/**
 * Version 2.0.0.0
 */
#define KEYGEN_VERSION_2_0_0_0 33554432


/**
 * Get the list of functions for operations corresponding to the specified version.
 *
 * Implementations can use this function for initialization, since it is guaranteed to be called before any operations.
 * However it is not an error to call this function multiple times, for the same or different version,
 * so implementations must ensure they only run their initialization once.
 *
 * The pointer returned from this function must not be freed by the caller, and its contents must not be mutated.
 *
 * # Errors
 *
 * - `KEYGEN_ERROR_INVALID_PARAMETER`:
 *   - `version` is not recognized by this implementation.
 *   - `pfunction_list` is NULL.
 */
KEYGEN_ERROR KEYGEN_get_function_list(KEYGEN_VERSION version,
                                      const KEYGEN_FUNCTION_LIST **pfunction_list);
