use issueflow::gitlab::commands::{RobotCommand, parse_note_command};

#[test]
fn parse_note_command_accepts_start_dev_on_first_non_empty_line() {
    let parsed = parse_note_command("\n/start-dev\nplease continue");
    assert_eq!(parsed, Some(RobotCommand::StartDev));
}

#[test]
fn parse_note_command_rejects_unknown_commands() {
    let parsed = parse_note_command("/ship-it");
    assert_eq!(parsed, None);
}
