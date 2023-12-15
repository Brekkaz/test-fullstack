use super::{database::Database, schema::battles::dsl::battles};
use crate::models::battle::Battle;
use diesel::{QueryDsl, RunQueryDsl};

pub fn get_battles(db: &Database) -> Vec<Battle> {
    let mut connection = db.get_connection();
    battles
        .load::<Battle>(&mut connection)
        .expect("Error loading all battles")
}

pub fn create_battle(db: &Database, battle: Battle) -> Result<Battle, diesel::result::Error> {
    let mut connection = db.get_connection();
    let battle = Battle {
        id: uuid::Uuid::new_v4().to_string(),
        created_at: None,
        updated_at: None,
        ..battle
    };
    diesel::insert_into(battles)
        .values(&battle)
        .execute(&mut connection)
        .expect("Error creating a new battle");
    Ok(battle)
}

pub fn get_battle_by_id(db: &Database, battle_id: &str) -> Option<Battle> {
    let mut connection = db.get_connection();
    match battles
        .find(battle_id)
        .get_result::<Battle>(&mut connection)
    {
        Ok(battle) => Some(battle),
        Err(_) => None,
    }
}

pub fn delete_battle_by_id(db: &Database, battle_id: &str) -> Option<usize> {
    let mut connection = db.get_connection();

    if let Ok(_existing_battle) = battles
        .find(battle_id)
        .get_result::<Battle>(&mut connection)
    {
        let count = diesel::delete(battles.find(battle_id))
            .execute(&mut connection)
            .expect("Error deleting battle by id");

        Some(count)
    } else {
        None
    }
}
