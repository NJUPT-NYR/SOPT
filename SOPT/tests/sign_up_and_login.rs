use cucumber_rust::{Steps, t};
use sopt::sopt_main;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::convert::Infallible;
use serde::{Serialize};
use cucumber_rust::{async_trait, World};

#[derive(Debug, Serialize, Clone)]
pub struct MyUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub struct MyWorld {
    mock_user: Option<MyUser>,
    response: Option<sopt::data::GeneralResponse>,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {

        Ok(Self {
            mock_user: None,
            response: None,
        })
    }
}


pub fn steps() -> Steps<MyWorld> {
    std::thread::spawn(|| sopt_main());

    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(
        r#"a new random user ([\w\s!]+)$"#,
        |mut world, ctx| {
            let s = &ctx.matches[1].trim().to_string();
            let mut mock: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
            for i in 0..3 {
                mock[i] = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();
            }
            let email = if s.eq("allowed") {
                mock[0].clone() + "@gmail.com"
            } else {
                mock[0].clone() + "@banned.com"
            };

            world.mock_user = Some(MyUser {
                email,
                username: mock[1].clone(),
                password: mock[2].clone(),
            });
            world
        }
    );

    steps.then(
        "mock_user is not null",
        |world, _ctx| {
            assert!(world.mock_user.is_some());
            world
        }
    );

    steps.when_regex_async(
        r#"I sign up as this user([\s\w!]*)$"#,
      t!(|mut world, _ctx| {
              let client = reqwest::Client::new();
              let json = world.mock_user.clone().unwrap();
              world.response = Some(client.post("http://127.0.0.1:8000/api/user/add_user")
                  .json(&json)
                  .send()
                  .await.unwrap()
                  .json()
                  .await.unwrap());
              world
      })
    );

    steps.then(
        "I get a return json",
        |world, _ctx| {
            assert!(world.response.is_some());
            world
        }
    );

    steps.then_regex(
        r#"return json is ([\w\s!]+)$"#,
        |world, ctx| {
            let s = &ctx.matches[1].trim().to_string();
            let res = world.response.clone();
            assert_eq!(res.unwrap().success, s.parse::<bool>().unwrap());
            world
        }
    );

    steps.when_async(
        "I login as this user",
        t!(|mut world, _ctx| {
            let client = reqwest::Client::new();
            let mock = world.mock_user.clone().unwrap();
            let new_login = Login {
                username: mock.username,
                password: mock.password,
            };
            world.response = Some(client.post("http://127.0.0.1:8000/api/user/login")
                .json(&new_login)
                .send()
                .await.unwrap()
                .json()
                .await.unwrap());
            world
        })
    );

    steps.when_async(
        "I login with wrong password",
        t!(|mut world, _ctx| {
            let client = reqwest::Client::new();
            let mock = world.mock_user.clone().unwrap();
            let new_login = Login {
                username: mock.username,
                password: mock.password + "114",
            };
            world.response = Some(client.post("http://127.0.0.1:8000/api/user/login")
                .json(&new_login)
                .send()
                .await.unwrap()
                .json()
                .await.unwrap());
            world
        })
    );

    steps
}