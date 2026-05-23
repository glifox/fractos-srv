use std::{env, fs, io::ErrorKind};
use loro::{ExportMode, LoroDoc};
use actix::{ActorContext, Handler, Recipient};
use guitite::{DocumentActor, Protocol, messages::{Disconnect, Response}};


#[derive(Debug, DocumentActor)]
#[document_actor(skip_disconnect)]
pub struct Persistance {
    path: String,
    doc: LoroDoc,
    server: Recipient<Response>,
}

impl Persistance {
    pub fn new(file: String, server: Recipient<Response>) -> Self {
        let base = match env::var("DIR") {
            Ok(s) => s,
            Err(_) => format!("/data"),
        };
        
        let path = format!("{base}/{file}.loro");
        
        let doc = LoroDoc::new();
        match fs::read(&path) {
            Ok(v) => _ = doc.import(&v),
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => (),
                    _ => log::error!("{{ file: {}, error: {:?} }}", &path, e),
                }
            },
        };

        Self { doc, server, path }
    }
}

impl Handler<Disconnect> for Persistance {
    type Result = ();

    fn handle(&mut self, _: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        let path = &self.path;
        
        match self.doc.export(ExportMode::Snapshot) {
            Ok(e) => _ = fs::write(&path, &e),
            Err(e) => log::error!("{{ file: {}, error: {:?} }}", &path, e),
        };
        
        ctx.stop();
    }
}

impl Protocol for Persistance {
    fn on_import(&self, _bytes: &Vec<u8>) {
        
    }
}
