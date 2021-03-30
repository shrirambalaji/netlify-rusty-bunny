use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn search(cmd: String) -> String {
    todo!()
}

#[lambda(http)]
#[tokio::main]
async fn main(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(format!(
        "🦀 hello {}",
        request
            .query_string_parameters()
            .get("cmd")
            .unwrap_or_else(|| "from rusty-bunny! 🦀 🧡 🐰")
    ))
}
