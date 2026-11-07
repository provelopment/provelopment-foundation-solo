//! `BootstrapPreflightV1` exercised only through its inbound interface.

use solo_application::{
    BootstrapInputV1, BootstrapOutcomeV1, BootstrapPreflightService, BootstrapPreflightV1,
    BootstrapRefusalCode,
};

fn execute(entity_id: &str, base_url: &str) -> BootstrapOutcomeV1 {
    BootstrapPreflightService.execute(BootstrapInputV1::new(entity_id, base_url))
}

#[test]
fn accepts_a_valid_pair_through_the_interface() {
    let outcome = execute("entity-7", "https://EXAMPLE.com:443/alpha");
    let BootstrapOutcomeV1::Accepted(accepted) = outcome else {
        panic!("expected acceptance");
    };
    assert_eq!(accepted.entity_id().as_str(), "entity-7");
    assert_eq!(accepted.base().as_str(), "https://example.com/alpha");
}

#[test]
fn refuses_an_invalid_entity_id() {
    let outcome = execute("Main", "https://example.com");
    let BootstrapOutcomeV1::Refused(refused) = outcome else {
        panic!("expected refusal");
    };
    assert_eq!(refused.code(), BootstrapRefusalCode::EntityIdInvalid);
    assert_eq!(refused.code().as_str(), "PREFLIGHT_ENTITY_ID_INVALID");
    assert_eq!(refused.detail(), "invalid-start");
}

#[test]
fn refuses_a_malformed_base() {
    let outcome = execute("solo", "https://user:pw@example.com");
    let BootstrapOutcomeV1::Refused(refused) = outcome else {
        panic!("expected refusal");
    };
    assert_eq!(refused.code(), BootstrapRefusalCode::BaseInvalid);
    assert_eq!(refused.detail(), "credentials");
}

#[test]
fn refuses_an_unsupported_base() {
    let outcome = execute("solo", "https://xn--mnchen-3ya.example");
    let BootstrapOutcomeV1::Refused(refused) = outcome else {
        panic!("expected refusal");
    };
    assert_eq!(refused.code(), BootstrapRefusalCode::BaseUnsupported);
    assert_eq!(refused.detail(), "idna-label");
}

#[test]
fn reports_the_identity_refusal_first_when_both_values_are_wrong() {
    let outcome = execute("Main", "ftp://example.com");
    let BootstrapOutcomeV1::Refused(refused) = outcome else {
        panic!("expected refusal");
    };
    assert_eq!(refused.code(), BootstrapRefusalCode::EntityIdInvalid);
}

#[test]
fn identity_is_independent_of_the_authored_base() {
    let first = execute("entity-7", "https://example.com/alpha");
    let second = execute("entity-7", "https://other.example/beta");
    let third = execute("entity-7", "http://third.example:8080/x");

    let ids: Vec<String> = [first, second, third]
        .into_iter()
        .map(|outcome| match outcome {
            BootstrapOutcomeV1::Accepted(accepted) => accepted.entity_id().as_str().to_owned(),
            BootstrapOutcomeV1::Refused(_) => panic!("expected acceptance"),
        })
        .collect();

    assert_eq!(ids, ["entity-7", "entity-7", "entity-7"]);
}

#[test]
fn different_identities_are_preserved_for_one_base() {
    for entity_id in ["solo", "entity-7", "a", "team-alpha"] {
        let outcome = execute(entity_id, "https://example.com");
        let BootstrapOutcomeV1::Accepted(accepted) = outcome else {
            panic!("expected acceptance for {entity_id}");
        };
        assert_eq!(accepted.entity_id().as_str(), entity_id);
    }
}

#[test]
fn execution_is_deterministic_and_leaves_no_trace() {
    let first = execute("entity-7", "https://EXAMPLE.com:443/alpha");
    let second = execute("entity-7", "https://EXAMPLE.com:443/alpha");
    assert_eq!(
        first, second,
        "identical input must yield identical outcome"
    );

    let refused_first = execute("BAD", "https://example.com");
    let refused_second = execute("BAD", "https://example.com");
    assert_eq!(refused_first, refused_second);

    // The service is a stateless unit value, so there is no instance state that a
    // call could carry into the next one.
    let service = BootstrapPreflightService;
    assert_eq!(
        service.execute(BootstrapInputV1::new("solo", "http://example.org")),
        execute("solo", "http://example.org")
    );
}

#[test]
fn raw_input_is_preserved_for_the_caller_but_never_trusted() {
    let input = BootstrapInputV1::new("Bad ID", "not a url");
    assert_eq!(input.entity_id(), "Bad ID");
    assert_eq!(input.base_url(), "not a url");
    assert!(matches!(
        BootstrapPreflightService.execute(input),
        BootstrapOutcomeV1::Refused(_)
    ));
}
