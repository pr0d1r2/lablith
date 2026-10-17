//! The OCI distribution flow (`src/fetch:V34`, `src/pin:V58`): `HEAD`
//! the manifest of `name:tag` asking for a multi-arch index; on `401`,
//! take an anonymous bearer token from the realm the registry names, and
//! ask again. The answer is the index digest, which covers every linux
//! system a runner may be.

use super::curl::{Http, Response};
use super::error::FetchError;

/// The index media types asked for, OCI and Docker's.
const ACCEPT: &str = "Accept: application/vnd.oci.image.index.v1+json, \
    application/vnd.docker.distribution.manifest.list.v2+json";

/// Docker Hub's registry host, for a name with no host of its own.
const HUB: &str = "registry-1.docker.io";

/// Tag -> digest, the seam `pin` resolves through.
pub(crate) trait Resolver {
    /// The index digest (`sha256:` + 64 hex) of `image:tag`.
    fn digest(&self, image: &str, tag: &str) -> Result<String, FetchError>;
}

/// The real resolver over an HTTP client.
pub(crate) struct Oci<H: Http>(pub(crate) H);

impl<H: Http> Resolver for Oci<H> {
    fn digest(&self, image: &str, tag: &str) -> Result<String, FetchError> {
        let url = manifest(image, tag)?;
        let answer = self.ask(&url).and_then(|r| index_digest(&r));
        answer.map_err(|why| FetchError::Resolve {
            image: image.to_owned(),
            tag: tag.to_owned(),
            why,
        })
    }
}

impl<H: Http> Oci<H> {
    /// `HEAD url` for the index; on `401`, again with a bearer token.
    fn ask(&self, url: &str) -> Result<Response, String> {
        let first = self.0.head(url, &[ACCEPT.to_owned()])?;
        if first.status != 401 {
            return Ok(first);
        }
        let auth = format!("Authorization: Bearer {}", self.token(&first)?);
        self.0.head(url, &[ACCEPT.to_owned(), auth])
    }

    /// An anonymous token from the `www-authenticate` challenge of `r`.
    fn token(&self, r: &Response) -> Result<String, String> {
        let challenge = r
            .header("www-authenticate")
            .ok_or("401 without a challenge")?;
        let field = |name: &str| quoted(challenge, name);
        let realm = field("realm").ok_or("challenge names no realm")?;
        let service = field("service").unwrap_or_default();
        let scope = field("scope").unwrap_or_default();
        let body = self
            .0
            .get(&format!("{realm}?service={service}&scope={scope}"))?;
        let token = quoted(&body.replace(": ", ":"), "\"token\"");
        token.ok_or_else(|| "no token in the answer".to_owned())
    }
}

/// `https://<host>/v2/<repo>/manifests/<tag>`, or a refusal when the name
/// or tag holds anything a registry name or tag cannot.
fn manifest(image: &str, tag: &str) -> Result<String, FetchError> {
    if !(name_ok(image) && tag_ok(tag)) {
        return Err(FetchError::Name {
            image: image.to_owned(),
            tag: tag.to_owned(),
        });
    }
    let (host, repo) = split(image);
    Ok(format!("https://{host}/v2/{repo}/manifests/{tag}"))
}

/// A registry name: lower-case letters, digits and `._-/:`.
fn name_ok(image: &str) -> bool {
    let ok = |c: char| {
        c.is_ascii_lowercase() || c.is_ascii_digit() || "._-/:".contains(c)
    };
    !image.is_empty() && image.chars().all(ok)
}

/// A tag: 1 to 128 of letters, digits and `._-`.
fn tag_ok(tag: &str) -> bool {
    let ok = |c: char| c.is_ascii_alphanumeric() || "._-".contains(c);
    (1..=128).contains(&tag.len()) && tag.chars().all(ok)
}

/// The registry host and repository of `image`: a first component with a
/// `.` or `:` (or `localhost`) is a host; otherwise Docker Hub, where a
/// bare name lives under `library/`.
fn split(image: &str) -> (String, String) {
    match image.split_once('/') {
        Some((first, rest))
            if first.contains(['.', ':']) || first == "localhost" =>
        {
            (first.to_owned(), rest.to_owned())
        }
        Some(_) => (HUB.to_owned(), image.to_owned()),
        None => (HUB.to_owned(), format!("library/{image}")),
    }
}

/// The digest of a `200` index answer, checked.
fn index_digest(r: &Response) -> Result<String, String> {
    if r.status != 200 {
        return Err(format!("registry answered {}", r.status));
    }
    let kind = r.header("content-type").unwrap_or_default();
    if !(kind.contains("image.index") || kind.contains("manifest.list")) {
        return Err(format!("`{kind}` is not a multi-arch index"));
    }
    let digest = r.header("docker-content-digest").unwrap_or_default();
    let hex = digest.strip_prefix("sha256:").unwrap_or_default();
    let ok = hex.len() == 64
        && hex
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c));
    ok.then(|| digest.to_owned())
        .ok_or_else(|| format!("digest `{digest}` is not sha256"))
}

/// The value of `name="value"` (or `name:"value"`) in `text`.
fn quoted(text: &str, name: &str) -> Option<String> {
    let after = [format!("{name}="), format!("{name}:")]
        .iter()
        .find_map(|key| text.split_once(key.as_str()).map(|(_, rest)| rest))?;
    let rest = after.strip_prefix('"')?;
    rest.split_once('"').map(|(v, _)| v.to_owned())
}

#[cfg(test)]
mod tests;
