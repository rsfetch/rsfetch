pub fn count_lines(data: Vec<u8>) -> usize {
    let mut count: usize = 0;

    // convert srcs from Vec<u8> to String
    let mut src = "".to_owned();
    for byte in data {
        src = format!("{}{}", src, byte as char);
    }

    let _ = src.split("\n").map(|_| count += 1).collect::<()>();
    count
}
