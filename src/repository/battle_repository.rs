use diesel::prelude::*;
use crate::models::battle::Battle;
use crate::repository::schema::battles::dsl::*;
use crate::repository::database::Database;

pub fn get_battles(db: &Database) -> Vec<Battle> {
    let mut connection = db.get_connection();
    battles
        .load::<Battle>(&mut connection)
        .expect("Error loading all battles")
}
