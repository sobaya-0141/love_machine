table! {
    machine (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
        displacement -> Nullable<Int4>,
        machine_type -> Nullable<Int4>,
    }
}

table! {
    machine_type (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    member (id) {
        id -> Int4,
        nickname -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
    }
}

table! {
    member_machine (id) {
        id -> Int4,
        member_id -> Nullable<Int4>,
        machine_id -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    machine,
    machine_type,
    member,
    member_machine,
);
