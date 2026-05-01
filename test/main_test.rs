use axum_project::app::Model as User;
use serde_json::json;

// The base URL of your running application.
const BASE_URL: &str = "http://127.0.0.1:3000";

// We use a single test function to ensure the execution order.
// This mimics a Postman collection runner.
#[tokio::test]
async fn e2e_crud_test() {
    let client = reqwest::Client::new();

    // 1. CREATE a new user
    println!("Testing: POST /users");
    let create_response = client
        .post(format!("{}/users", BASE_URL))
        .json(&json!({ "name": "Charlie" }))
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_response.status(), 201, "Expected status CREATED");
    let created_user = create_response
        .json::<User>()
        .await
        .expect("Failed to parse create response");
    assert_eq!(created_user.name, "Charlie");
    println!("Success: Created user '{}' with id {}", created_user.name, created_user.id);

    let user_id = created_user.id;

    // 2. GET the user to verify creation
    println!("\nTesting: GET /users/{}", user_id);
    let get_response = client
        .get(format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .expect("Failed to send get request");

    assert_eq!(get_response.status(), 200, "Expected status OK");
    let fetched_user = get_response
        .json::<User>()
        .await
        .expect("Failed to parse get response");
    assert_eq!(fetched_user.id, user_id);
    assert_eq!(fetched_user.name, "Charlie");
    println!("Success: Fetched user '{}'", fetched_user.name);

    // 3. UPDATE the user
    println!("\nTesting: PUT /users/{}", user_id);
    let update_response = client
        .put(format!("{}/users/{}", BASE_URL, user_id))
        .json(&json!({ "name": "Charlie Brown" }))
        .send()
        .await
        .expect("Failed to send update request");

    assert_eq!(update_response.status(), 200, "Expected status OK");
    let updated_user = update_response
        .json::<User>()
        .await
        .expect("Failed to parse update response");
    assert_eq!(updated_user.name, "Charlie Brown");
    println!("Success: Updated user name to '{}'", updated_user.name);

    // 4. DELETE the user
    println!("\nTesting: DELETE /users/{}", user_id);
    let delete_response = client
        .delete(format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .expect("Failed to send delete request");

    assert_eq!(delete_response.status(), 204, "Expected status NO_CONTENT");
    println!("Success: Deleted user");

    // 5. VERIFY the user is deleted
    println!("\nTesting: GET /users/{} (should be 404)", user_id);
    let verify_response = client
        .get(format!("{}/users/{}", BASE_URL, user_id))
        .send()
        .await
        .expect("Failed to send verify request");

    assert_eq!(verify_response.status(), 404, "Expected status NOT_FOUND");
    println!("Success: User not found, as expected");
}

#[tokio::test]
async fn e2e_create_test() {
    let client = reqwest::Client::new();

    // 1. CREATE a new user
    println!("Testing: POST /users");
    let create_response = client
        .post(format!("{}/users", BASE_URL))
        .json(&json!({ "name": "Charlie" }))
        .send()
        .await
        .expect("Failed to send create request");

    assert_eq!(create_response.status(), 201, "Expected status CREATED");
    let created_user = create_response
        .json::<User>()
        .await
        .expect("Failed to parse create response");
    assert_eq!(created_user.name, "Charlie");
    println!("Success: Created user '{}' with id {}", created_user.name, created_user.id);
    println!("now open your google chrome to view users at http://localhost:3000/users")
}