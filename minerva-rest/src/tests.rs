use super::launch;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};
use serde::Deserialize;
use serde_json::json;
use serial_test::serial;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    time::{Duration, SystemTime},
};

#[derive(Debug)]
struct Microservices {
    services: Vec<(&'static str, Child)>,
}

impl Microservices {
    fn spawn_microservice(name: &str, service: &str) -> Child {
        println!("Spawning microservice {}...", name);
        // Spawn through `cargo run`
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(service)
            .current_dir("../")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect(&format!("Failed to create child process for {}", service));

        // Check if read by waiting for "SERVICENAME is ready to accept connections."
        // in command output
        let expected_text = format!("{} is ready to accept connections.", name);

        println!("Awaiting for microservice {} to be ready...", name);
        let start = SystemTime::now();
        'await_child: loop {
            if let Some(stdout) = &mut child.stdout {
                // There aren't many lines so slurp them on memory
                let lines = BufReader::new(stdout).lines().enumerate();
                for (counter, line) in lines {
                    if line.unwrap().trim() == expected_text.trim() {
                        println!(
                            "Microservice {} is ready (as per line output {})",
                            name, counter
                        );
                        break 'await_child;
                    }
                }
            }
            // Check for timeout. Max tolerance: two minutes.
            let duration = SystemTime::now().duration_since(start).unwrap();
            if duration > Duration::from_secs(120) {
                child
                    .kill()
                    .expect("Gracefully kill microservice spawning that takes too long");
                panic!(
                    "Failed while spawning microservice {}: Timeout after two minutes",
                    name
                );
            }

            // Pause thread for two seconds
            std::thread::sleep(Duration::from_secs(2));
        }
        child
    }

    fn spawn(services: Vec<&'static str>) -> Self {
        Microservices {
            services: services
                .iter()
                .map(|name| {
                    let service = match *name {
                        "TENANCY" => "minerva-tenancy",
                        "USER" => "minerva-user",
                        "SESSION" => "minerva-session",
                        "PRODUCT" => "minerva-product",
                        "STOCK" => "minerva-stock",
                        "REPORT" => "minerva-report",
                        "CLIENT" => "minerva-client",
                        "AUDIT" => "minerva-audit",
                        "COMM" => "minerva-comm",
                        _ => panic!("Unknown service {}", name),
                    };
                    (*name, Microservices::spawn_microservice(name, service))
                })
                .collect(),
        }
    }

    fn dispose(&mut self) {
        for (svc, proc) in self.services.iter_mut() {
            proc.kill().expect(&format!(
                "Successfully send kill signal to {} microservice",
                svc
            ));
            proc.wait().unwrap();
        }
    }
}

fn make_client() -> Client {
    let config = rocket::Config {
        log_level: rocket::config::LogLevel::Critical,
        ..rocket::Config::debug_default()
    };
    Client::tracked(launch().configure(config)).expect("Inst??ncia v??lida da API")
}

/* Authentication */

