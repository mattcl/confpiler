use confpiler::FlatConfig;

fn main() {
    let (conf, _warnings) = FlatConfig::builder()
        .add_config("confpiler/examples/basic/config/default")
        .add_config("confpiler/examples/basic/config/production")
        .build()
        .expect("invalid config");

    println!("{:#?}", conf.items());
}
