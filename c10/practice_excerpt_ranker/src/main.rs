trait Describe {
    fn describe(&self) -> String;
}

trait RiskScore {
    fn risk_score(&self) -> i32;
}

struct LoginFailure {
    user: String,
    ip_address: String,
    attempts: i32,
    message: String,
}

struct ApiError {
    endpoint: String,
    status_code: i32,
    count: i32,
    message: String,
}

impl Describe for LoginFailure {
    fn describe(&self) -> String {
        format!(
            "login failure: user={} ip={} attempts={}",
            self.user, self.ip_address, self.attempts
        )
    }
}

impl Describe for ApiError {
    fn describe(&self) -> String {
        format!(
            "api error: endopoint={} status={} count={}",
            self.endpoint, self.status_code, self.count
        )
    }
}

impl RiskScore for LoginFailure {
    fn risk_score(&self) -> i32 {
        self.attempts * 10
    }
}

impl RiskScore for ApiError {
    fn risk_score(&self) -> i32 {
        self.count + if self.status_code == 500 { 30 } else { 0 }
    }
}

struct Alert<T> {
    event: T,
}

impl<T: Describe + RiskScore> Alert<T> {
    fn print(&self) {
        let score = self.event.risk_score();

        let severity = match score {
            70.. => "HIGH",
            40..=69 => "MEDIUM",
            _ => "LOW",
        };

        println!("=== Alert ===");
        println!("{}", self.event.describe());
        println!("score: {}", score);
        println!("severity: {}", severity);
    }
}

fn longest_message<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

struct Longest<'a> {
    message: &'a str,
}

fn main() {
    let login = LoginFailure {
        user: String::from("admin"),
        ip_address: String::from("203.0.113.10"),
        attempts: 8,
        message: String::from("admin user failed login repeatedly from an unfamiliar network"),
    };

    let api = ApiError {
        endpoint: String::from("/v1/payments"),
        status_code: 500,
        count: 12,
        message: String::from("payment endpoint returned repeated internal server errors"),
    };

    let login_alert = Alert { event: login };
    let api_alert = Alert { event: api };

    login_alert.print();
    api_alert.print();

    let longest = Longest {
        message: longest_message(&login_alert.event.message, &api_alert.event.message),
    };

    println!("longest message:");
    println!("{}", longest.message);
}
