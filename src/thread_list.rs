use crate::Error;

/// Represents a single thread within a list of threads.
#[derive(Debug)]
pub struct Thread {
    pub id: String,
    pub title: String,
    pub author_username: String,
    pub replies: i64,
    pub views: i64,
    pub last_post_date: String,
    pub last_post_username: String,

    /// Zero if there are no unread posts in this thread. Otherwise, the
    /// number of unread posts.
    pub unread: i64,
}

impl Thread {
    /// Parses all threads on a list of threads within a page.
    pub fn parse_list(document: &str) -> Result<Vec<Thread>, Error> {
        let mut threads = Vec::new();
        let document = scraper::Html::parse_document(document);
        let selector =
            scraper::Selector::parse(r#"tbody>tr.thread"#).expect("BUG: illegal selector");

        for thread in document.select(&selector) {
            let parsing_error = Error::ThreadParsingError(thread.inner_html());
            let Some(id) = thread.value().attr("id") else {
                    return Err(parsing_error);
                };
            if !id.starts_with("thread") {
                return Err(parsing_error);
            }
            let thread_id = id["thread".len()..].to_owned();
            let selector =
                scraper::Selector::parse(r#"a.thread_title"#).expect("BUG: illegal selector");
            let Some(title) = thread.select(&selector).next() else {
                    return Err(parsing_error);
                };
            let title = title.inner_html();

            let selector =
                scraper::Selector::parse(r#"td.author>a"#).expect("BUG: illegal selector");
            let Some(author) = thread.select(&selector).next() else {
                    return Err(parsing_error);
                };
            let author_username = author.inner_html();

            let selector =
                scraper::Selector::parse(r#"td.replies>a"#).expect("BUG: illegal selector");
            let Some(replies) = thread.select(&selector).next() else {
                    return Err(parsing_error);
                };
            let replies = replies.inner_html();
            let Ok(replies) = replies.parse() else {
                    return Err(parsing_error);
                };

            let selector = scraper::Selector::parse(r#"td.views"#).expect("BUG: illegal selector");
            let Some(views) = thread.select(&selector).next() else {
                    return Err(parsing_error);
                };
            let views = views.inner_html();
            let Ok(views) = views.parse() else {
                    return Err(parsing_error);
                };

            let selector =
                scraper::Selector::parse(r#"td.lastpost>div.date"#).expect("BUG: illegal selector");
            let Some(last_post_date) = thread.select(&selector).next() else {
                    return Err(parsing_error);
                };
            let last_post_date = last_post_date.inner_html();

            let selector =
                scraper::Selector::parse(r#"td.lastpost>a.author"#).expect("BUG: illegal selector");
            let Some(last_post_username) = thread.select(&selector).next() else {
                    return Err(parsing_error);
                };
            let last_post_username = last_post_username.inner_html();

            let selector =
                scraper::Selector::parse(r#"td.title>div.title_inner>div.lastseen>a.count>b"#)
                    .expect("BUG: illegal selector");
            let unread = thread
                .select(&selector)
                .next()
                .map(|x| x.inner_html())
                .unwrap_or(String::from("0"));
            let Ok(unread) = unread.parse() else {
                    return Err(parsing_error);
                };

            threads.push(Thread {
                id: thread_id,
                title,
                author_username,
                replies,
                views,
                last_post_date,
                last_post_username,
                unread,
            });
        }
        Ok(threads)
    }
}
