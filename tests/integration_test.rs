use watson::*;

#[test]
fn find_accounts() {
    let host = vec![(
        "vkontakte".to_string(),
        HostDetails {
            error_type: ErrorType::StatusCode,
            error_msg: None,
            url: "https://vk.com/{}".to_string(),
            url_probe: None,
        },
    )];
    let watson: WatsonData = WatsonBuilder::new("i3ima").load_json(Some(host)).build();
    assert_eq!(
        CheckResult {
            status: Status::Found,
            url: format!("{}{}", "https://vk.com/", watson.username),
            execution_time: 0
        }.status,
        watson.check_hosts(&watson.hosts)[0].status
    )
}
