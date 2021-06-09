use serde::{Deserialize, Serialize};

/// Data
#[derive(Serialize, Deserialize)]
pub struct Data {
    /// 原始格式
    pub from: Option<String>,
    /// 目标格式
    pub to: Option<String>,
    /// 文本数据
    pub text: Option<String>,
}

impl Data {
    /// 格式转换
    pub fn convert(&self) -> String {
        if let Some(text) = self.text.as_ref() {
            if let Some(to) = self.to.as_ref() {
                let from;
                if let Some(f) = self.from.as_ref() {
                    if f.is_empty() {
                        from = Self::auto(text);
                    } else {
                        from = f.to_string();
                    }
                } else {
                    from = Self::auto(text);
                }

                return match from.to_lowercase().as_ref() {
                    "json" => match to.to_lowercase().as_ref() {
                        "yaml" => Self::cto_yaml(Self::from_json::<serde_yaml::Value>(text)),
                        "toml" => Self::cto_toml(Self::from_json::<toml::Value>(text)),
                        _ => text.to_string(),
                    },
                    "yaml" => match to.to_lowercase().as_ref() {
                        "json" => Self::cto_json(Self::from_yaml::<serde_json::Value>(text)),
                        "toml" => Self::cto_toml(Self::from_yaml::<toml::Value>(text)),
                        _ => text.to_string(),
                    },
                    "toml" => match to.to_lowercase().as_ref() {
                        "json" => Self::cto_json(Self::from_toml::<serde_json::Value>(text)),
                        "yaml" => Self::cto_yaml(Self::from_toml::<serde_yaml::Value>(text)),
                        _ => text.to_string(),
                    },
                    _ => text.to_string(),
                };
            }
        }
        "".to_string()
    }
}

impl Data {
    /// 判断文件格式
    pub fn auto(text: &str) -> String {
        // FIXME: 寻找更好的办法
        if serde_json::from_str::<serde_json::Value>(text).is_ok() {
            return "json".to_string();
        }
        if serde_yaml::from_str::<serde_yaml::Value>(text).is_ok() {
            return "yaml".to_string();
        }
        if toml::from_str::<toml::Value>(text).is_ok() {
            return "toml".to_string();
        }
        "".to_string()
    }

    /// 校验数据格式是否正确
    pub fn check(&self) -> bool {
        if let Some(text) = self.text.as_ref() {
            if let Some(f) = self.from.as_ref() {
                // FIXME: 寻找更好的办法
                return match f.to_lowercase().as_ref() {
                    "json" => serde_json::from_str::<serde_json::Value>(text).is_ok(),
                    // FIXME: json也被认为是yaml格式的
                    "yaml" => serde_yaml::from_str::<serde_yaml::Value>(text).is_ok(),
                    "toml" => toml::from_str::<toml::Value>(text).is_ok(),
                    _ => false,
                };
            }
        }
        false
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

    fn cto_json<T>(v: T) -> String
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        serde_json::to_string(&v).expect("JSON 序列化失败")
    }

    fn cto_yaml<T>(v: T) -> String
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        serde_yaml::to_string(&v).expect("YAML 序列化失败")
    }

    fn cto_toml<T>(v: T) -> String
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        toml::to_string(&v).expect("TOML 序列化失败")
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod test {
    use super::Data;

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
        let mut tser = "".to_string();
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
