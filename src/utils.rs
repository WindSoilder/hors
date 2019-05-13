use rand::seq::SliceRandom;
use rand::thread_rng;

static USER_AGENTS: [&str; 6] =
    [
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:66.0) Gecko/20100101 Firefox/66.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:66.0) Gecko/20100101 Firefox/66.0",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.103 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0.3 Safari/605.1.15",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36 OPR/58.0.3135.132",
    ];
static DEFAULT_AGENT: &str = "hors";

/// Generate a random User-Agents.
///
/// # Return value
///
/// A User-Agent str which can be used for User-Agent fields.
pub fn random_agent() -> &'static str {
    let mut rng = thread_rng();
    match USER_AGENTS.choose(&mut rng) {
        Some(user_agent) => {
            debug!("selected User-Agent: {}", user_agent);
            return user_agent;
        }
        None => return DEFAULT_AGENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_agent() {
        assert!(USER_AGENTS.contains(&random_agent()));
    }
}
