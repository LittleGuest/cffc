use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    // 原始格式
    pub from: Option<String>,
    // 转变的格式
    to: Option<String>,
    // 文本数据
    text: Option<String>,
}

impl Data {
    // 格式转换
    pub fn convert(&self) -> String {
        match self.from.clone().unwrap().to_lowercase().as_ref() {
            "json" => match self.to.clone().unwrap().to_lowercase().as_ref() {
                "yaml" => Self::to_yaml(Self::from_json::<serde_yaml::Value>(
                    &self.text.clone().unwrap(),
                )),
                "toml" => {
                    Self::to_toml(Self::from_json::<toml::Value>(&self.text.clone().unwrap()))
                }
                _ => self.text.clone().unwrap(),
            },
            "yaml" => match self.to.clone().unwrap().to_lowercase().as_ref() {
                "json" => Self::to_json(Self::from_yaml::<serde_json::Value>(
                    &self.text.clone().unwrap(),
                )),
                "toml" => {
                    Self::to_toml(Self::from_yaml::<toml::Value>(&self.text.clone().unwrap()))
                }
                _ => self.text.clone().unwrap(),
            },
            "toml" => match self.to.clone().unwrap().to_lowercase().as_ref() {
                "json" => Self::to_json(Self::from_toml::<serde_json::Value>(
                    &self.text.clone().unwrap(),
                )),
                "yaml" => Self::to_yaml(Self::from_toml::<serde_yaml::Value>(
                    &self.text.clone().unwrap(),
                )),
                _ => self.text.clone().unwrap(),
            },
            _ => self.text.clone().unwrap(),
        }
    }
}

impl Data {
    // 校验数据格式是否正确
    pub fn check(&self) -> bool {
        true
    }

    fn from_json<T>(text: &str) -> T
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        serde_json::from_str(text).expect("JSON 反序列化失败")
    }

    fn from_yaml<T>(text: &str) -> T
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        serde_yaml::from_str::<T>(text).expect("YAML 反序列化失败")
    }

    fn from_toml<T>(text: &str) -> T
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        toml::from_str(text).expect("TOML 反序列化失败")
    }

    fn to_json<T>(v: T) -> String
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        serde_json::to_string(&v).expect("JSON 序列化失败")
    }

    fn to_yaml<T>(v: T) -> String
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        serde_yaml::to_string(&v).expect("YAML 序列化失败")
    }

    fn to_toml<T>(v: T) -> String
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        toml::to_string(&v).expect("TOML 序列化失败")
    }
}

mod test {
    use std::collections::HashMap;

    use serde_transcode::transcode;

    use super::*;

    #[test]
    fn test_convert() {
        let c = Data {
            from: Some("yaml".to_string()),
            to: Some("toml".to_string()),
            text: Some(
                r#"
            package:
              name: cffc
              version: 0.1.0
              authors:
              - gopher9527 <gopher9527@gmail.com>
              edition: "2018"
            dependencies:
              actix-web: 3.3.2
              serde: 1.0.125
              serde_json: 1.0.64
              tera: 1.8.0
              tokio: 1.5.0
              toml: 0.5.8
            "#
                .to_string(),
            ),
        };

        let cs = c.convert();
        println!("{}", cs);
    }

    const TOML: &str = r#"
    [package]
    name = "cffc"
    version = "0.1.0"
    authors = ["gopher9527 <gopher9527@gmail.com>"]
    edition = "2018"
    [dependencies]
    actix-web = "3.3.2"
    serde = "1.0.125"
    serde_json = "1.0.64"
    tera = "1.8.0"
    tokio = "1.5.0"
    toml = "0.5.8"
    "#;

    const JSON: &str = r#"
    {
        "package":{
            "name":"cffc",
            "version":"0.1.0",
            "authors":[
                "gopher9527 <gopher9527@gmail.com>"
            ],
            "edition":"2018"
        },
        "dependencies":{
            "actix-web":"3.3.2",
            "serde":"1.0.125",
            "serde_json":"1.0.64",
            "tera":"1.8.0",
            "tokio":"1.5.0",
            "toml":"0.5.8"
        }
    }
    "#;

    const YAML: &str = r#"
    package:
      name: cffc
      version: 0.1.0
      authors:
        - gopher9527 <gopher9527@gmail.com>
      edition: "2018"
    dependencies:
      actix-web: 3.3.2
      serde: 1.0.125
      serde_json: 1.0.64
      tera: 1.8.0
      tokio: 1.5.0
      toml: 0.5.8
    "#;

    #[test]
    fn test() {
        // toml --> json
        let mut deserializer = toml::Deserializer::new(TOML);
        let mut serializer = serde_json::Serializer::new(std::io::stdout());
        serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

        let t = toml::from_str::<serde_json::Value>(TOML).unwrap();
        let ts = serde_json::to_string(&t).unwrap();
        println!("\n{}", ts);

        println!("=======================================");

        // toml --> yaml
        let mut deserializer = toml::Deserializer::new(TOML);
        let mut serializer = serde_yaml::Serializer::new(std::io::stdout());
        serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

        println!("=======================================");

        // json --> yaml
        let mut jde = serde_json::Deserializer::from_str(JSON);
        let mut jse = serde_yaml::Serializer::new(std::io::stdout());
        serde_transcode::transcode(&mut jde, &mut jse);

        println!("=======================================");

        // json --> toml
        let mut jde = serde_json::Deserializer::from_str(JSON);
        let mut tser = String::new();
        let mut tse = toml::Serializer::new(&mut tser);
        serde_transcode::transcode(&mut jde, &mut tse);
        println!("{}", tser);

        println!("=======================================");

        // yaml --> json
        let yjs = serde_yaml::from_str::<serde_json::Value>(YAML).unwrap();
        let yjs = serde_json::to_string(&yjs).unwrap();
        println!("{}", yjs);

        println!("=======================================");

        // yaml --> toml
        let yt = serde_yaml::from_str::<toml::Value>(YAML).unwrap();
        let ys = toml::to_string(&yt).unwrap();
        println!("{}", ys);
    }
}
