/// Returning hyper::MapErr and Result for future
/// https://users.rust-lang.org/t/returning-hyper-maperr-and-result-for-future/16389

fn main() {
    let (tx, rx) = channel::DownloadRequest(0);
    let downloader = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = &core.handle();
        let hyper_client = Client::configure()
            .connector(HttpsConnector::new(10, handle).unwrap())
            .build(handle);

        let downloads = rx.for_each(move |chunk| {
            let DownloadRequest { url, offset } = chunk.clone();

            // TODO: handle errors
            // let uri = url.parse().unwrap();
            if let Some(uri) = url.parse().ok() {
                let req = Request::new(Method::Get, uri);
                // The compiler doesn’t like that in my two branches I’m
                // returning a MapErr and a Result, even though they
                // both satisfy the IntoFuture trait that is required by
                // for_each. It works fine if i just use the
                // uri.parse().unwrap() and just return the MapErr.
                //
                // Take a look at Either - it allows you to return two
                // different types as long as they’re both Futures that
                // yield same error and item type.
                // https://docs.rs/futures/0.1.18/futures/future/enum.Either.html
                hyper_client
                    .request(req)
                    .and_then(move |success| {
                        info!("download");
                        Ok(())
                    })
                    .map_err(move |err| {
                        info!("error download: {:?}", err);
                        ()
                    })
            }
            Ok(())
        });

        core.run(downloads).unwrap();
    });
}
