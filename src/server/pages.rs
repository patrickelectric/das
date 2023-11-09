use actix_web::{HttpRequest, HttpResponse};
use paperclip::actix::api_v2_operation;

use std::io::prelude::*;

use std::{ffi::OsStr, path::Path};

fn load_file(file_name: &str) -> String {
    // Load files at runtime only in debug builds
    if cfg!(debug_assertions) {
        let html_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/frontend/");
        let mut file = std::fs::File::open(html_path.join(file_name)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        return contents;
    }

    match file_name {
        "" | "index.html" => std::include_str!("../frontend/index.html").into(),
        _ => format!("File not found: {file_name:?}"),
    }
}

#[api_v2_operation]
pub fn root(req: HttpRequest) -> HttpResponse {
    let filename = match req.match_info().query("filename") {
        "" | "index.html" => "index.html",

        something => {
            //TODO: do that in load_file
            return HttpResponse::NotFound()
                .content_type("text/plain")
                .body(format!("Page does not exist: {something:?}"));
        }
    };
    let content = load_file(filename);
    let extension = Path::new(&filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    let mime = actix_files::file_extension_to_mime(extension).to_string();

    HttpResponse::Ok().content_type(mime).body(content)
}
