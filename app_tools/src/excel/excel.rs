/*!读写文件
 *
 */
use umya_spreadsheet;

// 写入 Excel
pub fn write(path: &str, sheet_name: &str, data_list: Vec<Vec<String>>) -> Result<(), String> {
    // 读取了文件路径
    let path = std::path::Path::new(path);
    // 新建文件
    let mut book = umya_spreadsheet::new_file();

    // 新建sheet
    // let _work_sheet = book.new_sheet("Sheet3");

    // use1:
    // book.get_sheet_by_name_mut("Sheet1")
    //     .unwrap()
    //     .get_cell_mut("A1")
    //     .set_value("TEST1");

    // use2:
    // book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");

    // use3:
    // book.get_sheet_mut(0)
    //     .get_cell_by_column_and_row_mut(1, 1)
    //     .set_value("TEST1");

    // 获取 sheet 页
    let mut sheet = book.get_sheet_by_name_mut(sheet_name);
    if let Err(e) = sheet {
        println!("{}, not found, create a new sheet! err: {}", sheet_name, e);
        sheet = book.new_sheet(sheet_name);
    }
    match sheet {
        Ok(sheet) => {
            for (i, rows) in data_list.iter().enumerate() {
                for (j, col) in rows.iter().enumerate() {
                    sheet
                        .get_cell_by_column_and_row_mut(&((j + 1) as u32), &((i + 1) as u32))
                        .set_value(col);
                }
            }
        }
        Err(err) => return Err(format!("{:?}", err)),
    }

    let w = umya_spreadsheet::writer::xlsx::write(&book, path);
    if let Err(err) = w {
        return Err(format!("{:?}", err));
    }
    Ok(())
}

// 读取 Excel
pub fn read(path: &str, sheet_name: &str) -> Result<Vec<Vec<String>>, String> {
    let path = std::path::Path::new(path);
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let sheet = book.get_sheet_by_name_mut(sheet_name);
    match sheet {
        Ok(sheet) => {
            return {
                // let s = sheet.get_row_breaks_mut();
                // println!("s:{:?}", s);
                let (cols, rows) = sheet.get_highest_column_and_row();

                let mut data_list = vec![];
                for row in 1..=rows {
                    let mut item_list = vec![];
                    for col in 1..cols {
                        let value = sheet.get_value_by_column_and_row(&col, &row);
                        item_list.push(value)
                    }
                    data_list.push(item_list);
                }
                Ok(data_list)
            }
        }
        Err(err) => return Err(format!("{:?}", err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let data_list = vec![
            vec![
                "City".to_string(),
                "State".to_string(),
                "Population".to_string(),
                "Latitude".to_string(),
                "Longitude".to_string(),
            ],
            vec![
                "Davidsons".to_string(),
                "AK".to_string(),
                "".to_string(),
                "65.2419444".to_string(),
                "-165.2716667".to_string(),
            ],
            vec![
                "Kenai".to_string(),
                "AK".to_string(),
                "7610".to_string(),
                "60.5544444".to_string(),
                "-151.2583333".to_string(),
            ],
            vec![
                "Oakman".to_string(),
                "AL".to_string(),
                "".to_string(),
                "33.7133333".to_string(),
                "-87.3886111".to_string(),
            ],
        ];
        let s = write("./zzzzzz.xlsx", "Sheet1", data_list);
        assert!(s.is_ok())
    }
    #[test]
    fn test_read() {
        let datas = read("./zzzzzz.xlsx", "Sheet1");
        assert!(datas.is_ok());
    }
}