#[test]
#[serial]
fn login_logout() {
    let mut svc = Microservices::spawn(vec!["SESSION"]);
    let client = make_client();

    // Login
    let response = client
        .post("/teste/login")
        .body(
            json! ({
                "login": "admin",
                "password": "admin"
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.cookies().get("tenant").is_some());
    assert!(response.cookies().get_private("auth_token").is_some());

    #[derive(Deserialize)]
    struct LoginResponse {
        pub token: String,
        pub tenant: String,
    }

    let data = response
        .into_json::<LoginResponse>()
        .expect("Deserialize login data");

    assert_eq!(data.tenant.trim(), "teste");
    assert!(!data.token.trim().is_empty());

    // Logout
    // Reuses previous cookies
    let response = client.post("/logout").dispatch();
    assert_eq!(response.status(), Status::Ok);

    svc.dispose();
}

/* User API */

#[test]
#[serial]
fn get_user_data() {
    use minerva_data::user::User;

    let mut svc = Microservices::spawn(vec!["SESSION", "USER"]);
    let client = make_client();

    // Login
    let response = client
        .post("/teste/login")
        .body(
            json! ({
                "login": "admin",
                "password": "admin"
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Get users
    let response: LocalResponse = client.get("/user").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user_list = response
        .into_json::<Vec<User>>()
        .expect("Deserialize User list");
    let user = user_list
        .iter()
        .find(|u| u.login.trim() == "admin")
        .expect("Could not find admin");
    assert_eq!(user.name.trim(), "Administrator");
    assert_eq!(user.email, None);

    // Get single user: the same administrator found before
    let id = user.id;
    let response = client.get(format!("/user/{}", id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user = response.into_json::<User>().expect("Deserialize User");
    assert_eq!(user.login.trim(), "admin");
    assert_eq!(user.name.trim(), "Administrator");
    assert_eq!(user.email, None);

    // Logout
    let response = client.post("/logout").dispatch();
    assert_eq!(response.status(), Status::Ok);

    svc.dispose();
}

#[test]
#[serial]
fn crud_user() {
    use minerva_data::user::User;

    let mut svc = Microservices::spawn(vec!["SESSION", "USER"]);
    let client = make_client();

    // Login
    let response = client
        .post("/teste/login")
        .body(
            json! ({
                "login": "admin",
                "password": "admin"
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Create user
    let response = client
        .post("/user")
        .body(
            json!({
            "login": "fulano_teste_rest",
            "name": "Fulano da Silva",
            "email": "fulano@exemplo.com",
            "password": "senha123",
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user = response.into_json::<User>().expect("Deserialize User");
    assert_eq!(user.login.trim(), "fulano_teste_rest");
    assert_eq!(user.name.trim(), "Fulano da Silva");
    assert_eq!(user.email, Some("fulano@exemplo.com".into()));

    // Fetch user as inserted
    let id = user.id;
    let response = client.get(format!("/user/{}", id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user = response.into_json::<User>().expect("Deserialize User");
    assert_eq!(user.login.trim(), "fulano_teste_rest");
    assert_eq!(user.name.trim(), "Fulano da Silva");
    assert_eq!(user.email, Some("fulano@exemplo.com".into()));

    // Update user data
    let response = client
        .put(format!("/user/{}", id))
        .body(
            json!({
                "login": user.login.clone(),
                "name": "Fulano de Tal",
                "email": user.email.clone()
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user = response.into_json::<User>().expect("Deserialize User");
    assert_eq!(user.login.trim(), "fulano_teste_rest");
    assert_eq!(user.name.trim(), "Fulano de Tal");
    assert_eq!(user.email, Some("fulano@exemplo.com".into()));

    // Fetch modified user again
    let response = client.get(format!("/user/{}", id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let user = response.into_json::<User>().expect("Deserialize User");
    assert_eq!(user.login.trim(), "fulano_teste_rest");
    assert_eq!(user.name.trim(), "Fulano de Tal");
    assert_eq!(user.email, Some("fulano@exemplo.com".into()));

    // Remove user
    let response = client.delete(format!("/user/{}", id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.into_string(), Some("{}".into()));

    // Logout
    let response = client.post("/logout").dispatch();
    assert_eq!(response.status(), Status::Ok);

    svc.dispose();
}

#[test]
#[serial]
fn failed_requests() {
    let client = make_client();

    // Requests for default catchers

    // 404 with an invalid route
    let response = client.get("/invalidroute").dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // 503 for trying to log in while SESSION was not created
    let response = client
        .post("/teste/login")
        .body(
            json! ({
                "login": "admin",
                "password": "admin"
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::ServiceUnavailable);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.cookies().get("tenant"), None);
    assert_eq!(response.cookies().get_private("token"), None);

    // Create microservices
    let mut svc = Microservices::spawn(vec!["SESSION", "USER"]);

    // 422 for a malformed login request
    let response = client
        .post("/teste/login")
        .body(
            json! ({
                "login": "admin",
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // 401 for attempting to list users without logging in
    let response = client.get("/user").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // Login
    let response = client
        .post("/teste/login")
        .body(
            json! ({
                "login": "admin",
                "password": "admin"
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 422 for a malformed user creation request
    let response = client
        .post("/user")
        .body(
            json!({
                "name": "Fulano da Silva",
                "email": "fulano@exemplo.com",
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // Logout
    let response = client.post("/logout").dispatch();
    assert_eq!(response.status(), Status::Ok);

    svc.dispose();
}
