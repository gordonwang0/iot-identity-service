// Copyright (c) Microsoft. All rights reserved.

#![deny(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::let_and_return,
    let_underscore_drop,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::type_complexity
)]

mod dynrange;
pub use dynrange::DynRangeBounds;

mod connector;
pub use connector::AsyncStream;
pub use connector::SOCKET_DEFAULT_PERMISSION;
pub use connector::{Connector, ConnectorError, Incoming, Stream};

mod proxy;
pub use proxy::{get_proxy_uri, MaybeProxyConnector};

mod request;
pub use request::{HttpRequest, HttpResponse};

pub mod server;

mod backoff;

mod uid;

pub type EndpointRegex = std::sync::LazyLock<regex::Regex>;

/// Ref <https://url.spec.whatwg.org/#path-percent-encode-set>
pub const PATH_SEGMENT_ENCODE_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`') // fragment percent-encode set
    .add(b'#')
    .add(b'?')
    .add(b'{')
    .add(b'}'); // path percent-encode set

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByteString(pub Vec<u8>);

impl<'de> serde::Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = ByteString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "a base64-encoded string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let engine = base64::engine::general_purpose::STANDARD;

                Ok(ByteString(
                    base64::Engine::decode(&engine, v).map_err(serde::de::Error::custom)?,
                ))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl serde::Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let engine = base64::engine::general_purpose::STANDARD;

        base64::Engine::encode(&engine, &self.0).serialize(serializer)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorBody<'a> {
    pub message: std::borrow::Cow<'a, str>,
}

impl std::convert::From<ErrorBody<'_>> for std::io::Error {
    fn from(err: ErrorBody<'_>) -> Self {
        std::io::Error::other(err.message)
    }
}
