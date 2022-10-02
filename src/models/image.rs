use diesel::{MysqlConnection, OptionalExtension, Queryable, RunQueryDsl};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub url: String,
}

// Static function
impl Image {
    // Now we can call this function like this:
    // let res = Image::find_by_id(1, &conn);
    pub fn find_by_id(_id: i32, conn: &MysqlConnection) -> Result<Option<Image>, Error> {
        use crate::schema::images::dsl::*;

        let res = images
            .filter(id.eq(_id))
            .first::<Image>(conn)
            .optional()?;

        Ok(res)
    }

    pub fn count(conn: &MysqlConnection) -> Result<i32, Error> {
        use crate::schema::images::dsl::*;

        let res = images
            .count()
            .get_result::<i64>(conn)?;

        Ok(res.try_into().unwrap())
    }
}
