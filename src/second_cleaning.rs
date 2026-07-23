use std::collections::HashSet;

pub fn second_cleaning_pass(linked_text: &mut String) -> String {

    let mut previously_okay: HashSet<String> = HashSet::new();

    let text_vector: Vec<&str> = linked_text.split(' ').collect();

    let mut cleaned_text = String::new();

    for word in text_vector {
        let unlinked = unlink_entity(word.to_string());
        let sliced_period = unlinked.replace('.', "");
        let sliced_comma = unlinked.replace(',', "");
        let relinked_period = format!("[[{sliced_period}]].");
        let relinked_comma = format!("[[{sliced_comma}]],");

        // Period inside the link, period is not part of an acronym
        if word.ends_with(".]]") && word.find('.') == Some(word.len() - 3) {
            if previously_okay.contains(&sliced_period) || previously_okay.contains(&relinked_period) {
                cleaned_text.push_str(&(unlinked.to_owned() + " "));
            } else {
                cleaned_text.push_str(&(relinked_period.to_owned() + " "));
            }
            // Comma inside the link
        } else if word.ends_with(",]]") {
            if previously_okay.contains(&sliced_comma) || previously_okay.contains(&relinked_comma) {
                cleaned_text.push_str(&(unlinked.to_owned() + " "));
            } else {
                cleaned_text.push_str(&(relinked_comma.to_owned() + " "));
            }
        // Otherwise
        } else {
            if previously_okay.contains(&sliced_period) || previously_okay.contains(&relinked_period) || 
                previously_okay.contains(&sliced_comma) || previously_okay.contains(&relinked_comma) {
                cleaned_text.push_str(&(unlinked.to_owned() + " "));
            } else {
                cleaned_text.push_str(&(word.to_owned() + " "));
            }
        }

        previously_okay.insert(word.to_string());
        previously_okay.insert(unlinked.to_string());
        previously_okay.insert(sliced_period.to_string());
        previously_okay.insert(sliced_comma.to_string());
        previously_okay.insert(relinked_period.to_string());
        previously_okay.insert(relinked_comma.to_string());

    }
    cleaned_text.trim().to_string()
}

fn unlink_entity(linked_entity: String) -> String {
    let unlinked;
    // Fully linked
    if linked_entity.starts_with('[') && linked_entity.ends_with(']') {
        unlinked = &linked_entity[2..linked_entity.len() - 2];
    // Initially linked
    } else if linked_entity.starts_with('[') && !linked_entity.ends_with(']') {
        unlinked = &linked_entity[2..linked_entity.len()];
    // Terminally linked
    } else if !linked_entity.starts_with('[') && linked_entity.ends_with(']') {
        unlinked = &linked_entity[0..linked_entity.len() - 2];
    // Unlinked
    } else {
        unlinked = &linked_entity;
    }

    unlinked.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unlinking Tests
    #[test]
    fn unlink_fully_linked() {
        let input = String::from("[[fully linked]]");
        let output = unlink_entity(input);
        assert_eq!("fully linked", output);
    }

    #[test]
    fn unlink_initially_linked() {
        let input = String::from("[[initially linked");
        let output = unlink_entity(input);
        assert_eq!("initially linked", output);
    }

    #[test]
    fn unlink_terminally_linked() {
        let input = String::from("terminally linked]]");
        let output = unlink_entity(input);
        assert_eq!("terminally linked", output);
    }

    #[test]
    fn pass_unlinked() {
        let input = String::from("unlinked entity");
        let output = unlink_entity(input);
        assert_eq!("unlinked entity", output);
    }

    // Second cleaning pass tests
    #[test]
    fn unlink_next_already_fixed() {
        let mut input = String::from("[[Beijing.]] [[Beijing]] [[Beijing]]");
        let output = second_cleaning_pass(&mut input);
        assert_eq!("[[Beijing]]. Beijing Beijing", output);
    }

    #[test]
    fn unlink_next_needing_fixing() {
        let mut input = String::from("[[China]] [[China.]] [[China.]]");
        let output = second_cleaning_pass(&mut input);
        assert_eq!("[[China]] China. China.", output);
    }

    #[test]
    fn unlink_next_already_fixed_comma() {
        let mut input = String::from("[[Beijing.]] [[Beijing]] [[Beijing]]");
        let output = second_cleaning_pass(&mut input);
        assert_eq!("[[Beijing]]. Beijing Beijing", output);
    }

    #[test]
    fn unlink_next_needing_fixing_comma() {
        let mut input = String::from("[[China]] [[China,]] [[China,]]");
        let output = second_cleaning_pass(&mut input);
        assert_eq!("[[China]] China, China,", output);
    }

}
