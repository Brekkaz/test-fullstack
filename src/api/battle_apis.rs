use crate::repository::battle_repository;
use crate::repository::monster_repository;
use crate::{models::battle::Battle, repository::database::Database};
use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[post("/battles")]
pub async fn create_battle(
    db: web::Data<Database>,
    mut new_battle: web::Json<Battle>,
) -> HttpResponse {
    //validate formats
    if !Uuid::parse_str(&new_battle.monster_a).is_ok() {
        return HttpResponse::NotFound().json("Monster a not found");
    }
    if !Uuid::parse_str(&new_battle.monster_b).is_ok() {
        return HttpResponse::NotFound().json("Monster b not found");
    }
    //validate if exist
    let monster_a = match monster_repository::get_monster_by_id(&db, &new_battle.monster_a) {
        Some(m) => m,
        None => return HttpResponse::NotFound().json("Monster a not found"),
    };
    let monster_b = match monster_repository::get_monster_by_id(&db, &new_battle.monster_b) {
        Some(m) => m,
        None => return HttpResponse::NotFound().json("Monster b not found"),
    };
    //sets turn order
    let (mut first_monster, mut second_monster) = if monster_a.speed > monster_b.speed {
        (monster_a, monster_b)
    } else if monster_a.speed < monster_b.speed {
        (monster_b, monster_a)
    } else {
        if monster_a.attack > monster_b.attack {
            (monster_a, monster_b)
        } else {
            (monster_b, monster_a)
        }
    };
    //battle
    while first_monster.hp > 0 && second_monster.hp > 0 {
        //first monster attack
        let mut damage = match first_monster.attack - second_monster.defense {
            diff if diff <= 0 => 1,
            diff => diff,
        };
        second_monster.hp = second_monster.hp - damage;
        if second_monster.hp <= 0 {
            new_battle.winner = first_monster.id.to_string();
            break;
        }
        //second monster attack
        damage = match second_monster.attack - first_monster.defense {
            diff if diff <= 0 => 1,
            diff => diff,
        };
        first_monster.hp = first_monster.hp - damage;
        if first_monster.hp <= 0 {
            new_battle.winner = second_monster.id.to_string();
            break;
        }
    }
    //save battle
    let battle = battle_repository::create_battle(&db, new_battle.into_inner());
    match battle {
        Ok(battle) => HttpResponse::Created().json(battle),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/battles/{id}")]
pub async fn get_battle_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    if !Uuid::parse_str(&id).is_ok() {
        return HttpResponse::NotFound().json("Battle not found");
    }
    let battle = battle_repository::get_battle_by_id(&db, &id);
    match battle {
        Some(battle) => HttpResponse::Ok().json(battle),
        None => HttpResponse::NotFound().json("Battle not found"),
    }
}

#[delete("/battles/{id}")]
pub async fn delete_battle_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    if !Uuid::parse_str(&id).is_ok() {
        return HttpResponse::NotFound().json("Battle not found");
    }
    let battle = battle_repository::delete_battle_by_id(&db, &id);
    match battle {
        Some(_) => HttpResponse::NoContent().finish(),
        None => HttpResponse::NotFound().json("Battle not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::{create_battle, delete_battle_by_id, get_battle_by_id, get_battles};
    use crate::models::battle::Battle;
    use crate::repository::database::Database;
    use crate::utils::test_utils::{init_test_battle, init_test_monsters};
    use actix_web::{http, test, web::Data, App};
    use serde_json::{self, json};
    use uuid::Uuid;

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
        let app = App::new().service(delete_battle_by_id);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/battles/999999").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_should_get_a_single_battle_correctly() {
        let mut db = Database::new();
        let test_battles = init_test_battle(&mut db).await;
        let app = App::new().app_data(Data::new(db)).service(get_battle_by_id);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get()
            .uri(format!("/battles/{}", test_battles[0].id).as_str())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_should_delete_a_battle_correctly() {
        let mut db = Database::new();
        let _test_battles = init_test_battle(&mut db).await;
        let app = App::new()
            .app_data(Data::new(db))
            .service(delete_battle_by_id);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::delete()
            .uri(format!("/battles/{}", _test_battles[0].id).as_str())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_rt::test]
    async fn test_should_create_a_battle_with_404_error_if_one_parameter_has_a_monster_id_does_not_exists(
    ) {
        let db = Database::new();
        let app = App::new().app_data(Data::new(db)).service(create_battle);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&json!({
                "monster_a":Uuid::default().to_string(),
                "monster_b":Uuid::default().to_string()
            }))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_should_create_a_battle_with_a_bad_request_response_if_one_parameter_is_null() {
        let db = Database::new();
        let app = App::new().app_data(Data::new(db)).service(create_battle);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&json!({
                "monster_a":Uuid::default().to_string(),
            }))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_a_winning() {
        let mut db = Database::new();
        let test_battles = init_test_battle(&mut db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&json!({
                "monster_a": test_battles[0].monster_a.clone(),
                "monster_b": test_battles[0].monster_b.clone(),
            }))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let battle_response: Battle = serde_json::from_slice(&test::read_body(resp).await)
            .expect("Failed to deserialize JSON");
        assert_eq!(battle_response.winner, test_battles[0].monster_b);
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_b_winning_if_theirs_speeds_same_and_monster_b_has_higher_attack(
    ) {
        let mut db = Database::new();
        let test_monsters = init_test_monsters(&mut db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&json!({
                "monster_a": test_monsters[4].id.clone(),
                "monster_b": test_monsters[1].id.clone(),
            }))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let battle_response: Battle = serde_json::from_slice(&test::read_body(resp).await)
            .expect("Failed to deserialize JSON");
        debug_assert!(
            test_monsters[4].speed == test_monsters[1].speed
                && test_monsters[1].attack > test_monsters[4].attack,
            "monster_a = {:#?}, monster_a = {:#?}",
            test_monsters[4],
            test_monsters[1]
        );
        assert_eq!(battle_response.winner, test_monsters[1].id);
    }
}
