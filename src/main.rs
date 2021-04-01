use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request, RequestExt,
};

mod utils;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[allow(dead_code)]
fn search(_cmd: String) -> String {
    todo!()
}

#[lambda(http)]
#[tokio::main]
async fn main(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let query_string = request.query_string_parameters();
    let cmd = query_string.get("cmd").unwrap_or_else(|| "");
    let command = utils::get_command_from_query_string(&cmd);

    // Keep in alphabetic order
    let redirect_url = match command {
        "cal" => String::from("https://calendar.google.com/"),
        "ck" => String::from("https://app.convertkit.com/"),
        "clojurefam" | "cljfam" => String::from("https://github.com/athensresearch/clojurefam"),
        "drive" => String::from("https://drive.google.com/"),
        "dp" | "disney" | "disneyplus" => String::from("https://disneyplus.com"),
        "figma" => String::from("https://figma.com"),
        "foam" => String::from("https://foambubble.github.io/foam"),
        "g" => utils::google::construct_google_search_url(&cmd),
        "gh foam" => String::from("https://github.com/foambubble/foam"),
        "gh" => utils::github::construct_github_url(&cmd),
        "hey" => String::from("https://app.hey.com/"),
        "jp" => String::from("https://joeprevite.com"),
        "ip" => String::from("https://instapaper.com"),
        "ih" => String::from("https://indiehackers.com"),
        "l3" => String::from("http://localhost:3000/"),
        "l8" => String::from("http://localhost:8000/"),
        "lh" => utils::localhost::construct_localhost_url(&cmd),
        "mail" => String::from("https://mail.google.com/"),
        "map" | "maps" => String::from("https://maps.google.com/"),
        "photo" | "photos" => String::from("https://photos.google.com/"),
        "og" => String::from("https://onegraph.com/"),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        // If no match, we search on Google
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Ok(redirect_url)
}
