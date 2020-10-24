mod generated {
    // There will be a function
    // pub fn attach<T>(app: &mut tide::Server<T>)
    // where T : 'static + Send + Sync + Clone
    include!(concat!(env!("OUT_DIR"), "/uni_build_generated.rs"));
}

fn main() -> Result<(), std::io::Error> {

    return async_std::task::block_on(async {
        // create regular tide::Server
        let mut app = tide::new();

        // Attach all generated pages to the tide::Server
        generated::attach(&mut app);

        // Start tide::Server
        app.listen("localhost:8080").await?;

        Ok(())
    });
}
