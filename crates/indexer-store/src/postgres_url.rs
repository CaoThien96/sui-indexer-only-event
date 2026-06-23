use std::env;

use anyhow::{Context, Result};
use url::Url;

/// Build a Postgres URL from `{prefix}_USER`, `{prefix}_PASSWORD`, `{prefix}_HOST`,
/// `{prefix}_PORT`, and `{prefix}_DB` when all required vars are set.
///
/// Passwords with special characters (e.g. `@`) are percent-encoded. This avoids
/// docker-compose `${POSTGRES_PASSWORD}` interpolation failures and broken `DATABASE_URL`
/// strings in `.env`.
pub fn resolve_postgres_url(prefix: &str, fallback_var: &str) -> Result<Url> {
    let user_key = format!("{prefix}_USER");
    let pass_key = format!("{prefix}_PASSWORD");
    let host_key = format!("{prefix}_HOST");
    let port_key = format!("{prefix}_PORT");
    let db_key = format!("{prefix}_DB");

    if let (Ok(user), Ok(password), Ok(host), Ok(db)) = (
        env::var(&user_key),
        env::var(&pass_key),
        env::var(&host_key),
        env::var(&db_key),
    ) {
        let port = env::var(&port_key).unwrap_or_else(|_| "5432".to_string());
        let mut url = Url::parse(&format!("postgres://{host}:{port}/{db}"))
            .with_context(|| format!("invalid postgres host for {prefix}_* env vars"))?;
        url.set_username(&user)
            .map_err(|()| anyhow::anyhow!("invalid {prefix}_USER"))?;
        url.set_password(Some(&password))
            .map_err(|()| anyhow::anyhow!("invalid {prefix}_PASSWORD"))?;
        return Ok(url);
    }

    let raw = env::var(fallback_var)
        .with_context(|| format!("{fallback_var} or {prefix}_* must be set"))?;
    raw.parse::<Url>()
        .with_context(|| format!("invalid {fallback_var}"))
}

#[cfg(test)]
mod tests {
    use super::resolve_postgres_url;
    use std::env;

    fn with_env(vars: &[(&str, &str)], f: impl FnOnce()) {
        let keys: Vec<&str> = vars.iter().map(|(k, _)| *k).collect();
        let prev: Vec<(&str, Option<String>)> = keys
            .iter()
            .map(|k| (*k, env::var(k).ok()))
            .collect();
        for (k, v) in vars {
            // SAFETY: test-only env mutation; single-threaded test runner.
            unsafe { env::set_var(k, v) };
        }
        f();
        for (k, v) in prev {
            match v {
                // SAFETY: test-only env mutation; single-threaded test runner.
                Some(val) => unsafe { env::set_var(k, val) },
                None => unsafe { env::remove_var(k) },
            }
        }
    }

    #[test]
    fn builds_url_with_encoded_password() {
        with_env(
            &[
                ("POSTGRES_USER", "postgres"),
                ("POSTGRES_PASSWORD", "thien@123"),
                ("POSTGRES_HOST", "postgres"),
                ("POSTGRES_PORT", "5432"),
                ("POSTGRES_DB", "sui_indexer"),
            ],
            || {
                let url = resolve_postgres_url("POSTGRES", "DATABASE_URL").unwrap();
                assert_eq!(url.username(), "postgres");
                assert_eq!(url.host_str(), Some("postgres"));
                assert!(
                    url.as_str().contains("thien%40123"),
                    "expected encoded password in connection string: {}",
                    url.as_str()
                );
            },
        );
    }
}
