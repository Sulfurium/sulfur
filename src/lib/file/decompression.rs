pub async fn decompression(from: String, to: String) -> bool {
    let mut decoder = {
        let file = std::fs::File::open(from.clone()).unwrap();
        zstd::Decoder::new(file).unwrap()
    };

    let name = from.trim_end_matches(".zst");
    let mut target = std::fs::File::create(name).unwrap();

    std::io::copy(&mut decoder, &mut target).unwrap();
    let e = async_std::fs::File::open(name).await.unwrap();
    println!("{:?}", e);
    let ar = async_tar::Archive::new(e);
    ar.unpack(to).await.unwrap();

    true
}
