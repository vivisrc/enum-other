use enum_other::other;

#[other(i16)]
#[derive(Debug, PartialEq, Eq)]
enum Digit {
    Thousandths = -3,
    Hundredths, // = -2
    Tenths,     // = -1
    Unit,       // = 0
    Tens,       // = 1
    Hundreds,   // = 2
    Thousands,  // = 3
}

fn main() {
    assert_eq!(Digit::from(2), Digit::Hundreds);
    assert_eq!(i16::from(Digit::Tenths), -1);

    assert_eq!(Digit::from(6), Digit::Other(6));
    assert_eq!(i16::from(Digit::Other(-4)), -4);
}

#[test]
fn run() {
    main()
}
