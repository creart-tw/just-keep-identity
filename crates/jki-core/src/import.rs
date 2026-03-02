use crate::{Account, AccountType};
use url::Url;

pub fn parse_otpauth_uri(uri: &str) -> Option<Account> {
    let url = Url::parse(uri).ok()?;
    if url.scheme() != "otpauth" { return None; }

    let host = url.host_str()?; // totp
    if host != "totp" { return None; }

    // Path is /Label or /Issuer:Label
    let path = url.path().trim_start_matches('/');
    let (issuer, name) = if let Some(pos) = path.find(':') {
        (Some(path[..pos].to_string()), path[pos+1..].to_string())
    } else {
        (None, path.to_string())
    };

    let query: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    let secret = query.get("secret")?.clone();
    let digits = query.get("digits").and_then(|d| d.parse::<u32>().ok()).unwrap_or(6);
    let issuer_query = query.get("issuer").cloned();
    
    let effective_issuer = issuer.clone().or(issuer_query);
    let account_type = if effective_issuer.as_deref() == Some("Steam") {
        AccountType::Steam
    } else if effective_issuer.as_deref() == Some("BattleNet") {
        AccountType::Blizzard
    } else {
        AccountType::Standard
    };

    Some(Account {
        id: uuid::Uuid::new_v4().to_string(),
        name: urllib_unquote(&name.replace('+', " ")),
        issuer: effective_issuer,
        account_type,
        secret,
        digits,
        algorithm: "SHA1".to_string(),
    })
}

fn urllib_unquote(s: &str) -> String {
    url::form_urlencoded::parse(s.as_bytes())
        .map(|(k, _)| k.into_owned())
        .collect::<Vec<_>>()
        .join("")
}
