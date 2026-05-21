table! {
    use diesel::sql_types::*;
    use crate::syslog::OpType;

    address (id) {
        id -> Int4,
        client_id -> Int4,
        #[sql_name = "type"]
        type_ -> Int2,
        location -> Varchar,
        number -> Varchar,
        complement -> Nullable<Varchar>,
        district -> Varchar,
        state -> Bpchar,
        city -> Varchar,
        country -> Bpchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::syslog::OpType;

    client (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Int2,
        name -> Varchar,
        document -> Varchar,
        active -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::syslog::OpType;

    product (id) {
        id -> Int4,
        description -> Varchar,
        unit -> Bpchar,
        price -> Numeric,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::syslog::OpType;

    stock (product_id) {
        product_id -> Int4,
        amount -> Numeric,
        cost -> Numeric,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::syslog::OpType;

    stock_mov (id) {
        id -> Int4,
        product_id -> Int4,
        document -> Varchar,
        amount -> Numeric,
        cost -> Numeric,
        shipping_cost -> Numeric,
        date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::syslog::OpType;

    syslog (id) {
        id -> Int4,
        service -> Varchar,
        requestor -> Varchar,
        entity -> Varchar,
        operation -> OpType,
        datetime -> Timestamptz,
        description -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::syslog::OpType;

    user (id) {
        id -> Int4,
        login -> Varchar,
        name -> Varchar,
        email -> Nullable<Varchar>,
        pwhash -> Bytea,
    }
}

joinable!(address -> client (client_id));

allow_tables_to_appear_in_same_query!(address, client, product, stock, stock_mov, syslog, user,);
