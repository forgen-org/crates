mod effect;
mod function;

use effect::*;
// use function::*;

struct LoginCommand {
    username: String,
    #[allow(dead_code)]
    password: String,
}

trait Runtime: Clone + Copy + Send + Sync + 'static {}

impl<R> ToEffect<R, String, ()> for LoginCommand
where
    R: Runtime + JwtService,
{
    fn to_effect(self) -> Effect<R, String, ()> {
        Effect::new({
            move |runtime: R| {
                let username = self.username.clone();
                async move {
                    let jwt = runtime.generate(&username);
                    Ok(jwt)
                }
            }
        })
    }
}

trait JwtService {
    fn generate(&self, username: &str) -> String;
}

#[derive(Clone, Copy)]
struct SomeJwtService;

impl JwtService for SomeJwtService {
    fn generate(&self, username: &str) -> String {
        format!("{}:{}", username, "jwt")
    }
}

impl Runtime for SomeJwtService {}

#[tokio::main]
async fn main() {
    let command = LoginCommand {
        username: "user".to_string(),
        password: "password".to_string(),
    };

    let runtime = SomeJwtService;

    let effect = command.to_effect();

    let res = effect.run(runtime).await;

    println!("res: {:?}", res);
}
