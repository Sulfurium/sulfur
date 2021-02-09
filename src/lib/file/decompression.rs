use std::path::Path;

pub async fn _decompression<P: AsRef<Path>>(from: String, to: P) {
    /* ZSTD DECOMPRESSION */
    let mut decoder = {
        let file = std::fs::File::open(from.clone()).unwrap();
        zstd::Decoder::new(file).unwrap()
    };

    let name = from.trim_end_matches(".zst");
    let mut target = std::fs::File::create(name).unwrap();

    std::io::copy(&mut decoder, &mut target).unwrap();
    
    /* TAR UNARCHIVAGE */

    let e = std::fs::File::open(name).unwrap();
    let mut ar = tar::Archive::new(e);
    ar.unpack(to).unwrap();
}
