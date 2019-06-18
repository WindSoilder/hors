use std::collections::HashMap;
use std::env;
#[cfg(target_os = "windows")]
use std::error::Error;
#[cfg(target_os = "windows")]
use winreg::enums::HKEY_CURRENT_USER;
#[cfg(target_os = "windows")]
use winreg::RegKey;

/// Get system proxies information.
///
/// It can only support Linux, Unix like, and windows system.  Note that it will always
/// return a HashMap, even if something runs into error when find registry information in
/// Windows system.
///
/// Returns:
///     System proxies information as a hashmap like
///     {"http": "http://127.0.0.1:80", "https": "https://127.0.0.1:80"}
pub fn get_proxies() -> HashMap<String, String> {
    let proxies: HashMap<String, String> = get_from_environment();

    if proxies.len() == 0 {
        // don't cared if we can't get proxies from registry, just return an empty proxies.
        #[cfg(target_os = "windows")]
        let proxies = get_from_registry();
        return proxies;
    } else {
        return proxies;
    }
}

fn get_from_environment() -> HashMap<String, String> {
    let mut proxies: HashMap<String, String> = HashMap::new();

    const PROXY_KEY_ENDS: &str = "_proxy";

    for (key, value) in env::vars() {
        let key: String = key.to_lowercase();
        if key.ends_with(PROXY_KEY_ENDS) {
            let end_indx = key.len() - PROXY_KEY_ENDS.len();
            let schema = &key[..end_indx];
            proxies.insert(String::from(schema), String::from(value));
        }
    }
    return proxies;
}

#[cfg(target_os = "windows")]
fn get_from_registry_impl() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let internet_setting: RegKey =
        hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings")?;
    // ensure the proxy is enable, if the value doesn't exist, an error will returned.
    let proxy_enable: u32 = internet_setting.get_value("ProxyEnable")?;
    let proxy_server: String = internet_setting.get_value("ProxyServer")?;

    if proxy_enable == 0 {
        return Ok(HashMap::new());
    }

    let mut proxies: HashMap<String, String> = HashMap::new();
    if proxy_server.contains("=") {
        // per-protocol settings.
        for p in proxy_server.split(";") {
            let protocol_parts: Vec<&str> = p.split("=").collect();
            match protocol_parts.as_slice() {
                [protocol, address] => {
                    proxies.insert(String::from(*protocol), String::from(*address));
                }
                _ => {
                    // Contains invalid protocol setting, just break the loop
                    // And make proxies to be empty.
                    proxies.clear();
                    break;
                }
            }
        }
    } else {
        // Use one setting for all protocols.
        if proxy_server.starts_with("http:") {
            proxies.insert(String::from("http"), proxy_server);
        } else {
            proxies.insert(String::from("http"), format!("http://{}", proxy_server));
            proxies.insert(String::from("https"), format!("https://{}", proxy_server));
            proxies.insert(String::from("ftp"), format!("https://{}", proxy_server));
        }
    }
    return Ok(proxies);
}

#[cfg(target_os = "windows")]
fn get_from_registry() -> HashMap<String, String> {
    let results = get_from_registry_impl();
    match results {
        Ok(proxies) => proxies,
        Err(_) => HashMap::new(),
    }
}
