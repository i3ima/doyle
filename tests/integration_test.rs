use doyle::*;

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
    let doyle: DoyleData = DoyleBuilder::new("i3ima").load_json(Some(host)).build();
    assert_eq!(
        CheckResult {
            status: Status::Found,
            url: format!("{}{}", "https://vk.com/", doyle.username),
            execution_time: 0
        }
        .status,
        doyle.check_hosts()[0].status
    )
}
