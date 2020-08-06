macro_rules! command {
    ($name:expr, $handler:item) => {
        $name => $handler(msg, ctx).await?
    };
}
