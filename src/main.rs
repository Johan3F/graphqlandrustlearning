use juniper::{
    graphql_object, graphql_value, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum,
    Variables,
};

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

// Arbitrary context data.
struct Ctx(Episode);

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    fn favoriteEpisode(context: &Ctx) -> FieldResult<Episode> {
        Ok(context.0)
    }

    fn giveMeJedi(_context: &Ctx) -> FieldResult<Episode> {
        Ok(Episode::Jedi {})
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create a context object.
    let ctx = Ctx(Episode::NewHope);

    // Run the executor.
    let (res, _errors) = juniper::execute_sync(
        "query { favoriteEpisode }",
        None,
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    // Ensure the value matches.
    assert_eq!(
        res,
        graphql_value!({
            "favoriteEpisode": "NEW_HOPE",
        })
    );
    println!("res {:?}", res);
}
