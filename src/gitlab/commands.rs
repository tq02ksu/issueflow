#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotCommand {
    Triage,
    Validate,
    StartDev,
    Verify,
    PrepareRelease,
    Release,
}

pub fn parse_note_command(note: &str) -> Option<RobotCommand> {
    note.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .and_then(|line| match line {
            "/triage" => Some(RobotCommand::Triage),
            "/validate" => Some(RobotCommand::Validate),
            "/start-dev" => Some(RobotCommand::StartDev),
            "/verify" => Some(RobotCommand::Verify),
            "/prepare-release" => Some(RobotCommand::PrepareRelease),
            "/release" => Some(RobotCommand::Release),
            _ => None,
        })
}
