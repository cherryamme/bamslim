
mod bam;
mod args;
mod plot;
mod writer;
use log::info;
use clap::Parser;
use std::fs;


fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let comands: Vec<String> = std::env::args().collect();
    let args = args::Args::parse();
    info!("Run Command: {:?}", comands);
    let start_time = std::time::Instant::now();
	let  (bam,header) = bam::read_bam_file(&args.input);

    fs::create_dir_all(&args.outdir).unwrap();
	let outbam =bam::write_bam_file(&args.outdir, &header);
    let chrom_map = bam::get_bam_chrom(&bam);
	let (raw_coverages, fix_coverages) = bam::slim_bam(args.log_record, chrom_map, bam, outbam, args.slim_depth);
	// debug!("raw_coverages:{:?}", raw_coverages);
	// plot::plotter_multi_coverages(raw_coverages, fix_coverages);
    let mut elapsed_time = start_time.elapsed();
    info!("Slim Time elapsed: {:.2?}",elapsed_time);
    info!("Starting plot and write info...");
    for (chrom, coverages) in &*raw_coverages {
		plot::plotter_coverages(&args.outdir, chrom.clone(), &coverages, &fix_coverages.get(chrom).unwrap()).expect("plotting err");
        writer::write_to_tsv(&args.outdir, chrom, &coverages, "raw_coverage.tsv").expect("write raw_coverage.tsv err");
        writer::write_to_tsv(&args.outdir, chrom, &fix_coverages.get(chrom).unwrap(), "fix_coverage.tsv").expect("write fix_coverage.tsv err");
	}
    elapsed_time = start_time.elapsed();
    info!("Done. Time elapsed: {:.2?}",elapsed_time);
}
