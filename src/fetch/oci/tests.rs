use std::cell::RefCell;

use super::{Oci, Resolver};
use crate::fetch::FetchError;
use crate::fetch::curl::{Http, Response};

const DIGEST: &str =
    "sha256:ff480dbb23511aa1c10710aa068ceea5b93fed9bfce5f5379f3a6b07282cbc7b";
const INDEX: &str = "application/vnd.docker.distribution.manifest.list.v2+json";

/// A registry that wants a token, then answers with `kind` and `digest`.
struct Hub {
    kind: &'static str,
    digest: &'static str,
    asked: RefCell<Vec<String>>,
}

const CHALLENGE: &str = "Bearer realm=\"https://auth.docker.io/token\",\
    service=\"registry.docker.io\",scope=\"repository:nixos/nix:pull\"";

impl Hub {
    /// The answer once a token is sent.
    fn found(&self) -> Response {
        let headers = vec![
            ("content-type".into(), self.kind.into()),
            ("docker-content-digest".into(), self.digest.into()),
        ];
        Response {
            status: 200,
            headers,
        }
    }
}

impl Http for Hub {
    fn head(&self, url: &str, headers: &[String]) -> Result<Response, String> {
        let asked = format!("HEAD {url} {}", headers.len());
        self.asked.borrow_mut().push(asked);
        if headers.len() == 1 {
            let challenge = vec![("www-authenticate".into(), CHALLENGE.into())];
            return Ok(Response {
                status: 401,
                headers: challenge,
            });
        }
        Ok(self.found())
    }

    fn get(&self, url: &str) -> Result<String, String> {
        self.asked.borrow_mut().push(format!("GET {url}"));
        Ok("{\"token\": \"t0k\", \"expires_in\": 300}".into())
    }
}

fn hub(kind: &'static str, digest: &'static str) -> Oci<Hub> {
    Oci(Hub {
        kind,
        digest,
        asked: RefCell::new(Vec::new()),
    })
}

#[test]
fn docker_hub_answers_after_an_anonymous_token() {
    let oci = hub(INDEX, DIGEST);
    assert_eq!(oci.digest("nixos/nix", "2.35.3"), Ok(DIGEST.to_owned()));
    let asked = oci.0.asked.borrow().clone();
    let url = "https://registry-1.docker.io/v2/nixos/nix/manifests/2.35.3";
    let token = "GET https://auth.docker.io/token?service=registry.docker.io&scope=repository:nixos/nix:pull";
    assert_eq!(
        asked,
        [
            format!("HEAD {url} 1"),
            token.to_owned(),
            format!("HEAD {url} 2")
        ]
    );
}

/// Where each image name is asked for.
const URLS: [(&str, &str); 4] = [
    (
        "alpine",
        "https://registry-1.docker.io/v2/library/alpine/manifests/3",
    ),
    ("ghcr.io/o/i", "https://ghcr.io/v2/o/i/manifests/3"),
    ("localhost/i", "https://localhost/v2/i/manifests/3"),
    ("reg:5000/i", "https://reg:5000/v2/i/manifests/3"),
];

#[test]
fn a_bare_name_is_a_docker_library_image_and_a_host_is_kept() {
    let oci = hub(INDEX, DIGEST);
    for (image, url) in URLS {
        assert!(oci.digest(image, "3").is_ok());
        let asked = oci.0.asked.borrow().iter().any(|a| a.contains(url));
        assert!(asked, "{image}");
    }
}

#[test]
fn a_single_arch_manifest_or_a_bad_digest_is_refused() {
    let one = hub("application/vnd.oci.image.manifest.v1+json", DIGEST)
        .digest("nixos/nix", "1");
    assert!(
        matches!(&one, Err(FetchError::Resolve { why, .. }) if why.contains("not a multi-arch index")),
        "{one:?}"
    );
    let bad = hub(INDEX, "sha256:abc").digest("nixos/nix", "1");
    assert!(
        matches!(&bad, Err(FetchError::Resolve { why, .. }) if why.contains("not sha256")),
        "{bad:?}"
    );
}

#[test]
fn an_unsafe_name_or_tag_never_reaches_the_network() {
    for (image, tag) in [
        ("Nix", "1"),
        ("a b", "1"),
        ("", "1"),
        ("nixos/nix", "1;rm"),
        ("nixos/nix", ""),
    ] {
        let oci = hub(INDEX, DIGEST);
        assert!(
            matches!(oci.digest(image, tag), Err(FetchError::Name { .. })),
            "{image}:{tag}"
        );
        assert!(oci.0.asked.borrow().is_empty());
    }
}

/// A registry that answers `status` with no challenge or token.
struct Plain(u16, Vec<(String, String)>);

impl Http for Plain {
    fn head(&self, _: &str, _: &[String]) -> Result<Response, String> {
        Ok(Response {
            status: self.0,
            headers: self.1.clone(),
        })
    }
    fn get(&self, _: &str) -> Result<String, String> {
        Ok("{}".into())
    }
}

#[test]
fn a_failed_answer_names_why() {
    let why = |p: Plain| match Oci(p).digest("nixos/nix", "1") {
        Err(FetchError::Resolve { why, .. }) => why,
        other => format!("{other:?}"),
    };
    assert_eq!(why(Plain(404, Vec::new())), "registry answered 404");
    assert_eq!(why(Plain(401, Vec::new())), "401 without a challenge");
    let no_realm =
        vec![("www-authenticate".into(), "Bearer service=\"x\"".into())];
    assert_eq!(why(Plain(401, no_realm)), "challenge names no realm");
    let realm = vec![(
        "www-authenticate".into(),
        "Bearer realm=\"https://a\"".into(),
    )];
    assert_eq!(why(Plain(401, realm)), "no token in the answer");
}
