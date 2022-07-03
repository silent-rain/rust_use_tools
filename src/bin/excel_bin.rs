/*!excel 写文件
 * [dependencies]
 * umya-spreadsheet = "0.7"
 */

use seasnft7::excel;

fn main() {
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
    let s = excel::write("./zzzzzz.xlsx", "Sheet1", data_list);
    assert!(s.is_ok());

    let data_list = excel::read("./zzzzzz.xlsx", "Sheet1");
    println!("{:?}", data_list);
}
