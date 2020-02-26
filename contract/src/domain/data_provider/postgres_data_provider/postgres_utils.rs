use postgres::Row;
use std::collections::HashMap;
use serde_json::Value;
use serde_json::json;

pub fn row_to_map(row: Row) -> HashMap<String, Value> {
    let columns = row.columns();
    println!("{:?}", columns);
    // let map: Map<&str, Value> = columns.into_iter()

    let _result: HashMap<String, Value> = HashMap::new();

    // let map: HashMap<String, Value> = columns.into_iter()
    columns.into_iter()
        .enumerate()
        .map(|(stream, c)| {
            // let column_name = c.name();
            let column_name = c.name().to_string();
            let type_name = c.type_().name();

            println!("{:?}", c);
            println!("{:?}", type_name);

            let value = match type_name {
                "bool" => {
                    let value = row.get::<usize, bool>(stream);
                    let json_value = json!(value);
                    Ok(json_value)
                }
                "char" => {
                    let value = row.get::<usize, i8>(stream);
                    let json_value = json!(value);
                    Ok(json_value)
                }
                "smallint" | "smallserial" => {
                    let value: i16 = row.get::<usize, i16>(stream);
                    let json_value = json!(value);
                    Ok(json_value)
                }
                "integer" | "serial" | "int4" => {
                    let value: i32 = row.get::<usize, i32>(stream);
                    let json_value = json!(value);
                    Ok(json_value)
                }
                "oid" => {
                    let value = row.get::<usize, u32>(stream);
                    let json_value = json!(value);
                    Ok(json_value)
                }
                "bigint" | "bigserial" => {
                    let value: i64 = row.get::<usize, i64>(stream);
                    let json_value = json!(value);
                    Ok(json_value)
                }
                "decimal" | "numeric" => unimplemented!(),
                "real" | "double precision" => unimplemented!(),
                "text" => {
                    let value: String = row.get(stream);
                    let json_value = json!(value);
                    Ok(json_value)
                }
                "bytea" => {
                    let result: Result<Vec<u8>, postgres::error::Error> = row.try_get(stream);
                    let json_value = match result.ok() {
                        Some(value) => json!(value),
                        _ => Value::Null
                    };

                    Ok(json_value)
                }
                _ => Err(format!("not implemented for column the {:?} with type {:?}", stream, type_name))
            }.unwrap();
            // result.insert(column_name, value);
            (column_name, value)
        }).collect()

    // result
}
