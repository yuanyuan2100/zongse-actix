use linkify::LinkFinder;

// Find links in text; Convert LinkFinder::Link to Vec<str>
pub fn link_finder_str(input: &str) -> Vec<String> {
    let mut links_str = Vec::new();
    let finder = LinkFinder::new();
    let links: Vec<_> = finder.links(input).collect();

    for link in links.iter() {
        links_str.push(link.as_str().to_string());
    }
    links_str
}

// add html tags to link
pub fn link_html_converter(links: &Vec<String>) -> Vec<String> {
    let mut link_html = Vec::new();

    for link in links.iter() {
        if link.ends_with(".jpg") || link.ends_with(".png") {
            link_html.push("<img src=".to_owned() + link + &" width='800'> ");
        } else if link.ends_with(".gif") {
            link_html.push("<img src=".to_owned() + link + &"> ");
        } else {
            link_html.push("<a target='_blank' href='".to_owned() + link + &"'>" + link + &"</a> ");
        }
    }
    link_html
}

// Convert links in text to clickable url
pub fn url_in_text(input: &str, mut links_str: Vec<String>, mut links_html: Vec<String>) -> String {
    let text_workingon = &input.to_string();

    if links_str.len() == 0 {
        text_workingon.to_string()
    } else {
        let text_workingon = &input.replace(&links_str[0], &links_html[0]).to_string();

        links_str.remove(0);
        links_html.remove(0);

        url_in_text(&text_workingon, links_str, links_html)
    }
}

pub fn url_converter(input: &str) -> String {
    let links = link_finder_str(&input);
    let links_html = link_html_converter(&links);

    url_in_text(&input, links, links_html)
}
