pub fn parse_ints(ints: Vec<String>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for item in ints.iter() {
        match item.parse::<i32>() {
            Ok(v) => {
                res.push(v);
            },
            Err(e) => {},
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_it_comply() {
        let ints = parse_ints(
            vec!["1", "2", "a", "3"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        println!("{:?}", ints);
        assert_eq!(&ints[..], [1, 2, 3]);
    }
}
