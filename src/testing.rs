#[cfg(test)]

mod tests {
    use super::*;
    use axum::Json;
    use serde_json::json;
    use crate::web::routes_signup::api_signup;
    use crate::strs::LoginPayload;
    use crate::web::routes_login::api_login;
    use crate::strs::SignupPayload;

    #[tokio::test]
    async fn test_api_signup_success() {


    
        let payload = SignupPayload {
            username: "newuser2".to_string(),
            password: "password123".to_string(),
            email: "newuser2@example.com".to_string(),
            first_name: "Johny".to_string(),
            last_name: "Doee".to_string(),
            address: "123 Test St".to_string(),
            city: "Test City".to_string(),
            state: "Test State".to_string(),
            zipcode: "12345".to_string(),
            country: "Testland".to_string(),
            phonenumber: "123-456-7880".to_string(),
        };

        
        let result = api_signup(Json(payload)).await;

        // Check if the result is successful
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0["message"], "Signup Success");

        // Check that the user is actually inserted into the database
  
    }

    #[tokio::test]
    async fn test_login(){
        let payload = LoginPayload {
            username: "amir_hosseini98".to_string(),
            password: "AmirSecure!456".to_string(),
        };
       // let pool = Connection::open("database.db").unwrap();
        // let result = api_login(pool,Json(payload)).await;
        // assert!(result.is_ok());
       //  assert_eq!(result.unwrap().0["access_token"], "access_token");
    }











  }
    
