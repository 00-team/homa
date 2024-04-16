#[derive(Debug)]
pub enum StartArg {
    Invite { id: i64 },
    None,
}

pub fn parse_start_args(arg: &str) -> StartArg {
    let mut value = arg.split("-");
    let key = if let Some(k) = value.next() {
        k
    } else {
        return StartArg::None;
    };

    match key {
        "invite" => {
            if let Some(id) = value.next() {
                if let Ok(id) = id.parse::<i64>() {
                    return StartArg::Invite { id };
                }
            }

            StartArg::None
        }
        _ => StartArg::None,
    }
}
