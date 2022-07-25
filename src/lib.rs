//! > **A partial safe API to libBigWig**
//!
//! All methods are modeled on the library examples in c found here:
//!
//! <https://github.com/dpryan79/libBigWig/blob/09a1b9211019f1cca667feae8c77229ae78d1d30/test/exampleWrite.c>
//!
//! # Example
//!
//! ```
//! use rust_libbigwig::BigWigWriter;
//!
//! let bw_filename = "test.bw";
//! let chrom_names: Vec<String> = vec!["chr1".to_string(), "chr2".to_string()];
//! let chrom_len = vec![100, 200];
//! let mut bigwig_writer = BigWigWriter::new(bw_filename, &chrom_names, &chrom_len).unwrap();
//!
//! let mut start_pos: Vec<u32> = Vec::new();
//! let mut score: Vec<f32> = Vec::new();
//! start_pos.push(10);
//! score.push(0.5);
//! start_pos.push(20);
//! score.push(0.1);
//!
//! bigwig_writer.add_interval_spans("chr1", &mut start_pos, 1, &mut score).unwrap();
//! ```

mod bw_writer;
mod inc_bindings;

pub use bw_writer::BigWigWriter;
