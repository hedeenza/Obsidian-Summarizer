use regex::Regex;
use std::collections::HashSet;

pub fn link_entities_new(text_vector: &[String]) -> String {
    // Set the capital word regex pattern
    let capital_word = Regex::new(r"^[A-Z]").unwrap();

    // Set a vector of allowable filler words for Pointer 2 to move through
    let allowable_filler = vec!["of", "the", "from"];

    // Create a string to hold the linked text
    let mut linked_text = String::new();

    // Create a new vector to hold
    let mut previously_linked: Vec<String> = Vec::new();

    // Set the initial Pointer 1 index to the beginning
    let mut pointer1_index = 0;

    // While the Pointer 1 index is less than the length of the content vector...
    while pointer1_index < text_vector.len() {
        // If Pointer 1 hits a line with content in it...
        if capital_word.is_match(&text_vector[pointer1_index]) {
            // Create a new string to hold the entire entity to link
            let mut entity: String = String::new();
            // Set Pointer 2 to where Pointer 1 is
            let mut pointer2_index = pointer1_index;
            // While the Pointer 2 index is less than the length of the content vector...
            while pointer2_index < text_vector.len() {
                // If Pointer 2 hits a blank line...
                if !capital_word.is_match(&text_vector[pointer2_index]) && !allowable_filler.contains(&text_vector[pointer2_index].as_str()) {
                    // Push each line between Pointer 1 and Pointer 2 to a Vector
                    for word in &text_vector[pointer1_index..pointer2_index] {
                        entity.push_str(&word);
                    }

                    // Manipulate the string to account for final commas, periods, etc.
                    if entity.ends_with(".") {
                        let stripped_entity = entity.replace(".", "");
                        let linked_entity = format!("[[{}]].", stripped_entity);
                        linked_text.push_str(&(linked_entity + " "));
                    } else if entity.ends_with(",") {
                        let stripped_entity = entity.replace(",", "");
                        let linked_entity = format!("[[{}]],", stripped_entity);
                        linked_text.push_str(&(linked_entity + " "));
                    } else if entity.ends_with("'s") {
                        let stripped_entity = entity.replace("'s", "");
                        let linked_entity = format!("[[{}]]'s", stripped_entity);
                        linked_text.push_str(&(linked_entity + " "));
                    } else {
                        let linked_entity = format!("[[{}]]", entity);
                        linked_text.push_str(&(linked_entity + " "));
                    }
                    
                    // Create the variants of the Entity and add those to "previously_linked"
                    let strip_characters = vec![".", ",", "'s"];
                    for character in strip_characters {
                        let variant = entity.replace(character, "");
                        previously_linked.push(variant)
                    }

                    // Move Pointer 1 up to Pointer 2
                    pointer1_index = pointer2_index;
                    break;
                }
                // Increment Pointer 2 by One
                pointer2_index += 1;
            }
            // Push the tuple vector to the output vector
            previously_linked.push(entity);
        } else {
            // If it is not an entity, append to the linked text
            linked_text.push_str(&text_vector[pointer1_index]);
        }
        // Increment Pointer 1 by One
        pointer1_index += 1;
    }
    linked_text
}

