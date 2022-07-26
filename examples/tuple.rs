use enum_other::other;

#[other((u8, u8, u8))]
#[derive(Debug, PartialEq, Eq)]
enum Color {
    Black = (0, 0, 0),
    Red = (255, 0, 0),
    Green = (0, 255, 0),
    Blue = (0, 0, 255),
    Yellow = (255, 255, 0),
    Magenta = (255, 0, 255),
    Cyan = (0, 255, 255),
    White = (255, 255, 255),
}

fn main() {
    assert_eq!(<(u8, u8, u8)>::from(Color::Magenta), (255, 0, 255));
    assert_eq!(Color::from((0, 0, 255)), Color::Blue);

    assert_eq!(
        <(u8, u8, u8)>::from(Color::Other(255, 127, 0)),
        (255, 127, 0)
    );
    assert_eq!(Color::from((255, 0, 127)), Color::Other(255, 0, 127));
}

#[test]
fn run() {
    main()
}
