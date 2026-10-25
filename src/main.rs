use std::collections::HashMap;
use config::Config;

fn main()
{

    let settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("FRIEND"))
        .build()
        .unwrap();

    println!("{:?}", settings.try_deserialize::<HashMap<String, String>>().unwrap());
}
