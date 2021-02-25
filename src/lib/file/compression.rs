use std::path::Path;

pub async fn _compression<P: Clone + AsRef<Path>>(folder: P, file: P) {
    let created_file = std::fs::File::create(file.as_ref()).unwrap();

    let folder_file = _get_list_of_file(folder.as_ref().to_str().unwrap().to_string());

    let mut tar = tar::Builder::new(created_file);
    for i in folder_file {
        tar.append_path(i.clone()).unwrap();
    }

    let read_buf = std::fs::read(file.as_ref()).unwrap();

    let mut e = zstd::encode_all(read_buf.as_slice(), 0).unwrap();

    std::fs::write(file.as_ref(), &mut e).expect("Error");
}

pub fn _get_list_of_file(folder: String) -> Vec<String> {
    let read_dir = std::fs::read_dir(folder).unwrap();
    let mut result = Vec::new();
    for res in read_dir {
        let entry = res.unwrap();
        if entry.metadata().unwrap().is_dir() {
            result.append(&mut _get_list_of_file(
                entry.path().to_str().unwrap().to_string(),
            ));
        } else {
            result.push(entry.path().to_str().unwrap().to_string());
        }
    }
    result
}
