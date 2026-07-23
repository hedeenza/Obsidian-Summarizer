#![warn(clippy::pedantic)]
use regex::Regex;
// use std::time::Instant;
use std::collections::HashSet;

pub fn link_entities_new(mut text_vector: Vec<&str>) -> String {
    // // Start Benchmarking Timer
    // let program_start = Instant::now();
    //
    text_vector.push("");
    
    // Set the capital word regex pattern
    let capital_word = Regex::new(r"^[A-Z]").unwrap();

    // Set a vector of allowable filler words for Pointer 2 to move through
    let allowable_filler = ["of", "the", "from"];

    // Create a string to hold the linked text
    let mut linked_text = String::new();

    // Create a new vector to hold
    let mut previously_linked: HashSet<String> = HashSet::new();

    // Set the initial Pointer 1 index to the beginning
    let mut pointer1_index = 0;

    // While the Pointer 1 index is less than the length of the content vector...
    while pointer1_index < text_vector.len() {
        // If Pointer 1 hits an entity...
        if capital_word.is_match(text_vector[pointer1_index]) {
            // Create a new string to hold the entire entity to link
            let mut entity: String = String::new();
            // Set Pointer 2 to where Pointer 1 is
            let mut pointer2_index = pointer1_index;
            // While the Pointer 2 index is less than the length of the content vector...
            while pointer2_index < text_vector.len() {
                // If Pointer 2 hits the end of the entity...
                if !capital_word.is_match(text_vector[pointer2_index])
                    && !allowable_filler.contains(&text_vector[pointer2_index])
                {
                    // Push each word between Pointer 1 and Pointer 2 to a String
                    for word in &text_vector[pointer1_index..pointer2_index] {
                        let formatted = format!("{word} ");
                        entity.push_str(&formatted);
                    }

                    // Remove the final space from the entity
                    let entity = match entity.char_indices().next_back() {
                        Some((i, _)) => &entity[..i],
                        None => &entity,
                    };

                    if previously_linked.contains(entity) {
                        linked_text.push_str(&(entity.to_owned() + " "));
                        pointer1_index = pointer2_index - 1;
                        break
                    }

                    previously_linked.insert(entity.to_string());

                    // Manipulate the string to account for final commas, periods, etc.
                    if entity.find('.') == Some(entity.len() - 1) {
                        let stripped_entity = entity.replace('.', "");
                        let linked_entity = format!("[[{stripped_entity}]].");
                        linked_text.push_str(&(linked_entity + " "));
                    } else if entity.ends_with(',') {
                        let stripped_entity = entity.replace(',', "");
                        let linked_entity = format!("[[{stripped_entity}]],");
                        linked_text.push_str(&(linked_entity + " "));
                    } else if entity.ends_with("'s") {
                        let stripped_entity = entity.replace("'s", "");
                        let linked_entity = format!("[[{stripped_entity}]]'s");
                        linked_text.push_str(&(linked_entity + " "));
                    } else {
                        let linked_entity = format!("[[{entity}]]");
                        linked_text.push_str(&(linked_entity + " "));
                    }

                    // Create the variants of the Entity and add those to "previously_linked"
                    let strip_characters = vec![".", ",", "'s"];
                    for character in strip_characters {
                        let variant_removed = entity.replace(character, "");
                        previously_linked.insert(variant_removed);
                        let variant_added = entity.to_owned() + character;
                        previously_linked.insert(variant_added);
                    }

                    // Move Pointer 1 up to Pointer 2
                    pointer1_index = pointer2_index - 1;
                    break;
                }
                // Increment Pointer 2 by One
                pointer2_index += 1;
            }
            // Push the entity to the HashSet
            previously_linked.insert(entity);
        } else {
            // If it is not an entity, append to the linked text
            linked_text.push_str(&(text_vector[pointer1_index].to_owned() + " "));
        }
        // Increment Pointer 1 by One
        pointer1_index += 1;
    }
    // Stop benchmarking Timer
    // let program_duration = program_start.elapsed();
    // println!("Summary Linked in {program_duration:.2?}");
    linked_text.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_not_link_variants() {
        let input = String::from("Taiwan ; Taiwan, ; Taiwan.");
        let vector: Vec<&str> = input.split(' ').collect();
        let linked_input = link_entities_new(vector);
        assert_eq!("[[Taiwan]] ; Taiwan, ; Taiwan.", linked_input);
    }

    #[test]
    fn do_not_remove_periods_from_acronyms() {
        let input = String::from("N.A.T.O. and NATO");
        let vector: Vec<&str> = input.split(' ').collect();
        let linked_input = link_entities_new(vector);
        assert_eq!("[[N.A.T.O.]] and NATO", linked_input);
    }
}
