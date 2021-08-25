use watson::*;

#[test]
fn find_accounts() {
    let username = "i3ima";
    let hosts = vec![
        String::from("https://vk.com/"),
        String::from("https://github.com/"),
    ];
    let watson: WatsonData = Watson::new("i3ima", hosts);
    assert_eq!(
        CheckResult {
            status: Status::Found,
            url: watson.hosts[0].to_string() + &username.to_string()
        },
        watson.check_host(&watson.hosts[0])
    )
}
