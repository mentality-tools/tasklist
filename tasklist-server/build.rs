use actix_web_static_files::NpmBuild;

fn main() {
    NpmBuild::new("./web")
        .install().unwrap()
        .run("build").unwrap()
        .target("./web/build")
        .to_resource_dir()
        .build().unwrap();
}