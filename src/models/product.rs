use diesel::PgConnection;
use crate::schema::products;

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>
}

use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::schema::products::dsl;

impl ProductList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = 
            products::table
                .limit(10)
                .load::<Product>(connection)
                .expect("Error loading products");

        ProductList(result)
    }
}

impl NewProduct {
    pub fn create(&self, connection: &PgConnection) -> Result<Product, diesel::result::Error> {
        diesel::insert_into(products::table)
            .values(self)
            .get_result(connection)
    }
}

impl Product {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Product, diesel::result::Error> {
        products::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::products.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_product: &NewProduct, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::update(dsl::products.find(id))
            .set(new_product)
            .execute(connection)?;
        Ok(())
    }
}
