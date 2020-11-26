pub async fn _get_appimage() {
    let ee = reqwest::Client::new()
        .get("https://dl.bintray.com/probono/AppImages/")
        .send()
        .await
        .expect("Error");
    let body = ee.text().await.unwrap();
    let vec: Vec<&str> = body.split(">").collect();
    let mut gvec: Vec<String> = Vec::new();
    let mut n = 0;
    for v in &vec {
        if v.contains("<a") {
            gvec.push(vec[n + 1].to_string().replace("</a", ""));
        }
        n += 1;
    }
    println!("{:#?}", gvec);
}
