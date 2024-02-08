use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub id: String,
}

#[derive(Debug, PartialEq, Queryable, Selectable, Insertable, Identifiable, Associations)]
#[diesel(table_name = super::schema::keys)]
#[diesel(belongs_to(DbUser, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbAsymmetricKeyPair {
    pub id: Uuid,
    pub user_id: String,

    pub public_key: String,
    pub private_key_encrypted: String,
}
