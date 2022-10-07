//! Implements the Netscape Bookmark file format.
//!
//! See Also: [Netscape Bookmark File Format](https://learn.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa753582(v=vs.85))

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::marker::PhantomData;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bookmark {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Selection")]
    pub _selection: Option<String>,
    #[serde(rename = "Folder")]
    pub folder: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: usize,
}

pub struct NetscapeBookmarks<'a, S: HtmlState> {
    state: Box<BookmarksHtmlState<'a>>,
    phantom: PhantomData<S>,
}

impl<'a, S: HtmlState> Display for NetscapeBookmarks<'a, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.state.doctype)?;
        f.write_char('\n')?;

        f.write_str(&self.state.title)?;
        f.write_char('\n')?;

        f.write_str(&self.state.header)?;
        f.write_char('\n')?;

        f.write_str(&self.state.description_list)?;
        f.write_char('\n')
    }
}

impl<'a> NetscapeBookmarks<'a, Doctype> {
    pub fn new() -> NetscapeBookmarks<'a, Doctype> {
        let state = BookmarksHtmlState {
            doctype: "",
            title: "".to_string(),
            header: "".to_string(),
            description_list: "".to_string(),
        };

        Self {
            state: Box::new(state),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
struct BookmarksHtmlState<'a> {
    doctype: &'a str,
    title: String,
    header: String,
    description_list: String,
}

pub trait HtmlState {}

pub struct Doctype;

impl HtmlState for Doctype {}

pub struct Title;

impl HtmlState for Title {}

pub struct Header;

impl HtmlState for Header {}

pub struct DescriptionList(Vec<Bookmark>);

impl HtmlState for DescriptionList {}

pub struct Render;

impl HtmlState for Render {}

impl<'a> NetscapeBookmarks<'a, Doctype> {
    pub fn doctype(mut self) -> NetscapeBookmarks<'a, Title> {
        self.state.doctype = r#"<!DOCTYPE NETSCAPE-Bookmark-file-1>
    <!--This is an automatically generated file.
    It will be read and overwritten.
    Do Not Edit! -->"#;
        NetscapeBookmarks {
            state: self.state,
            phantom: PhantomData,
        }
    }
}

impl<'a> NetscapeBookmarks<'a, Title> {
    pub fn title(mut self, title: &str) -> NetscapeBookmarks<'a, Header> {
        self.state.title = format!("<Title>{title}</Title>");
        NetscapeBookmarks {
            state: self.state,
            phantom: PhantomData,
        }
    }
}

impl<'a> NetscapeBookmarks<'a, Header> {
    pub fn header(mut self, header: &str) -> NetscapeBookmarks<'a, DescriptionList> {
        self.state.header = format!("<H1>{header}</H1>");
        NetscapeBookmarks {
            state: self.state,
            phantom: PhantomData,
        }
    }
}

impl<'a> NetscapeBookmarks<'a, DescriptionList> {
    pub fn description_lists(
        mut self,
        folders: HashMap<String, Vec<Bookmark>>,
    ) -> NetscapeBookmarks<'a, Render> {
        let mut builder = String::new();
        builder.push_str("<DL>");
        builder.push('\n');

        for (folder, bookmarks) in folders {
            let outer_dt = format!("<DT><H3 FOLDED>{}</H3>", folder);
            builder.push_str(outer_dt.as_str());
            builder.push('\n');
            builder.push_str("<DL><p>");
            builder.push('\n');
            for bookmark in bookmarks {
                let dt = format!(r#"<DT><A HREF="{}" ADD_DATE="{}">{}</A>"#,
                                 bookmark.url,
                                 bookmark.timestamp,
                                 bookmark.title,
                );
                builder.push_str(dt.as_str());
                builder.push('\n');
            }
            builder.push_str("</DL><p>");
            builder.push('\n');
        }

        builder.push_str("</DL>");
        builder.push('\n');

        self.state.description_list = builder;
        NetscapeBookmarks {
            state: self.state,
            phantom: PhantomData,
        }
    }
}

impl<'a> NetscapeBookmarks<'a, Render> {
    pub fn render(self) -> String {
        self.to_string()
    }
}
