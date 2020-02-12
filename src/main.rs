fn main() {
    let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");
    let request_token = futures::executor::block_on(egg_mode::request_token(&con_token, "oob"));
//    let auth_url = egg_mode::authorize_url(&request_token);
//
//    let verifier = "123456";
//    let (token, user_id, screen_name) =
//        egg_mode::access_token(con_token, &request_token, verifier).await.unwrap();
}
