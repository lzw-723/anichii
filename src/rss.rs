use std::borrow::Cow;
use std::io::BufRead;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::name::QName;
use rss::{Channel, Error, Item};
use quick_xml::Reader;
use regex::Regex;
use crate::ani::Episode;

pub struct Torrent {
    link: String,
    content_length: u64,
    pub_date: String,
}

impl Torrent {
    pub fn link(&self) -> &str {
        self.link.as_str()
    }
    pub fn set_link<V>(&mut self, link: V)
    where
        V: Into<String>,
    {
        self.link = link.into();
    }
    pub fn content_length(&self) -> u64 {
        self.content_length
    }
    pub fn set_content_length(&mut self, content_length: u64) {
        self.content_length = content_length;
    }
    pub fn pub_date(&self) -> &str {
        self.pub_date.as_str()
    }
    pub fn set_pub_date<V>(&mut self, pub_date: V)
    where
        V: Into<String>,
    {
        self.pub_date = pub_date.into();
    }
}

impl Torrent {
    pub fn from_xml<'s, R: BufRead>(reader: &mut Reader<R>,
                                    element: &'s BytesStart<'s>, ) ->  Result<Self, Error>  {
        let mut torrent = Torrent {
            link: "".to_string(),
            content_length: 0,
            pub_date: "".to_string(),
        };
        for attr in element.attributes().with_checks(false).flatten() {
            match decode(attr.key.as_ref(), reader)?.as_ref() {
                "url" => torrent.link = attr_value(&attr, reader)?.to_string(),
                "length" => torrent.content_length = attr_value(&attr, reader)?.to_string().parse().unwrap(),
                "type" => torrent.pub_date = attr_value(&attr, reader)?.to_string(),
                _ => {}
            }
        }
        skip(element.name(), reader)?;
        Ok(torrent)
    }
}

fn decode<'s, B: BufRead>(
    bytes: &'s [u8],
    reader: &Reader<B>,
) -> Result<Cow<'s, str>, Error> {
    let text = reader.decoder().decode(bytes)?;
    Ok(text)
}

fn attr_value<'s, B: BufRead>(
    attr: &'s Attribute<'s>,
    reader: &Reader<B>,
) -> Result<Cow<'s, str>, Error> {
    let value = attr.decode_and_unescape_value(reader.decoder())?;
    Ok(value)
}

fn skip<B: BufRead>(end: QName<'_>, reader: &mut Reader<B>) -> Result<(), Error> {
    reader.read_to_end_into(end, &mut Vec::new())?;
    Ok(())
}
pub fn parse_rss(rss: &str) -> Vec<Episode> {
    let channel = Channel::read_from(rss.as_bytes()).unwrap();
    let items: Vec<Item> = channel.items().to_vec();
    let mut episodes: Vec<Episode> = Vec::new();
    for x in items {
        episodes.push(Episode::new(x.title().unwrap().to_string(), x.link().unwrap().to_string(),
                                   x.enclosure().unwrap().url().to_string(),
                                   x.description().unwrap().to_string()));
    }
    episodes
}

pub fn parse_title(title: &str) -> (&str, &str) {
    let re = Regex::new(r"\[(.*?)\] ([^-]*) - (\d+) (\[.*?\])*").unwrap();
    if let Some(captures) = re.captures(title) {
        let subtitle_group = captures.get(1).map_or("", |m| m.as_str());
        let title = captures.get(2).map_or("", |m| m.as_str());
        return (title, subtitle_group);
    } else {
        println!("未找到匹配的字幕组和标题。");
    }
    ("title", "title")
}
