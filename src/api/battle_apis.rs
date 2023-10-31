use actix_web::{web, get, post, delete, HttpResponse};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{models::battle::Battle, repository::database::Database};
use crate::models::monster::Monster;
use crate::repository::battle_repository;
use crate::repository::monster_repository;

#[derive(Serialize, Deserialize)]
pub struct CreateBattleRequest {
    monster_a: Option<String>,
    monster_b: Option<String>,
}

#[get("/battles")]
pub async fn get_battles(db: web::Data<Database>) -> HttpResponse {
    let battles = battle_repository::get_battles(&db);
    HttpResponse::Ok().json(battles)
}

#[cfg(test)]
mod tests {
    use actix_web::{test, http, App};
    use actix_web::web::Data;
    use crate::{
        utils::test_utils::init_test_battle,
        utils::test_utils::init_test_monsters
    };
    use serde_json;

    use super::*;

    #[actix_rt::test]
    async fn test_should_get_all_battles_correctly() {
        let db = Database::new();
        let app = App::new().app_data(Data::new(db)).service(get_battles);

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/battles").to_request();
        let resp = test::call_service(&mut app, req).await;
        
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_should_get_404_error_if_battle_does_not_exists() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_get_a_single_battle_correctly() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_delete_a_battle_correctly() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_delete_with_404_error_if_battle_does_not_exists() {
       //Todo
    }

    #[actix_rt::test]
    async fn test_should_create_a_battle_with_404_error_if_one_parameter_has_a_monster_id_does_not_exists() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_create_a_battle_with_a_bad_request_response_if_one_parameter_is_null() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_a_winning() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_b_winning() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_a_winning_if_theirs_speeds_same_and_monster_a_has_higher_attack() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_b_winning_if_theirs_speeds_same_and_monster_b_has_higher_attack() {
        //Todo
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_a_winning_if_theirs_defense_same_and_monster_a_has_higher_speed() {
        //Todo
    }
}

