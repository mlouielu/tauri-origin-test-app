use futures::StreamExt;
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use warp::{sse::Event, Filter};

// create server-sent event
fn sse_counter(counter: u64) -> Result<Event, Infallible> {
    Ok(warp::sse::Event::default()
        .event("counter")
        .data(counter.to_string()))
}

#[tokio::main]
async fn main() {
    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);
    let routes = warp::path("ticks")
        .and(warp::get())
        .map(|| {
            let mut counter: u64 = 0;
            // create server event source
            let interval = interval(Duration::from_secs(1));
            let stream = IntervalStream::new(interval);
            let event_stream = stream.map(move |_| {
                counter += 1;
                sse_counter(counter)
            });
            // reply using server-sent events
            warp::sse::reply(event_stream)
        })
        .with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3050)).await;
}
