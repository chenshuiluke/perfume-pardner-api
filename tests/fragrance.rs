use dotenvy::dotenv;
use perfume_pardner_api::{app, db};
#[cfg(test)]
mod fragrance {
    use actix_web::{body, http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_fragrance_endpoint() {
        dotenv().ok();
        db::init_db_pool().await.unwrap();
        app::fragrance::service::load_fragrances_from_json_to_db().await;
        let app =
            test::init_service(App::new().configure(app::fragrance::routes::route_config)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::json())
            .uri("/fragrances?query=creed")
            .to_request();
        let resp = test::call_service(&app, req).await;
        let bytes = body::to_bytes(resp.into_body()).await.unwrap();
        print!("@@@{:?}", std::str::from_utf8(&bytes));
        // assert!(resp.status().is_success());
    }
}
