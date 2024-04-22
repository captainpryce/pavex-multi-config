use pavex::http::StatusCode;

use crate::helpers::TestApi;

#[tokio::test]
async fn greet_happy_path() {
    let api = TestApi::spawn().await;

    let response = api
        .api_client
        .get(&format!("{}/big_data_api/product_info/5566778899", &api.api_address))
        .header("User-Agent", "Test runner")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), StatusCode::OK.as_u16());
    assert_eq!(response.text().await.unwrap(), "Hello, Ursula!");
}