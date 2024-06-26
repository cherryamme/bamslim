use rust_htslib::{bam, bam::Read};
use log::info;
use std::collections::HashMap;
use std::path::Path;

pub fn read_bam_file(path: &str) -> (bam::Reader, bam::Header) {
	info!("Reading bam file: {}", path);
	let bam = bam::Reader::from_path(path).unwrap();
	let header = bam::Header::from_template(bam.header());
	(bam, header)
}


pub fn write_bam_file(outdir: &str, header: &bam::Header) -> bam::Writer {
	let filepath = Path::new(&outdir).join(format!("bamslim_out.bam"));
	info!("Writing bam file: {:?}", filepath);
	bam::Writer::from_path(filepath, header, bam::Format::Bam).unwrap()
}


pub fn get_bam_chrom(bam: &bam::Reader) -> HashMap<u32, String>{
	let mut chrom_map = HashMap::new();
	let header = bam::Header::from_template(bam.header());
	let headerview = bam::HeaderView::from_header(&header);
	// 使用header的target_name方法将tid转为染色体名称
	for i in 0..headerview.target_count() {
		let chrom = std::str::from_utf8(headerview.tid2name(i as u32)).unwrap().to_string();
		info!("find contig: {}", chrom);
		chrom_map.insert(i, chrom);
		// let chrom = std::str::from_utf8(headerview.tid2name(tid)).expect("can't change to chrname").to_string();
	}
	chrom_map
}



pub fn slim_bam(log_record: u32, chrom_map: HashMap<u32, String>, mut bam: bam::Reader, mut outbam: bam::Writer, slim_depth: u32) -> (Box<HashMap<String, HashMap<u32, u32>>>, Box<HashMap<String, HashMap<u32, u32>>>) {
	info!("slimming bam file...");
	let mut raw_coverages:  Box<HashMap<String, HashMap<u32, u32>>> =  Box::new(HashMap::new());
	let mut fix_coverages:  Box<HashMap<String, HashMap<u32, u32>>> =  Box::new(HashMap::new());
	let mut count = 0;
	let mut record;
	let mut chrom;
	let mut start;
	let mut end;
	let mut drop_reads;
	let last_time = std::time::Instant::now();
	let mut elapsed;
	for r in bam.records() {
			record = r.expect("Fail to read record");
			chrom = chrom_map.get(&(record.tid() as u32)).unwrap_or(&"unknown".to_string()).to_string();
			// chrom = get_bam_chrom(record.tid() as u32, &header);
			// // debug!("record:{:?}", record);
			start = record.pos() as u32;
			end = record.cigar().end_pos() as u32;
			drop_reads = true;
			count += 1;
			if count % log_record == 0 {
				elapsed = last_time.elapsed();
				info!("{} reads processed. Speed: {:.2?} record/s", count, log_record as f64 / elapsed.as_secs_f64());
			}
			for position in start..end {
				if drop_reads {
					let depth = *raw_coverages.entry(chrom.clone()).or_insert_with(|| HashMap::with_capacity(10000000)).entry(position).or_insert(0);
					if depth <= slim_depth {
						drop_reads = false;
	
					};
				}
				*raw_coverages.entry(chrom.clone()).or_insert_with(|| HashMap::with_capacity(10000000)).entry(position).or_insert(0) += 1;
			}
			// debug!("{}-{} drop_reads:{}",start, end, drop_reads);
			if drop_reads {
				continue;
			}else {
				for position in start..end {
					*fix_coverages.entry(chrom.clone()).or_insert_with(|| HashMap::with_capacity(10000000)).entry(position).or_insert(0) += 1;
				}
				outbam.write(&record).unwrap();
			}

		}
	info!("Slim bam done. Total reads count:{}", count);
	(raw_coverages, fix_coverages)
}


#[test]
fn test_bam_reader() {

}



#[test]
fn test_pileup(){
	let mut bam = bam::Reader::from_path(&"example/alpha_23B06506676_hap0.bam").unwrap();
	for p in bam.pileup() {
		let pileup = p.unwrap();
		println!("{}:{} depth {}", pileup.tid(), pileup.pos(), pileup.depth());
	
		for alignment in pileup.alignments() {
			if !alignment.is_del() && !alignment.is_refskip() {
				println!("Base {}", alignment.record().seq()[alignment.qpos().unwrap()]);
			}
			// mark indel start
			match alignment.indel() {
				bam::pileup::Indel::Ins(len) => println!("Insertion of length {} between this and next position.", len),
				bam::pileup::Indel::Del(len) => println!("Deletion of length {} between this and next position.", len),
				bam::pileup::Indel::None => ()
			}
		}
	}
}



#[test]
fn test_ref_name(){
	let bam = bam::Reader::from_path("/home/jiangchen/project/bamslim/example/barcode11.bam").unwrap();
	let header = bam::Header::from_template(bam.header());
	let headerview = bam::HeaderView::from_header(&header);
	// 使用header的target_name方法将tid转为染色体名称
	for i in 0..headerview.target_count() {
		let chrom = std::str::from_utf8(headerview.tid2name(i as u32)).unwrap().to_string();
		println!("{}", chrom);
		// let chrom = std::str::from_utf8(headerview.tid2name(tid)).expect("can't change to chrname").to_string();
	}
// if tid >=0 {
// 	let chrom = std::str::from_utf8(headerview.tid2name(tid)).unwrap().to_string();
	// println!("{}", chrom);
	// chrom
}
