use diesel::result::Error;

use crate::utils::connections::*;

pub trait CRUD<CreatedModel, UpdateModel, PK> {
    fn create(conn: DB, from: &CreatedModel) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(conn: DB) -> Vec<Self>
    where
        Self: Sized;

    fn update(conn: DB, pk: PK, value: &UpdateModel) -> Result<Self, Error>
    where
        Self: Sized;

    fn delete(conn: DB, pk: PK) -> Result<usize, Error>
    where
        Self: Sized;

    fn get_by_pk(conn: DB, pk: PK) -> Result<Self, Error>
    where
        Self: Sized;
}