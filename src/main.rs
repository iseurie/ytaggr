extern crate url;
extern crate regex;
use ::std::io::{self, Read};
use ::std::io::prelude::*;
use ::std::borrow::Cow;
use ::url::Url;
use ::regex::Regex;
use ::std::collections::LinkedList;

fn main() {
    let stdout = io::stdout();
    let mut buf = String::new();
    let mut id_lst = LinkedList::new();
    let _ = io::stdin().read_to_string(&mut buf);

    for s in buf.split_whitespace() {
        let uri = Url::parse(s).expect("URI parse");
        let domain = uri.host_str().expect("No URI host parsed");
        
        if let Some(video_ids) = 
             uri.query_pairs()
                .filter(|&(ref key, _)| key == &Cow::Borrowed("video_ids"))
                .map(|(_, val)| val)
                .next()
        {
            video_ids.split(",").for_each(|id| id_lst.push_back(id.to_owned()));
            continue
        }

        let id = if Regex::new(".*[.]?youtube.com").expect("regex").is_match(domain)
        {
            uri.query_pairs()
               .filter(|&(ref key, _)| key == &Cow::Borrowed("v"))
               .map(|(_, val)| val)
               .next()
               .expect("youtube.com links require video ID in `v` query parameter")
        } else {
            if domain != "youtu.be" {
                panic!("Not a YouTube URI");
            }
            let emsg = "youtu.be links require video ID in path";
            Cow::from(uri.path_segments().expect(emsg).next().expect(emsg))
        };

        id_lst.push_back(id.into_owned());
    }

    let mut hdl = stdout.lock();
    let _ = hdl.write(b"https://youtube.com/watch_videos?video_ids=");
    let len = id_lst.len() - 1;
    for (i, id) in id_lst.iter().enumerate() {
        let _ = hdl.write(id.as_bytes());
        if i != len {
            let _ = hdl.write(b",");
        }
    }
}
