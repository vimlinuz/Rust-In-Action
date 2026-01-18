use std::collections::HashSet;

struct Difference<'first, 'second> {
    first_only: Vec<&'first str>,
    second_only: Vec<&'second str>,
}

fn find_difference<'fst, 'snd>(
    sentence1: &'fst str,
    sentence2: &'snd str,
) -> Difference<'fst, 'snd> {
    let sentence_1_words: HashSet<&str> = sentence1.split(" ").collect();
    let sentence_2_words: HashSet<&str> = sentence2.split(" ").collect();
    let mut first_only = Vec::new();
    let mut second_only = Vec::new();

    sentence_1_words.iter().for_each(|item| {
        if !sentence_2_words.contains(*item) {
            first_only.push(*item);
        }
    });
    first_only.sort();

    sentence_2_words.iter().for_each(|item| {
        if !sentence_1_words.contains(*item) {
            second_only.push(*item);
        }
    });

    Difference {
        first_only: first_only,
        second_only: second_only,
    }
}

fn main() {
    let first_sentence = String::from("I love the surf and the sand.");
    let second_sentence = String::from("I hate the surf and the sand.");

    let first_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&first_sentence, &third_sentence);
        diff.first_only
    };

    assert_eq!(first_only, vec!["love", "surf"]);

    let second_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&third_sentence, &second_sentence);
        diff.second_only
    };

    assert_eq!(second_only, vec!["surf"]);
}
