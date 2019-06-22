use super::*;
use url::Url;

thread_local! {
    static POLICIES: RefCell<Vec<Policy>> = RefCell::new(Vec::new())
}

pub fn set_policies(policies: Vec<Policy>) {
    POLICIES.with(|p| {
        *p.borrow_mut() = policies;
    });
}

pub fn policy_check(method: &str, url: &str, header_map: &HeaderMap) -> bool {
    if !policy_check_url(method, url) {
        return false;
    }

    let mut failures = 0;
    // This will match host, Host, hOSt, etc.
    let hosts = header_map.get_all("host").iter();

    let mut host_matches = false;
    for host_override in hosts {
        host_matches = true;
        let new_url = swap_url_host(url, host_override.to_str().unwrap());
        let result = policy_check_url(method, &new_url.unwrap());
        if !result {
            failures += 1;
        }
    }

    // if there are no host headers it's ok. If any host headers fail it's not ok
    !host_matches || failures <= 0
}

fn policy_check_url(method: &str, url: &str) -> bool {
    let url = match url.find('#') {
        Some(index) => url.split_at(index).0,
        None => url,
    };
    let url = match url.find('?') {
        Some(index) => url.split_at(index).0,
        None => url,
    };
    let mut matches = 0;
    POLICIES.with(|p| {
        for policy in p.borrow().iter() {
            if policy.matches(method, url) {
                matches += 1;
                break;
            }
        }
    });

    matches > 0
}

fn swap_url_host(
    original_url: &str,
    host_override: &str,
) -> Result<std::string::String, url::ParseError> {
    let mut parsed_url = Url::parse(original_url).unwrap();
    parsed_url.set_host(Some(host_override))?;

    Ok(parsed_url.into_string())
}
