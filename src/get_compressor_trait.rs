use crate::compressor::Compression;

/// `GetCompressor` trait provides a unified interface for Session to get a compressor
/// for further decompressing received data.
pub trait GetCompressor<'a> {
  /// Returns actual compressor.
  fn get_compressor(&self) -> Compression;
}
