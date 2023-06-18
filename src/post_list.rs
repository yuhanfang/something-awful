use crate::Error;

/// Represents a specific post within a thread.
#[derive(Debug)]
pub struct Post {
    pub id: String,
    pub index: i64,
    pub author_username: String,
    pub author_registration_date: String,
    pub post_date: String,
    pub post_body: String,
}

impl Post {
    // Parses all posts on a thread page.
    pub fn parse_list(document: &str) -> Result<Vec<Post>, Error> {
        let mut posts = Vec::new();
        let document = scraper::Html::parse_document(document);
        let selector = scraper::Selector::parse(r#"table.post"#).expect("BUG: illegal selector");

        for post in document.select(&selector) {
            let parsing_error = Error::PostParsingError(post.inner_html());
            let selector =
                scraper::Selector::parse(r#"table.post>tbody>tr"#).expect("BUG: illegal selector");
            let mut post_body = post.select(&selector);

            let Some(author_and_body) = post_body.next() else {
                return Err(parsing_error);
            };

            let Some(date_and_links) = post_body.next() else {
                return Err(parsing_error);
            };

            let Some(id) = post.value().attr("id") else {
                return Err(parsing_error);
            };

            if !id.starts_with("post") {
                return Err(parsing_error);
            }
            let id = id["post".len()..].to_owned();

            let Some(index) = post.value().attr("data-idx") else {
                return Err(parsing_error);
            };
            let Ok(index) = index.parse() else {
                return Err(parsing_error);
            };

            let selector =
                scraper::Selector::parse(r#"dl.userinfo>dt"#).expect("BUG: illegal selector");
            let Some(author_username) = author_and_body.select(&selector).next() else {
                return Err(parsing_error);
            };

            let mut author_username = author_username.text();
            let Some(author_username) = author_username.next() else {
                return Err(parsing_error);
            };
            let author_username = author_username.to_owned();

            let selector = scraper::Selector::parse(r#"dl.userinfo>dd.registered"#)
                .expect("BUG: illegal selector");
            let Some(author_registration_date) = author_and_body.select(&selector).next() else {
                return Err(parsing_error);
            };
            let author_registration_date = author_registration_date.inner_html();

            let selector =
                scraper::Selector::parse(r#"tr>td.postdate"#).expect("BUG: illegal selector");
            let Some(post_date) = date_and_links.select(&selector).next() else {
                return Err(parsing_error);
            };
            let Some(post_date) = post_date.text().last() else {
                return Err(parsing_error);
            };
            let post_date = post_date.trim().to_owned();

            let selector =
                scraper::Selector::parse(r#"td.postbody"#).expect("BUG: illegal selector");
            let Some(post_body) = author_and_body.select(&selector).next() else {
                return Err(parsing_error);
            };
            let post_body = post_body.inner_html();

            posts.push(Post {
                id,
                index,
                author_username,
                author_registration_date,
                post_date,
                post_body,
            });
        }

        Ok(posts)
    }
}
