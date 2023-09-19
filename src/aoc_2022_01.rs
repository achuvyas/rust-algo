use std::io::Result;
use std::str::FromStr;

fn read_chunks<T>(path: &str) -> Result<Vec<u32>>
where
    T: FromStr,
{
    let contents = std::fs::read_to_string(path)?;
    let mut chunk_sum = 0;
    let mut sums = Vec::new();

    for line in contents.lines() {
        if let Ok(value) = line.parse::<u32>() {
            chunk_sum += value;
        } else {
            // When an empty line is encountered, add the accumulated sum to the vector
            if !line.is_empty() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid input format",
                ));
            }
            sums.push(chunk_sum);
            chunk_sum = 0;
        }
    }

    // Add the final accumulated sum to the vector
    sums.push(chunk_sum);

    Ok(sums)
}

pub fn max_calorie() -> Result<u32> {
    let chunk_sums = read_chunks::<u32>("./data/aoc202201.txt")?;

    // Create a sorted vector of chunk_sums in descending order
    let mut sorted_sums: Vec<u32> = chunk_sums.iter().cloned().collect();
    sorted_sums.sort_by(|a, b| b.cmp(a));

    // Take the top 3 maximum values and sum them
    let sum_of_top3: u32 = sorted_sums.iter().take(3).sum();

    if sum_of_top3 == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No calorie values found.",
        ));
    }

    Ok(sum_of_top3)
}
