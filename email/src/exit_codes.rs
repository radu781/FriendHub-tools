use std::process::exit;

pub(crate) enum Exit<'a> {
    Ok,
    TemplateLoadFailure(&'a dyn std::error::Error),
    RenderError(&'a dyn std::error::Error),
    CannotSendEmail(&'a dyn std::error::Error),
}

impl<'a> Exit<'a> {
    fn value(self) -> (String, i32) {
        match self {
            Exit::Ok => ("Success".to_owned(), 0),
            Exit::TemplateLoadFailure(info) => {
                (format!("Could not load template files: {}", info), 1)
            }
            Exit::RenderError(info) => (format!("Render error: {}", info), 2),
            Exit::CannotSendEmail(info) => (format!("Could not send email: {}", info), 3),
        }
    }
}

pub(crate) fn exit_with_info(type_: Exit) -> ! {
    let (error_message, exit_code) = type_.value();
    eprintln!("{}", error_message);
    exit(exit_code)
}
