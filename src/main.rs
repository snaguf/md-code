use pulldown_cmark::{Event, Options, Parser, Tag};

fn get_code<'a>(
    (mut events, in_block): (Vec<Event<'a>>, bool),
    event: Event<'a>,
) -> (Vec<Event<'a>>, bool) {
    let is_code = match event {
        Event::Start(Tag::CodeBlock(..)) => true,
        Event::End(Tag::CodeBlock(..)) => false,
        _ => in_block,
    };
    if in_block && is_code {
        events.push(event)
    };
    return (events, is_code);
}

fn main() {
    let md_input: &str =
        "This is markdown with codeblock:\n\n```\nfn main()\n```\n\nCodeblock above!";
    println!("Markdown: {}", md_input);

    let (events, _) =
        Parser::new_ext(md_input, Options::empty()).fold((Vec::new(), false), get_code);
    println!("Events: {:?}", events)
}
