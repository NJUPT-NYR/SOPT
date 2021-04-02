use cucumber_rust::{criteria::feature, futures::FutureExt, Context, Cucumber};
use sopt::sopt_main;

mod sign_up_and_login;

#[tokio::main]
async fn main() {
    let addr = dotenv::var("DATABASE_URL").unwrap();
    let pool = sqlx::PgPool::connect(&addr)
        .await
        .expect("unable to connect to database");
    std::thread::spawn(|| sopt_main());

    Cucumber::<sign_up_and_login::MyWorld>::new()
        .features(&["./tests/features/"])
        .steps(sign_up_and_login::steps())
        .context(Context::new().add(pool))
        .after(feature("Register a new user and sign in."), |ctx| {
            let pool = ctx.get::<sqlx::PgPool>().unwrap().clone();
            async move {
                sqlx::query!("DELETE FROM user_info WHERE id > 1;")
                    .execute(&pool)
                    .await
                    .unwrap();
                sqlx::query!("DELETE FROM users WHERE id > 1;")
                    .execute(&pool)
                    .await
                    .unwrap();
            }
            .boxed()
        })
        .cli()
        .run()
        .await;
}
