use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "fortune")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub message: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}