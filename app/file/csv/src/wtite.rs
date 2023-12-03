//! CSV 读写器
use serde::Serialize;
use std::error::Error;
use std::io;

/// 写到标准输出
pub fn writer_stdout() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    // Since we're writing records manually, we must explicitly write our
    // header record. A header record is written the same way that other
    // records are written.
    wtr.write_record(["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    wtr.flush()?;
    Ok(())
}

/// 写到文件
pub fn writer_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;

    wtr.write_record(["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    wtr.flush()?;
    Ok(())
}

// Note that structs can derive both Serialize and Deserialize!
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    city: &'a str,
    state: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

/// 从结构体数据写到文件
pub fn writer_file_by_struct(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;

    wtr.serialize(Record {
        city: "Davidsons Landing",
        state: "AK",
        population: None,
        latitude: 65.2419444,
        longitude: -165.2716667,
    })?;
    wtr.serialize(Record {
        city: "Kenai",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;
    wtr.serialize(Record {
        city: "Oakman",
        state: "AL",
        population: None,
        latitude: 33.7133333,
        longitude: -87.3886111,
    })?;

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_stdout() {
        let ok = writer_stdout().is_ok();
        assert!(ok)
    }

    #[test]
    fn test_writer_file() {
        let ok = writer_file("demo1.csv").is_ok();
        assert!(ok)
    }

    #[test]
    fn test_writer_file_by_struct() {
        let ok = writer_file_by_struct("demo2.csv").is_ok();
        assert!(ok)
    }
}
