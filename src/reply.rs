use crate::Error;

/// Reply parameters read from the server.
pub struct ReplyParams {
    action: String,
    threadid: String,
    formkey: String,
    form_cookie: String,
}

impl ReplyParams {
    /// Prepares a reply by parsing a reply form for metadata that must be
    /// sent back in the reply request.
    pub fn parse(document: &str) -> Result<ReplyParams, Error> {
        let document = scraper::Html::parse_document(document);

        let selector =
            scraper::Selector::parse(r#"form[name="vbform"]"#).expect("BUG: illegal selector");
        let Some(form) = document.select(&selector).next() else {
            return Err(Error::ReplyParsingError);
        };

        let selector =
            scraper::Selector::parse(r#"input[name="action"]"#).expect("BUG: illegal selector");
        let Some(action) = form.select(&selector).next() else {
            return Err(Error::ReplyParsingError);
        };
        let Some(action) = action.value().attr("value") else {
            return Err(Error::ReplyParsingError);
        };

        let selector =
            scraper::Selector::parse(r#"input[name="threadid"]"#).expect("BUG: illegal selector");
        let Some(threadid) = form.select(&selector).next() else {
            return Err(Error::ReplyParsingError);
        };
        let Some(threadid) = threadid.value().attr("value") else {
            return Err(Error::ReplyParsingError);
        };

        let selector =
            scraper::Selector::parse(r#"input[name="formkey"]"#).expect("BUG: illegal selector");
        let Some(formkey) = form.select(&selector).next() else {
            return Err(Error::ReplyParsingError);
        };
        let Some(formkey) = formkey.value().attr("value") else {
            return Err(Error::ReplyParsingError);
        };

        let selector = scraper::Selector::parse(r#"input[name="form_cookie"]"#)
            .expect("BUG: illegal selector");
        let Some(form_cookie) = form.select(&selector).next() else {
            return Err(Error::ReplyParsingError);
        };
        let Some(form_cookie) = form_cookie.value().attr("value") else {
            return Err(Error::ReplyParsingError);
        };

        Ok(ReplyParams {
            action: action.to_owned(),
            threadid: threadid.to_owned(),
            formkey: formkey.to_owned(),
            form_cookie: form_cookie.to_owned(),
        })
    }

    /// Consumes the builder and a reply, returning a form corresponding to the
    /// post reply payload.
    pub fn into_form(self, reply: Reply) -> Result<reqwest::multipart::Form, Error> {
        let (attachment_filename, attachment_contents) =
            reply.attachment.unwrap_or((String::new(), Vec::new()));
        Ok(reqwest::multipart::Form::new()
            .text("action", self.action)
            .text("threadid", self.threadid)
            .text("formkey", self.formkey)
            .text("form_cookie", self.form_cookie)
            .text("message", reply.message)
            .text("bookmark", if reply.bookmark { "yes" } else { "no" })
            .text("submit", "Submit Reply")
            .part(
                "attachment",
                reqwest::multipart::Part::bytes(attachment_contents)
                    .file_name(attachment_filename)
                    .mime_str("application/octet-stream")?,
            ))
    }
}

/// Represents a message that the user will reply with.
pub struct Reply {
    message: String,
    bookmark: bool,
    attachment: Option<(String, Vec<u8>)>,
}

impl Reply {
    /// Creates a reply with the given BBCode message.
    pub fn new(message: &str) -> Reply {
        Reply {
            message: message.to_owned(),
            bookmark: true,
            attachment: None,
        }
    }

    /// Sets whether the reply should trigger subscribing to the thread.
    /// Defaults to true.
    pub fn with_bookmark(mut self, bookmark: bool) -> Self {
        self.bookmark = bookmark;
        self
    }

    /// Sets a post attachment, overriding any existing attachment. Defaults to
    /// no attachment.
    pub fn with_attachment(mut self, filename: &str, contents: &[u8]) -> Self {
        self.attachment = Some((filename.to_owned(), contents.to_owned()));
        self
    }
}
