use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: String,
    pub end_cursor: String,
}

#[macro_export]
macro_rules! relay_connection {
    ($connection:ident, $edge:ident, $type:ty) => {
        relay_connection!($connection, $edge, $type, ());
    };
    ($connection:ident, $edge:ident, $type:ty, $context:ty) => {
        #[derive(juniper::GraphQLObject)]
        #[graphql(context = $context)]
        pub struct $edge {
            pub node: $type,
            pub cursor: String,
        }

        #[derive(juniper::GraphQLObject)]
        #[graphql(context = $context)]
        pub struct $connection {
            pub total_count: ::sushii_model::model::BigInt,
            pub edges: Vec<$edge>,
            pub page_info: $crate::relay::PageInfo,
        }
    };
}
