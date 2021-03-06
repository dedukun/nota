use pulldown_cmark::{Parser, Options, html, Event, Tag::{Heading, Link}, CowStr};
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::convert::TryInto;
use std::path::{Path, PathBuf};

use anyhow::Result;
use std::fs;

#[derive(Debug)]
pub struct ParsedNota {
    title: String,
    contents_digest: String
}

pub fn parse_to_html( in_path: PathBuf) -> Result<String> {
    
    debug!("parse to HTML file {:?}", in_path);

    let buffer: String = fs::read_to_string(&in_path)?.parse()?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&buffer, options);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    Ok(html_output)
}


//pub fn parse(mut in_file: File) -> Box<ParsedNota>{
//
//    debug!("Parsing...");
//
//    //let mut link_range = None;
//
//    let mut title : Option<String> = None;
//
//    let mut links : Vec<LinkInfo> = Vec::new();
//
//    let mut last_event =  None;
//
//    let mut buffer = String::new();
//
//    in_file.read_to_string(&mut buffer).expect("Hum... ");
//
//    let mut options = Options::empty();
//    options.insert(Options::ENABLE_STRIKETHROUGH);
//
//    let callback = |a: &str, b: &str| {
//        // For some reason we get the same value in both variables
//        debug!("It CAME HERE");
//
//        None
//
//        //match database::get_uid_from_title(a) {
//        //    Ok(uid)  => {
//        //        return Some((format!("./{}.md", uid), String::from("")))
//        //    }
//        //    Err(_) => {
//        //        return None
//        //    }
//        //}
//    };
//
//    let parser = Parser::new_with_broken_link_callback(
//        &buffer, 
//        options,
//        Some(&callback)
//    );
//
//    //let mut iter = parser.into_offset_iter();
//    let mut iter = parser.into_iter();
//
//    for element in iter{
//        debug!("Elem: {:?}", &element);
//
//        match &element {
//            Event::Start(Heading(1)) => {
//                last_event = Some(element);
//            },
//            Event::Start(Heading(3)) => {
//                last_event = Some(element);
//            },
//            Event::Start(Link(_,_,_)) => {
//                last_event = Some(element);
//            },
//            Event::Text(t) => {
//                if let Some(Event::Start(Heading(1))) = last_event {
//                    title = Some(t.to_string());
//                    last_event = None;
//                }
//                if let Some(Event::Start(Heading(3))) = last_event {
//                    // Parse 
//                    last_event = None
//                }
//                if let Some(Event::Start(Link(_,dest,_title))) = last_event {
//                    debug!("Link -> {} To -> {}", t, dest);
//                    let dest = dest.to_string();
//                    let uid = Path::new(&dest);
//                    let uid: u32 = uid.file_stem().unwrap().to_str().unwrap().to_string().parse().unwrap();
//                    let link = LinkInfo {details: None , link_to_text: Some(t.to_string()), link_to_uid: Some(uid)};
//                    links.push(link);
//                    last_event = None
//                }
//            },
//            _ => ()
//        }
//    }
//
//    // let d = String::from("Ai Ai Ai");
//
//    //let demo = vec![Event::Start(Heading(3)), Event::Text(d.try_into().unwrap()), Event::End(Heading(3))]; 
//
//    //let vecs = iter.chain(demo.into_iter());
//
//    //let buffer = String::new();
//
//    //cmark(vecs, &mut buffer, None);
//
//    //let byte_pos = link_range.unwrap().start; 
//
//    //in_file.seek(SeekFrom::Start(byte_pos.try_into().unwrap())).expect("Hum");
//
//    //let text_to_write = format!("{} {}", crate::REVERSE_LINKS_HEADING_LEVEL, crate::REVERSE_LINKS_TEXT);
//
//    //let bytes_written = in_file.write(text_to_write.as_bytes()).expect("Hum...");
//
//    //let new_end_pos = byte_pos + bytes_written;
//
//    //in_file.set_len(new_end_pos.try_into().unwrap()).expect("Hum");
//
//    //in_file.flush().expect("Hum...");
//
//    let title = title.unwrap();
//
//    let info = ParsedNota { title, links };
//
//    debug!("Parsing... Done: {:?}", info);
//
//    Box::new(info)
//}


