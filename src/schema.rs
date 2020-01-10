table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        stock -> Float8,
        price -> Nullable<Int4>,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(products -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
