fn main() {
    // Configure vergen
    vergen::EmitBuilder::builder()
        .build_timestamp()
        .git_commit_date()
        .git_describe(true, true, None)
        .git_sha(true)
        .idempotent()
        .all_build()
        .emit()
        .expect("Unable to generate the cargo keys!");

    // set SKIP_BINDINGS=1 to skip typescript bindings generation
    if std::env::var("SKIP_BINDINGS").is_err() {
        generate_typescript_bindings();
    }
}

fn generate_typescript_bindings() {
    println!("cargo:warning='Generating bindings..")
}
