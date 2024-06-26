use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;

pub fn write_to_tsv(outdir: &String,chrom: &String, coverages: &HashMap<u32, u32>, filename: &str) -> std::io::Result<()> {
    let filename = Path::new(&outdir).join(filename);
    let mut file = File::create(filename)?;
    for (key, value) in coverages.iter() {
        writeln!(file, "{}\t{}\t{}", chrom, key, value)?;
    }
    Ok(())
}
