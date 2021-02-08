pub async fn sync(url: String) -> Result<(), String> {
    println!("{}", url);
    let e = reqwest::Client::new().get(&format!("{}/package.json", url));
    let a = e.send().await.unwrap();
    println!("{:?}", a);
    Ok(())
}
