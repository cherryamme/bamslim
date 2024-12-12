# bamslim
![bamslim](assets/bamslim2.png)
Use to sample BAM files by depth, keeping low-depth reads and dropping high-depth reads.

## Usage

```sh
bamslim -i '/path/to/input.bam' -s 2000 -o outdir
```

## Options

Usage: bamslim [OPTIONS] --input <INPUT>

Options:

- `-i`, `--input <INPUT>`: The path of input BAM file
- `-o`, `--outdir <OUTDIR>`: Output directory [default: bamslim_out]
- `-s`, `--slim-depth <SLIM_DEPTH>`: Slim depth under this value [default: 200]
- `-l`, `--log-record <LOG_RECORD>`: Process record log interval [default: 100000]
- `-h`, `--help`: Print help
- `-V`, `--version`: Print version
```
