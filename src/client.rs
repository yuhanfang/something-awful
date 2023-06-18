use crate::{post::Post, thread::Thread, Error};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use std::{
    io::{BufRead, Write},
    sync::Arc,
};
use url::Url;

pub struct Client {
    base: Url,
    client: reqwest::Client,
    cookie_store: Arc<CookieStoreMutex>,
}

/// References a forum user.
pub enum User<'a> {
    /// The current logged-in user.
    CurrentUser,

    /// A user ID.
    UserID(&'a str),

    /// A username.
    Username(&'a str),
}

/// References a page of posts.
pub enum PostIndex {
    /// The beginning of the posts.
    First,

    /// The last posts.
    Last,

    /// New posts.
    New,

    /// A specific page of posts.
    Page(usize),
}

/// Public or private user profile.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub userid: i64,
    pub username: String,
    pub homepage: String,
    pub icq: String,
    pub aim: String,
    pub yahoo: String,
    pub gender: String,
    pub usertitle: String,
    pub joindate: i64,
    pub lastpost: i64,
    pub posts: i64,
    pub receivepm: i64,
    pub postsperday: f64,
    pub role: String,
    pub biography: String,
    pub location: String,
    pub interests: String,
    pub occupation: String,
    pub picture: String,
    pub avpath: String,
}

impl Client {
    pub fn new() -> Result<Client, Error> {
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));
        Ok(Client {
            base: Url::parse("https://forums.somethingawful.com")?,
            client: reqwest::Client::builder()
                .cookie_provider(cookie_store.clone())
                .build()?,
            cookie_store,
        })
    }

    /// Attempts to login. Returns ReqwestError on a communication error or
    /// LoginError if the login request failed.
    pub async fn login(&self, username: &str, password: &str) -> Result<(), Error> {
        let response = self
            .client
            .post(self.base.join("account.php?json=1")?)
            .form(&[
                ("action", "login"),
                ("username", username),
                ("password", password),
                ("next", "/index.php?json=1"),
            ])
            .send()
            .await?;

        if response.error_for_status().is_err() {
            Err(Error::LoginError)
        } else {
            Ok(())
        }
    }

    /// Returns the profile of a user. If there is no user, returns None.
    pub async fn fetch_profile<'a>(&self, user: User<'a>) -> Result<Option<Profile>, Error> {
        let query = match user {
            User::CurrentUser => vec![("action", "getinfo"), ("json", "1")],
            User::UserID(userid) => vec![("action", "getinfo"), ("userid", userid), ("json", "1")],
            User::Username(username) => {
                vec![("action", "getinfo"), ("username", username), ("json", "1")]
            }
        };
        let response = self
            .client
            .get(self.base.join("member.php")?)
            .query(&query)
            .send()
            .await?;

        // If the username doesn't exist, we get an HTML page that will decode
        // incorrectly.
        match response.json().await {
            Ok(res) => Ok(res),
            Err(err) => {
                if err.is_decode() {
                    Ok(None)
                } else {
                    Err(err.into())
                }
            }
        }
    }

    pub async fn fetch_posts(&self, thread_id: &str, index: PostIndex) -> Result<Vec<Post>, Error> {
        let mut _page_string = None;
        let query = match index {
            PostIndex::First => {
                vec![("threadid", thread_id), ("perpage", "40")]
            }
            PostIndex::Last => {
                vec![
                    ("threadid", thread_id),
                    ("perpage", "40"),
                    ("goto", "lastpost"),
                ]
            }
            PostIndex::New => {
                vec![
                    ("threadid", thread_id),
                    ("perpage", "40"),
                    ("goto", "newpost"),
                ]
            }
            PostIndex::Page(page) => {
                _page_string = Some(format!("{page}"));
                vec![
                    ("threadid", thread_id),
                    ("perpage", "40"),
                    ("pagenumber", _page_string.as_ref().unwrap()),
                ]
            }
        };
        let response = self
            .client
            .get(self.base.join("showthread.php")?)
            .query(&query)
            .send()
            .await?
            .text()
            .await?;

        Post::parse(&response)
    }

    /// Returns all bookmarked threads.
    pub async fn fetch_bookmarked_threads(&self) -> Result<Vec<Thread>, Error> {
        let mut bookmarked_threads = Vec::new();
        let mut page = 1;
        loop {
            let response = self
                .client
                .get(self.base.join("bookmarkthreads.php")?)
                .query(&[
                    ("action", "view"),
                    ("perpage", "40"),
                    ("pagenumber", &format!("{page}")),
                ])
                .send()
                .await?
                .text()
                .await?;

            let mut threads = Thread::parse(&response)?;
            let fetch_next = threads.len() == 40;
            bookmarked_threads.append(&mut threads);
            if fetch_next {
                page += 1;
            } else {
                break;
            }
        }
        Ok(bookmarked_threads)
    }

    /// Saves credentials to JSON. The user must be logged in for the
    /// credentials to be useful.
    pub fn save_credentials<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let store = self.cookie_store.lock().expect("BUG: lock failed");
        store.save_json(writer).map_err(Error::CookieIOError)?;
        Ok(())
    }

    /// Loads credentials from JSON. The JSON must have been written with
    /// save_credentials.
    pub fn load_credentials<R: BufRead>(&self, reader: R) -> Result<(), Error> {
        let loaded = CookieStore::load_json(reader).map_err(Error::CookieIOError)?;
        let mut store = self.cookie_store.lock().expect("BUG: lock failed");
        for cookie in loaded.iter_unexpired() {
            store.insert(cookie.clone(), &self.base)?;
        }
        Ok(())
    }
}
