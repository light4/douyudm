use serde_json::{json, Value};

fn escape(v: &str) -> String {
    v.replace('@', "@A").replace('/', "@S")
}

fn unescape(v: &str) -> String {
    v.replace("@S", "/").replace("@A", "@")
}

pub fn serialize(msg: &Value) -> String {
    match msg {
        Value::Object(map) => map
            .iter()
            .map(|(k, v)| format!("{}@={}", k, serialize(v)))
            .collect::<Vec<String>>()
            .join(""),
        Value::Array(arr) => arr.iter().map(serialize).collect::<Vec<String>>().join(""),
        Value::String(s) => escape(s) + "/",
        Value::Number(n) => escape(&n.to_string()) + "/",
        _ => "".to_string(),
    }
}

pub fn deserialize(data: &str) -> Value {
    if data.contains("//") {
        let msg = data
            .split("//")
            .filter(|i| !i.is_empty())
            .map(deserialize)
            .collect();
        return Value::Array(msg);
    }

    if data.contains("@=") {
        let mut obj = json!({});
        data.split('/')
            .filter(|i| !i.is_empty())
            .map(|s| {
                let mut splited = s.split("@=");
                let k = splited.next().unwrap();
                let v = splited.next().unwrap_or_default();
                obj[k] = deserialize(v);
            })
            .for_each(drop);
        return obj;
    }

    Value::String(unescape(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(escape("login@req"), "login@Areq");
        assert_eq!(escape("login/req"), "login@Sreq");
    }

    #[test]
    fn test_unescape() {
        assert_eq!("login@req", unescape("login@Areq"));
        assert_eq!("login/req", unescape("login@Sreq"));
    }

    #[test]
    fn test_serialize() {
        assert_eq!(
            serialize(&json!({"type": "login@req", "roomid": 9999})),
            "type@=login@Areq/roomid@=9999/"
        );
        assert_eq!(serialize(&json!([1, 3, 3])), "1/3/3/");
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            json!({ "type": "login@req", "roomid": "9999" }),
            deserialize("type@=login@Areq/roomid@=9999/")
        );
        assert_eq!(json!(['1', '3', '3']), deserialize("1//3//3"));
    }
}
