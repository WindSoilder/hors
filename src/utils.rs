use rand::seq::SliceRandom;
use rand::thread_rng;

static USER_AGENTS: [&str; 1] =
    ["Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:66.0) Gecko/20100101 Firefox/66.0"];
static DEFAULT_AGENT: &str = "hors";

pub fn random_agent() -> &'static str {
    let mut rng = thread_rng();
    match USER_AGENTS.choose(&mut rng) {
        Some(user_agent) => return user_agent,
        None => return DEFAULT_AGENT,
    }
}
