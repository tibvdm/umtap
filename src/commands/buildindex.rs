//! The `umgap buildindex` command.

use std::io;

use crate::errors;

/// The `umgap buildindex` command takes tab-separated strings and taxon IDs, and creates a
/// searchable FST index of this mapping.
///
/// The input is given on *standard input*. It should be in a TSV format with two columns, ordered
/// by the first. The unique strings in the first column are to be mapped to the integers (taxon
/// IDs) in the second column. A binary file with a compressed mapping is written to *standard
/// output*.
///
///     $ cat input.tsv
///     AAAAA	2759
///     BBBBBB	9153
///     $ umgap buildindex < input.tsv > tiny.index
///     $ umgap printindex tiny.index
///     AAAAA	2759
///     BBBBBB	9153
#[derive(Debug, StructOpt)]
pub struct BuildIndex {}

/// Implements the buildindex command
pub fn buildindex(_args: BuildIndex) -> errors::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(io::stdin());

    let mut index = fst::MapBuilder::new(io::stdout())?;

    for record in reader.deserialize() {
        let (kmer, lca): (String, u64) = record?;
        index.insert(kmer, lca)?;
    }

    index.finish()?;

    Ok(())
}
