// auth.rs

use jwt::{decode, Validation, Algorithm};

/* 
    Claims struct, for JWT
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64
}

/* 
    Authorization function, checks access when calling endpoint
 */
pub fn jwt_authorization(token: &str) {
    let secret = "123";
    let token_data = match decode::<Claims>(&token.to_string(), secret.as_ref(), &Validation::new(Algorithm::HS512)) {
        Ok(c) => c,
        Err(err) => panic!("Invalid token")
    };
}               