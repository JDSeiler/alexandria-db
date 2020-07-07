use rusqlite::Connection;

/**

Returns a `Result` that is either a usable connection to the database
or a `rusqlite::Error`. The path to the database is currently
hardcoded into the function as a relative path. Starting alexandria-db
from a directory besides the project root results in this relative path
being incorrect and this function returning an error.

**/
pub fn get_database_connection() -> Result<Connection, rusqlite::Error> {
    let test_db_path = "./src/db_storage/dummy.db";
    let maybe_conn = Connection::open(&test_db_path);
    maybe_conn
}

/** 
This is getting a gigantic doc comment because I don't understand this
super well and documentation will help me internalize and remember.

First, #[macro_export] is the attribute you have to use to make a 
macro usable across modules.

Next, this macro is "overloaded", once for non pub structs and onces for
pub structs.

Fundamentally what this does is take in some Rust code as a pattern, then
spit out some Rust code in response. This macro takes in a struct
definition and spits out an identical struct definition with the derive
attributes added to it, and also creates an impl block for the field_names
method.
**/
#[macro_export]
macro_rules! create_struct_with_impl {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Serialize, Deserialize, Debug)]
        struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            fn field_names() -> Vec<&'static str> {
                vec![$(stringify!($field_name)), *]
            }
        }
    };

    (pub struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            fn field_names() -> Vec<&'static str> {
                vec![$(stringify!($field_name)), *]
            }
        }
    }
}
