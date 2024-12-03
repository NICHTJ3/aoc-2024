advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut buffer = String::new();
    let mut multiplying = false;

    let mut arg0 = None;
    let mut products = vec![];

    for c in input.chars() {
        if buffer == "mul" {
            if c == '(' {
                multiplying = true;
                continue;
            }

            buffer.clear();
        }

        if multiplying && c == ',' {
            arg0 = Some(buffer.parse::<u32>().unwrap());
            buffer.clear();
            continue;
        }

        if multiplying && c == ')' {
            let arg1 = buffer.parse::<u32>().unwrap();
            products.push(arg0.unwrap() * arg1);

            buffer.clear();
            multiplying = false;
            continue;
        }

        buffer.push(c);

        if !multiplying && !buffer.starts_with(&"mul("[..buffer.len().clamp(0, 4)]) {
            buffer.clear();
        }

        // If we're multiplying and we find a character that's not a number we can clear the buffer
        // and stop trying since commas are handled above
        if multiplying && !c.is_ascii_digit() {
            buffer.clear();
            multiplying = false;
        }
    }

    Some(products.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut buffer = String::new();
    let mut multiplying = false;
    let mut can_multiply = true;

    let mut arg0 = None;
    let mut products = vec![];

    for c in input.chars() {
        if buffer == "do()" {
            can_multiply = true;
            buffer.clear();
        }

        if buffer == "don't()" {
            can_multiply = false;
            buffer.clear();
        }

        if buffer == "mul" && c == '(' {
            multiplying = true;
            buffer.clear();
            continue;
        }

        if multiplying && c == ',' {
            arg0 = Some(buffer.parse::<u32>().unwrap());
            buffer.clear();
            continue;
        }

        if multiplying && c == ')' {
            let arg1 = buffer.parse::<u32>().unwrap();

            if can_multiply {
                products.push(arg0.unwrap() * arg1);
            }

            buffer.clear();
            multiplying = false;
            continue;
        }

        buffer.push(c);

        // If we're not multiplying and we haven't found any tokens despite having buffered enough characters we can clear the buffer
        if !multiplying
            && !buffer.starts_with(&"mul("[..buffer.len().clamp(0, 4)])
            && !buffer.starts_with(&"do()"[..buffer.len().clamp(0, 4)])
            && !buffer.starts_with(&"don't()"[..buffer.len()])
        {
            buffer.clear();
        }

        // If we're multiplying and we find a character that's not a number we can clear the buffer
        if multiplying && !c.is_ascii_digit() {
            buffer.clear();
            multiplying = false;
        }
    }

    Some(products.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
