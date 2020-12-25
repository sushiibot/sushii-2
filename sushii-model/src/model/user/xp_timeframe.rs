#[derive(juniper::GraphQLEnum, Debug, Copy, Clone)]
#[graphql(description = "An XP timeframe")]
pub enum TimeFrame {
    AllTime,
    Day,
    Week,
    Month,
}
