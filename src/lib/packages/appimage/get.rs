pub async fn _get_appimage() {
    let ee = reqwest::Client::new()
        .get("https://dl.bintray.com/probono/AppImages/")
        .send()
        .await
        .expect("Error");
    let body = ee.text().await.unwrap();
    let vec: Vec<&str> = body.split('>').collect();
    let mut gvec: Vec<String> = Vec::new();
    for (n, v) in vec.iter().enumerate() {
        if v.contains("<a") {
            gvec.push(vec[n + 1].to_string().replace("</a", ""));
        }
    }
    println!("{:#?}", gvec);
}