pub fn link_entities(
    text_vector: Vec<&str>,
    window_size: usize,
    linked: &mut String,
    entity_detect: regex::Regex,
    mut previously_linked: HashSet<Vec<Vec<String>>>,
) {
    // Create a window to examine the text vector 4 words at a time
    let mut window = text_vector.windows(window_size);

    // While there are still viable "windows"...
    while let Some(index) = window.next() {
        // Create values to stand in for the true/false results of matches to keep if statements cleaner
        let w = entity_detect.is_match(index[0]);
        let x = entity_detect.is_match(index[1]);
        let y = entity_detect.is_match(index[2]);
        let z = entity_detect.is_match(index[3]);

        // Create vectors to hold variants of the linked items to prevent them from being linked
        // again if they appear again in other written contexts
        let mut outer_comma = Vec::new();
        let mut comma_vec = Vec::new();
        let mut outer_strip_comma = Vec::new();
        let mut strip_comma_vec = Vec::new();
        let mut outer_period = Vec::new();
        let mut period_vec = Vec::new();
        let mut outer_strip_period = Vec::new();
        let mut strip_period_vec = Vec::new();
        let mut outer_possessive = Vec::new();
        let mut possessive_vec = Vec::new();
        let mut outer_strip_possessive = Vec::new();
        let mut strip_possessive_vec = Vec::new();

        // If all 4 slots contain capital words...
        if w & x & y & z | w & (index[1] == "of") & y & z | w & x & (index[2] == "of") & z {
            // Create an empty vector to hold the values to compare to
            let mut tester = Vec::new();
            // Push the values in the window to the testing vector
            let slice: Vec<String> = vec![
                index[0].to_string(),
                index[1].to_string(),
                index[2].to_string(),
                index[3].to_string(),
            ];
            tester.push(slice.clone());

            // If this has already been linked, move the window to skip it
            if previously_linked.contains(&tester) {
                for word in slice {
                    linked.push_str(&word);
                    linked.push(' ');
                }
                window.next();
                window.next();
                window.next();
            // Otherwise...
            } else {
                // For each
                for (i, _content) in slice.clone().into_iter().enumerate() {
                    // Add the opening linking brackets, [[, to the first word
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked.push_str(&(linked_line + " "));
                    // Add the closing linking brackets, ]], to the last word
                    } else if i == slice.len() - 1 {
                        // If there's a possessive, add the closing linking brackets before the 's
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        // If there's a comma, add the closing linking brackets before the comma
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        // // If there's a period, add the closing linking brackets before the period
                        // } else if index[i].contains(".") {
                        //     let stripped_word = index[i].replace(".", "");
                        //     let linked_wordi = format!("{}]].", stripped_word);
                        //     let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                        //     linked.push_str(&(linked_linei + " "));
                        // Otherwise, link the last word normally
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        }
                    // Otherwise don't add any linking brackets to the word
                    } else {
                        let linked_word = index[i].to_string();
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked.push_str(&(linked_line + " "));
                    }

                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }

                // Add the tester contents to the previously linked items
                previously_linked.insert(tester);
                // Move the window past all the words in this window
                window.next();
                window.next();
                window.next();

                let linked_comma = format!("{} {},", index[0], index[1]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let linked_period = format!("{} {}.", index[0], index[1]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let linked_possessive = format!("{} {}'s", index[0], index[1]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                outer_strip_comma.push(strip_comma_vec);
                outer_strip_period.push(strip_period_vec);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);
            }
        // If the first 3 slots contain capital words...
        } else if w & x & y
            | w & (index[1] == "of") & y
            | w & (index[1] == "the") & y
            | w & (index[1] == "for") & y
        {
            let mut tester = Vec::new();
            let slice: Vec<String> = vec![
                index[0].to_string(),
                index[1].to_string(),
                index[2].to_string(),
            ];
            tester.push(slice.clone());
            if previously_linked.contains(&tester) {
                for word in slice {
                    linked.push_str(&word);
                    linked.push(' ');
                }
                window.next();
                window.next();
            } else {
                for (i, _content) in slice.clone().into_iter().enumerate() {
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked.push_str(&(linked_line + " "));
                    } else if i == slice.len() - 1 {
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        // } else if index[0].ends_with(".") {
                        //     let stripped_word = index[0].replace(".", "");
                        //     let linked_word0 = format!("[[{}]].", stripped_word);
                        //     let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                        //     linked.push_str(&(linked_line0 + " "));
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        }
                    } else {
                        let linked_word = index[i].to_string();
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked.push_str(&(linked_line + " "));
                    }
                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }
                previously_linked.insert(tester);
                window.next();
                window.next();

                let linked_comma = format!("{} {},", index[0], index[1]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let linked_period = format!("{} {}.", index[0], index[1]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let linked_possessive = format!("{} {}'s", index[0], index[1]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                outer_strip_comma.push(strip_comma_vec);
                outer_strip_period.push(strip_period_vec);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);
            }
        // If the first 2 slots contain capital words...
        } else if w & x {
            let mut tester = Vec::new();
            let slice: Vec<String> = vec![index[0].to_string(), index[1].to_string()];
            tester.push(slice.clone());
            if previously_linked.contains(&tester) {
                for word in slice {
                    linked.push_str(&word);
                    linked.push(' ');
                }
                window.next();
            } else {
                for (i, _content) in slice.clone().into_iter().enumerate() {
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked.push_str(&(linked_line + " "));
                    } else if i == slice.len() - 1 {
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        // } else if index[0].ends_with(".") {
                        //     let stripped_word = index[0].replace(".", "");
                        //     let linked_word0 = format!("[[{}]].", stripped_word);
                        //     let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                        //     linked.push_str(&(linked_line0 + " "));
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked.push_str(&(linked_linei + " "));
                        }
                    } else {
                        let linked_word = index[i].to_string();
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked.push_str(&(linked_line + " "));
                    }

                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }

                window.next();

                let linked_comma = format!("{} {},", index[0], index[1]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let linked_period = format!("{} {}.", index[0], index[1]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let linked_possessive = format!("{} {}'s", index[0], index[1]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                outer_strip_comma.push(strip_comma_vec);
                outer_strip_period.push(strip_period_vec);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);
            }
        // If only the first slots contains a capital word...
        } else if w {
            let mut tester = Vec::new();
            let slice: Vec<String> = vec![index[0].to_string()];
            tester.push(slice.clone());

            if previously_linked.contains(&tester) {
                for word in slice {
                    linked.push_str(&word);
                    linked.push(' ');
                }
                window.next();
            } else {
                if index[0].contains("'s") {
                    let stripped_word = index[0].replace("'s", "");
                    let linked_word0 = format!("[[{}]]'s", stripped_word);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked.push_str(&(linked_line0 + " "));
                } else if index[0].contains(",") {
                    let stripped_word = index[0].replace(",", "");
                    let linked_word0 = format!("[[{}]],", stripped_word);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked.push_str(&(linked_line0 + " "));
                // } else if index[0].ends_with(".") {
                //     let stripped_word = index[0].replace(".", "");
                //     let linked_word0 = format!("[[{}]].", stripped_word);
                //     let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                //     linked.push_str(&(linked_line0 + " "));
                } else {
                    let linked_word0 = format!("[[{}]]", index[0]);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked.push_str(&(linked_line0 + " "));
                }

                let linked_comma = format!("{},", index[0]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let strip_comma = index[0].replace(",", "");
                let linked_strip_comma = strip_comma.to_string();
                strip_comma_vec.push(linked_strip_comma);
                outer_strip_comma.push(strip_comma_vec);

                let linked_period = format!("{}.", index[0]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let strip_period = index[0].replace(".", "");
                let linked_strip_period = strip_period.to_string();
                strip_period_vec.push(linked_strip_period);
                outer_strip_period.push(strip_period_vec);

                let linked_possessive = format!("{}'s", index[0]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                let strip_possessive = index[0].replace("'s", "");
                let linked_strip_possessive = strip_possessive.to_string();
                strip_possessive_vec.push(linked_strip_possessive);
                outer_strip_possessive.push(strip_possessive_vec);

                previously_linked.insert(tester);
                previously_linked.insert(outer_comma);
                previously_linked.insert(outer_strip_comma);
                previously_linked.insert(outer_period);
                previously_linked.insert(outer_strip_period);
                previously_linked.insert(outer_possessive);
                previously_linked.insert(outer_strip_possessive);
            }
        // If no slots contain capital words...
        } else {
            linked.push_str(&(index[0].to_owned() + " "));
        }
    }
}
