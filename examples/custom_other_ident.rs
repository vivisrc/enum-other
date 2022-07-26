use enum_other::other;

#[other(u8, Surround)]
#[derive(Debug, PartialEq, Eq)]
enum AudioChannels {
    Mono = 1,
    Stereo = 2,
}

fn main() {
    assert_eq!(AudioChannels::from(2), AudioChannels::Stereo);
    assert_eq!(u8::from(AudioChannels::Mono), 1);

    assert_eq!(AudioChannels::from(8), AudioChannels::Surround(8));
    assert_eq!(u8::from(AudioChannels::Surround(6)), 6);
}

#[test]
fn run() {
    main()
}
