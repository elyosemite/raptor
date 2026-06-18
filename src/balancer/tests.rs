use super::*;

#[test]
fn should_reject_empty_backends() {
    let result = Balancer::new(vec![]);

    assert!(result.is_err());
}

#[test]
fn should_cycle_through_backends_round_robin() {
    let mut balancer = Balancer::new(vec![
        "127.0.0.1:3001".to_string(),
        "127.0.0.1:3002".to_string(),
        "127.0.0.1:3003".to_string(),
    ])
    .unwrap();

    assert_eq!(balancer.next(), "127.0.0.1:3001");
    assert_eq!(balancer.next(), "127.0.0.1:3002");
    assert_eq!(balancer.next(), "127.0.0.1:3003");
    assert_eq!(balancer.next(), "127.0.0.1:3001");
}
