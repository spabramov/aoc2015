use std::fs::read_to_string;

use anyhow::anyhow;

pub fn count_floor(filename: &str) -> anyhow::Result<i32> {
    Ok(read_to_string(filename)?
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            '\n' => 0,
            c => panic!("unsupported char {c:?}"),
        })
        .sum())
}

pub fn find_basement(filename: &str) -> anyhow::Result<usize> {
    let steps = read_to_string(filename)?;
    let steps = steps.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        '\n' => 0,
        c => panic!("unsupported char {c:?}"),
    });

    let mut floor = 0;
    let mut num = 0;
    for s in steps {
        floor += s;
        num += 1;
        if floor < 0 {
            return Ok(num);
        }
    }
    Err(anyhow!("no basement"))
}

#[cfg(test)]
mod test {
    use super::count_floor;

    #[test]
    fn not_quite_lisp() {
        let value = count_floor("data/1.txt").unwrap();
        assert_eq!(value, 138);
    }

    #[test]
    fn find_basement() {
        let value = super::find_basement("data/1.txt").unwrap();
        assert_eq!(value, 0);
    }
}
