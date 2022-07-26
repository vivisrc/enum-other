use enum_other::other;

#[other(u8)]
#[derive(Debug, PartialEq, Eq)]
enum Signal {
    Hangup = 1,
    Interrupt = 2,
    Quit = 3,
    IllegalInstruction = 4,
    BreakpointTrap = 5,
    Abort = 6,
    FloatingPointException = 8,
    Kill = 9,
    SegmentationFault = 11,
    BrokenPipe = 13,
    Alarm = 14,
    Terminate = 15,
}

fn main() {
    assert_eq!(Signal::from(9), Signal::Kill);
    assert_eq!(u8::from(Signal::Interrupt), 2);

    assert_eq!(Signal::from(7), Signal::Other(7));
    assert_eq!(u8::from(Signal::Other(19)), 19);
}

#[test]
fn run() {
    main()
}
