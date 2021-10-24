use std::fmt;

use rocket::serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

pub struct LogRequest {
    pub text: String,
    pub source: String,
}

impl LogRequest {
    pub fn new(text: &str, source: &str) -> LogRequest {
        LogRequest {
            text: text.to_string(),
            source: source.to_string(),
        }
    }
}

impl<'de> de::Deserialize<'de> for LogRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        enum Field {Text, Source}

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`text` or `source`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "text" => Ok(Field::Text),
                            "source" => Ok(Field::Source),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct LogRequestVisitor;

        impl<'de> Visitor<'de> for LogRequestVisitor {
            type Value = LogRequest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LogRequest")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<LogRequest, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let text = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let source = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(LogRequest::new(text, source))
            }

            fn visit_map<V>(self, mut map: V) -> Result<LogRequest, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut text = None;
                let mut source = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Text => {
                            if text.is_some() {
                                return Err(de::Error::duplicate_field("text"));
                            }
                            text = Some(map.next_value()?);
                        }
                        Field::Source => {
                            if source.is_some() {
                                return Err(de::Error::duplicate_field("source"));
                            }
                            source = Some(map.next_value()?);
                        }
                    }
                }
                let text = text.ok_or_else(|| de::Error::missing_field("text"))?;
                let source = source.ok_or_else(|| de::Error::missing_field("source"))?;
                Ok(LogRequest::new(text, source))
            }
        }

        const FIELDS: &'static [&'static str] = &["text", "source"];
        deserializer.deserialize_struct("LogRequest", FIELDS, LogRequestVisitor)
    }
}
