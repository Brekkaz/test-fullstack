use super::battle_apis::{create_battle, delete_battle_by_id, get_battles};
use super::monster_apis::{
    create_monster, delete_monster_by_id, get_monster_by_id, get_monsters, import_csv,
    update_monster_by_id,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_monsters)
            .service(create_monster)
            .service(get_monster_by_id)
            .service(delete_monster_by_id)
            .service(update_monster_by_id)
            .service(import_csv)
            .service(get_battles)
            .service(create_battle)
            .service(delete_battle_by_id),
    );
}

#[cfg(test)]
mod tests {
    use super::config;
    use crate::repository::database::Database;
    use actix_web::web::Data;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_should_get_all_battles_correctly() {
        let db = Database::new();
        let mut app =
            test::init_service(App::new().app_data(Data::new(db)).configure(config)).await;
        let request = test::TestRequest::get().uri("/api/battles").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());
    }
}
