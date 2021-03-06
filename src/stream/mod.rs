//! Types which operate over [`Stream`](futures::stream::Stream)`<Item =
//! `[`io::Result`](std::io::Result)`<`[`Bytes`](bytes::Bytes)`>>` streams, both encoders and
//! decoders for various formats.
//!
//! The `Stream` is treated as a single byte-stream to be compressed/decompressed, each item is a
//! chunk of data from this byte-stream. There is not guaranteed to be a one-to-one relationship
//! between chunks of data from the underlying stream and the resulting compressed/decompressed
//! stream, the encoders and decoders will buffer the incoming data and choose their own boundaries
//! at which to yield a new item.

#[cfg(feature = "brotli")]
mod brotli;
#[cfg(feature = "deflate")]
mod deflate;
#[cfg(any(feature = "deflate", feature = "gzip", feature = "zlib"))]
mod flate;
#[cfg(feature = "gzip")]
mod gzip;
#[cfg(feature = "zlib")]
mod zlib;
#[cfg(feature = "zstd")]
mod zstd;

#[cfg(feature = "brotli")]
pub use self::brotli::{BrotliDecoder, BrotliEncoder};
#[cfg(feature = "deflate")]
pub use self::deflate::{DeflateDecoder, DeflateEncoder};
#[cfg(feature = "gzip")]
pub use self::gzip::{GzipDecoder, GzipEncoder};
#[cfg(feature = "zlib")]
pub use self::zlib::{ZlibDecoder, ZlibEncoder};
#[cfg(feature = "zstd")]
pub use self::zstd::{ZstdDecoder, ZstdEncoder};
