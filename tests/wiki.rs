use ini::Ini;

#[test]
fn wiki() {
    let str = std::fs::read_to_string("tests/wiki.ini").unwrap();
    let ini = Ini::from_str(&str).unwrap();
    assert_eq!(ini["owner"]["name"], "John Doe");
    assert_eq!(ini["owner"]["organization"], "Acme Widgets Inc.");
    assert_eq!(ini["database"]["server"], "192.0.2.62");
    assert_eq!(ini["database"]["port"], "143");
    assert_eq!(ini["database"]["file"], "payroll.dat");
}
