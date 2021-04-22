use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;

pub fn route_root() -> Vec<u8> {
    r#"<html><body>
    <strong>This is very simple HTTP server.</strong>
    <br/>
    It supports only GET request on following URIs:
    <ul>
        <a href="/"><tt>/</tt></a> - this root page
    </ul>
    <ul>
        <a href="/hello"><tt>/hello</tt></a> - prints Hello world! message.
    </ul>
    <ul>
        <a href="/lorem"><tt>/lorem</tt></a> - prints some lorem ipsum text.
    </ul>
    <ul>
        <a href="/joke"><tt>/joke</tt></a> - prints a random joke.
    </ul>
    </body></html>"#
        .into()
}

pub fn route_hello() -> Vec<u8> {
    "<html><body>Hello world!</body></html>".into()
}

pub fn route_lorem() -> Vec<u8> {
    r#"
    <p>Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Nullam rhoncus aliquam metus.
    Nullam eget nisl.
    Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur? Sed elit dui, pellentesque a, faucibus vel, interdum nec, diam.
    Lorem ipsum dolor sit amet, consectetuer adipiscing elit.
    Nulla quis diam.
    Nullam justo enim, consectetuer nec, ullamcorper ac, vestibulum in, elit.
    Nam sed tellus id magna elementum tincidunt.
    Donec iaculis gravida nulla.
    Praesent in mauris eu tortor porttitor accumsan.
    Aliquam erat volutpat.
    </p>
    "#.into()
}

static JOKES: [&str; 15] = [
    "An IPv4 address space walks into a bar, \"A strong CIDR please. I'm exhausted.\"",
    "A TCP packet walks into a bar \"I want a beer.\" Bartender responds \"You want a beer?\" Packet responds \"I want a beer.\"",
    "DNS servers must feel sad, nobody calls them by their name.",
    "I'd tell you the one about the CIDR block, but you're too classy.",
    "A UDP packet walks into a bar without a checksum. Nobody cares.",
    "Chuck Norris doesn't do TCP handshake - he does TCP roundhouse-kick to initiate the connection",
    "Doctor: What seems to be the problem? Router: It hurts when IP.",
    "I tried to come up with an IPv4 joke, but the good ones were all already exhausted.",
    "The best thing about UDP jokes is that I don't care if you get them or not.",
    "People who tell routing jokes always exceed their time-to-live.",
    "The problem with TCP/IP jokes is that when I tell them, all I want is an ACK but usually get FINs and RSTs",
    "I had a funny UDP joke to tell, but I lost it somewhere&#8230;",
    "The worst part about HTTP jokes is that you can never remember in which state you heard the last one.",
    "HTTP jokes are rarely better than OK",
    "I really don't GET HTTP 404 jokes.",
];

pub fn route_joke() -> Vec<u8> {
    let dist = Uniform::new(0, JOKES.len());
    let i = dist.sample(&mut thread_rng());

    let joke = JOKES[i];

    return format!(
        r#"<html><body>
    <strong>{}</strong>
    </br>
    </br>
    Credits: http://attrition.org/misc/ee/protolol.txt
    </body></html>"#,
        joke
    )
    .into();
}
