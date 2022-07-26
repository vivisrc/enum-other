use enum_other::other;

#[other(String)]
#[derive(Debug, PartialEq, Eq)]
enum HttpMethod {
    Get = "GET",
    Head = "HEAD",
    Post = "POST",
    Put = "PUT",
    Delete = "DELETE",
    Options = "OPTIONS",
    Patch = "PATCH",
}

fn main() {
    assert_eq!(String::from(HttpMethod::Put), "PUT");
    assert_eq!(HttpMethod::from("GET".to_string()), HttpMethod::Get);

    assert_eq!(
        String::from(HttpMethod::Other("CONNECT".to_string())),
        "CONNECT".to_string(),
    );
    assert_eq!(
        HttpMethod::from("TRACE".to_string()),
        HttpMethod::Other("TRACE".to_string()),
    );
}

#[test]
fn run() {
    main()
}
