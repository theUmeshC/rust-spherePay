use tokio;

#[tokio::main]
async fn main() {
    use anyhow::Result;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use tracing::instrument;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomerResponse {
        pub customer: Customer,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Address {
        pub line1: String,
        pub line2: String,
        pub city: String,
        pub state: String,
        pub country: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Customer {
        pub id: String,
        #[serde(rename = "type")]
        pub customer_type: String,
        pub address: Address,
        pub email: String,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(rename = "phoneNumber")]
        pub phone_number: String,
        pub kyc: String,
        #[serde(rename = "kycRequirements")]
        pub kyc_requirements: Vec<String>,
        pub tos: String,
        pub wallets: Vec<String>,
        #[serde(rename = "bankAccounts")]
        pub bank_accounts: Vec<String>,
        pub created: String,
        pub updated: String,
        pub mock: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SphereResponse<T> {
        pub ok: bool,
        pub object: String,
        pub message: String,
        pub error: Option<String>,
        pub data: T,
        pub ts: String,
        pub request: String,
    }

    #[instrument]
    pub async fn create_customer(
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_number: &str,
        dob_day: &i64,
        dob_month: &i64,
        dob_year: &i64,
        ssn: &str,
        address_line_1: &str,
        address_line_2: &str,
        address_city: &str,
        address_state: &str,
        address_country: &str,
        address_postal_code: &str,
        mock: bool,
    ) -> Result<Customer> {
        let client = Client::new();
        let mock_str = if mock { "true" } else { "false" };
        let url = &format!("https://api.spherepay.co/v1/customer?mock={}", mock_str);
        let resp = client
            .post(url)
            .header(
                "Authorization",
                "Bearer ",
            )
            .json(&serde_json::json!({
                "type": "individual",
                "firstName": first_name,
                "lastName": last_name,
                "email": email,
                "phoneNumber": phone_number,
                "dob": &serde_json::json!({
                    "day": dob_day,
                    "month": dob_month,
                    "year": dob_year,
                }),
                "ssn": ssn,
                "address": &serde_json::json!({
                    "line1": address_line_1,
                    "line2": address_line_2,
                    "city": address_city,
                    "state": address_state,
                    "country": address_country,
                    "postalCode": address_postal_code,
                }),
            }))
            .send()
            .await?;
        let response_body = resp.text().await?;

        let response: SphereResponse<CustomerResponse> = serde_json::from_str(&response_body)
            .expect(&format!("Failed to deserialize response {:?}", response_body).to_string());

        let customer = response.data.customer;
        Ok(customer)
    }
    // match create_customer(
    //     "Usdsh",
    //     "cdx",
    //     "umesh@cel.com",
    //     "+1234567890",
    //     &1,
    //     &1,
    //     &1990,
    //     "123-45-6789",
    //     "123 Main St",
    //     "Apt 45",
    //     "Cityville",
    //     "CA",
    //     "USA",
    //     "12345",
    //     true
    // )
    // .await
    // {
    //     Ok(customer) => {
    //         println!("Successfully created customer: {:?}", customer);
    //         // Add further processing or error handling as needed
    //     }
    //     Err(err) => {
    //         eprintln!("Error creating customer: {:?}", err);
    //         // Handle the error
    //     }
    // }
    use reqwest::header::AUTHORIZATION;
    use reqwest::multipart;
    use reqwest::Body;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FilesResponse {
        pub files: Vec<File>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct File {
        pub id: String,
        #[serde(rename = "type")]
        pub file_type: String,
        pub name: String,
        pub description: String,
        pub url: String,
        pub mock: bool,
    }
    pub async fn create_file(mock: bool) -> Result<Vec<File>> {
        let client = Client::new();
        let empty_buffer = Vec::new();
        let body = Body::from(empty_buffer);
        let form = multipart::Form::new()
        .text("customer", "customer_70c0e905e3474ceab0d0c4cea8b7b859")
        .text("type", "customerIdentityDocument") // or "customerUboDocument", "customerIncorporationDocument"
        .text("name", "My Customer's Passport")
        .text("description", "A document showing the information of the customer")
        .text("links", "https://images.wsj.net/im-889295?width=607&height=405,https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf")
        .part("file", multipart::Part::stream(body).file_name("emptyBuffer").mime_str("application/octet-stream")?);
        let mock_str = if mock { "true" } else { "false" };
        let url = &format!("https://api.spherepay.co/v1/file?mock={}", mock_str);
        let resp = client
            .post(url)
            .header(
                AUTHORIZATION,
                "Bearer ",
            )
            .multipart(form)
            .send()
            .await?;
        let response_body = resp.text().await?;
        println!("Successfully created customer: {:?}", response_body);
        let response: SphereResponse<FilesResponse> =
            serde_json::from_str(&response_body).expect("Failed to deserialize response");
        let files = response.data.files;
        Ok(files)
    }
    // pub async fn create_file(
    //     mock: bool,
    // ) -> Result<Vec<File>> {
    //     let client = Client::new();
    //     let mut file = fs::File::open("file:///C:/Users/umesh.c/Downloads/applicant-summary-65af8897db4fda0bff8230f5.pdf").map_err(|e| anyhow::Error::new(e))?;
    //     let mut file_contents = Vec::new();
    //     file.read_to_end(&mut file_contents)
    //         .map_err(|e| anyhow::Error::new(e))?;
    //     let form = multipart::Form::new()
    //         .text("customer", "customer_70c0e905e3474ceab0d0c4cea8b7b859")
    //         .text("type", "customerIdentityDocument")
    //         .text("name", "My Customer's Passport")
    //         .text(
    //             "description",
    //             "A document showing the information of the customer",
    //         )
    //         .part(
    //             "file",
    //             multipart::Part::bytes(file_contents)
    //                 .file_name("applicant-summary-65af8897db4fda0bff8230f5.pdf")
    //                 .mime_str("application/pdf")?,
    //         );
    //     let url = &format!(
    //         "https://api.spherepay.co/v1/file?mock={}",
    //         if mock { "true" } else { "false" }
    //     );
    //     let resp = client
    //         .post(url)
    //         .header("Authorization", "Bearer ")
    //         .multipart(form)
    //         .send()
    //         .await?;
    //     let response_body = resp.text().await?;
    //     let response: SphereResponse<FilesResponse> = serde_json::from_str(&response_body)
    //         .expect(&format!("Failed to deserialize response {:?}", response_body).to_string());
    //     let files = response.data.files;
    //     Ok(files)
    // }
    match create_file(true).await {
        Ok(file) => {
            println!("Successfully created file: {:?}", file);
        }
        Err(err) => {
            eprintln!("Error creating customer: {:?}", err);
            // Handle the error
        }
    }
}
