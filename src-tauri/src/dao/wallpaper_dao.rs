use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel::associations::HasTable;
use diesel::connection::SimpleConnection;
use crate::dao::db;

use crate::dao::models::{Bing, NewBing, Page};
use crate::dao::schema::bing::{created_date, source};
use crate::dao::schema::bing::dsl::bing;

pub(crate) fn find_all(current_page:i64, other_source: &str) -> anyhow::Result<Page> {
    let mut connection = db::establish_db_connection();
    let total = bing.count().filter(source.eq(other_source)).get_result::<i64>(&mut connection);
    let _offset: i64 = (current_page - 1) * 60;
    let record = bing.select(Bing::as_select()).filter(source.eq(other_source)).limit(60).offset(_offset).order_by(created_date.desc()).load(&mut connection);
        
    if let (Ok(t), Ok(r)) = (total, record){
        return Ok(Page{
            data: r,
            totals: t,
            current_page,
        })
    }

    Ok(Page{
        data: vec![],
        totals: 0,
        current_page,
    })
}

pub(crate) fn insert(wallpaper_vec: Vec<NewBing>) -> bool {
    if wallpaper_vec.len() == 0 {
        return false;
    }

    let mut connection = db::establish_db_connection();

    let wallpaper_all: Vec<Bing> = bing
        .select(Bing::as_select())
        .load(&mut connection)
        .expect("Error loading posts");

    let need_wallpaper_vec = wallpaper_vec.iter().filter(|item| wallpaper_all.iter().any(|x| x.url == item.url) == false).map(|x|x.clone()).collect::<Vec<_>>();
    let mut flag = true;
    if need_wallpaper_vec.len() < wallpaper_vec.len() {
        flag = false;
    }

    diesel::insert_into(bing::table()).values(&need_wallpaper_vec).execute(&mut connection).unwrap();

    return flag;

}




#[cfg(test)]
mod tests {
    use crate::dao::wallpaper_dao::*;

    #[test]
    fn find_all_test(){
        let result = find_all(1,"bing").unwrap();

        println!("{:#?}", result);
    }
}