use pavex::response::Response;
use pavex::http::StatusCode;
use pavex::request::path::PathParams;
use crate::routes::integrations::big_data_api::bigdata_client::BigDataClient;

struct ProductInfo {
    upc: String,
    title: String,
    description: String,
    price: f64,
    retail_price: f64,
    vendor: String,
    categories: Vec<String>,
    tags: Vec<String>,
    images: Vec<String>,
}

// Get Product Data Path Parameters
#[PathParams]
pub struct GetProductDataParams {
    upc: String,
}
// TODO:: To fix the compile time error for this method we need to reference the Path parameters correctly using attribute linking.
pub async fn get_product_data(params: PathParams<GetProductDataParams>, client: BigDataClient) -> Response {
    // Create a new HTTP request
    let GetProductDataParams { upc } = params.0;
    let url = "gtin/".to_string() + &*upc;
    let config = client.config.big_data_config();
    // Print the values
    println!("URL: {}", url);
    println!("RapidAPI Key: {}", config.rapid_api_key);
    println!("RapidAPI Host: {}", config.rapid_api_host);

    let response = client.client.get(&url)
        .header("X-RapidAPI-Key", &config.rapid_api_key)
        .header("X-RapidAPI-Host", &config.rapid_api_host)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let response_body = response.text().await.unwrap();
        println!("Response: {}", response_body);
        Response::new(StatusCode::OK)
    } else {
        println!("Request failed: {}", response.status());
        Response::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
impl ProductInfo {
    // constructor function accepts a UPC values as a parameter. If given, it will initialize the ProductInfo struct
    pub fn new(upc: String) -> Self {
        Self {
            upc,
            title: "".to_string(),
            description: "".to_string(),
            price: 0.0,
            retail_price: 0.0,
            vendor: "".to_string(),
            categories: vec![],
            tags: vec![],
            images: vec![],
        }
    }
}