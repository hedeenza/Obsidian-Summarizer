use regex::Regex;
use std::collections::HashSet;

fn main() {

    // Use sample text from Reuters article
    let text = "BEIJING, May 12 (Reuters) - A year ago, U.S. President Donald Trump predicted that towering trade tariffs would bring America's main economic rival to heel. He heads to China this week with that ambition blunted by court rulings, narrowing his goals to a few deals on beans, beef and Boeing (BA.N), opens new tab jets, and enlisting China's help to resolve his unpopular Iran war, political analysts say. The Reuters Iran Briefing newsletter keeps you informed with the latest developments and analysis of the Iran war. Sign up here. The modest expectations for Trump's May 14-15 meetings with Xi Jinping - the first since they ‌paused a bruising trade war in October - underscore how Trump's bombastic approach has failed to deliver an advantage ahead of the talks, according to analysts. Trump *kind of needs China more than China needs him,* said Alejandro Reyes, a professor specialising in Chinese foreign policy at the University of Hong Kong. *He needs a kind of foreign policy victory: a victory that shows that he is looking to ensure stability in the world and that he's not just disrupting global politics,* Reyes added. Since their last brief meeting at an airbase in South Korea where Trump suspended triple-digit tariffs on Chinese goods and Xi backed away from choking global supplies of rare earths, China has quietly sharpened its economic pressure toolkit aimed at Washington. Trump, meanwhile, has been preoccupied fighting U.S. court rulings against his tariffs and a war with Iran that has sapped his approval ratings ahead of November's midterm elections. This week's meeting in the Chinese capital will be a grander occasion, with the leaders set to hold a summit at the Great Hall of the People, tour UNESCO-heritage site Temple of Heaven, dine at a state banquet and take tea and lunch together. But the anticipated economic deliverables amount to a handful of deals and mechanisms to manage future trade, while it remains unclear whether the leaders will even agree to extend their trade truce, officials involved in the planning said. Trump will be joined by CEOs including Tesla's (TSLA.O), opens new tab Elon Musk and Apple's (AAPL.O), opens new tab Tim Cook, though the business delegation is smaller than when he last visited Beijing in 2017. Aside from trade, Trump said on Monday he will discuss arms sales to Taiwan and the case of jailed media tycoon Jimmy Lai with Xi. Families of two Americans imprisoned in China for more than a decade are also urging Trump to seek their release. *We used to be taken advantage of for years with our previous presidents, and now we're doing great with China,* Trump said. *I respect him (Xi) a lot, and hopefully he respects me.* 
    ONE BATTLE AFTER ANOTHER The mood music has changed dramatically since Trump declared in a Truth Social post in April 2025 that his tariffs would make China realise that the *days of ripping off* the United States were over. Those levies prompted Beijing to restrict exports of rare earths, brutally exposing the West's dependency on elements vital to the manufacturing of everything from electric cars to weapons, and eventually led to Trump and Xi's ‌fragile truce. Since then, Trump has faced countless other battles: capturing Venezuela's leader, threatening to annex fellow NATO member Greenland and waging a war on Iran that has plunged the Middle East into chaos and stoked a global energy crisis. More than 60% of Americans disapprove of his Iran war, according to a Reuters/Ipsos survey last month. Now, Trump wants China to convince Tehran to make a deal with Washington to end the conflict. China maintains ties with Iran and remains a major consumer of its oil exports. Matt Pottinger, who served as deputy national security advisor during Trump's first term, told a forum in Taipei last week that while China would like to see an outcome that weakens American power it is not immune to the economic cost of a protracted conflict. But Beijing will want something in return, and top of Xi's agenda is Taiwan, the democratically governed island claimed by China. While some fear a bargain that could embolden China to take Taiwan by force, even a nuanced change in Washington's wording would raise anxiety about the commitment of Taipei's most important backer that would reverberate across other U.S. allies in Asia. Wu Xinbo, a professor at Fudan University in Shanghai who serves on the policy advisory board of China's foreign ministry, said Trump should make clear that he *won't support independence or take actions that encourage a separatist political agenda*.
    'SUPERFICIAL CEASEFIRE' China also wants the Trump administration to commit to not taking future retaliatory trade action such as technology export controls, and to roll back existing controls on chipmaking equipment and advanced memory chips, people briefed on the talks said. And since last October, Beijing has been expanding its own economic leverage, such as enacting laws to punish foreign entities that shift supply chains away from China and tightening its rare earth licensing regime. A majority of Americans (53 percent) now say the United States should undertake friendly cooperation and engagement with China, up from 40 percent in 2024, according to a survey by the Chicago Council on Global Affairs published in October. So just keeping relations on an even keel and extending the trade war truce could be enough for Trump to claim a win. That leaves the main outcome likely to be *a superficial ceasefire that is largely to China's advantage,* said Scott Kennedy of the Center for Strategic and International Studies think tank in Washington.*";

    // Create an empty string to contain the "linked" text.
    let mut linked_text = String::new();

    // Create Regex pattern to match words that start with a capital letter
    let capital_detect = Regex::new(r"^[A-Z]").unwrap();

    // Split text on white space and collect into a vector
    let mut text_vec: Vec<&str> = text.split(" ").collect();
    
    // The window will be 4 wide, so add 3 "trailing filler" values so the last word can fall into place
    text_vec.push("trailing filler");
    text_vec.push("trailing filler");
    text_vec.push("trailing filler");

    // Create a HashSet to contain a unique set of all words that have previously been linked
    // This will help prevent the same terms from getting linked over and over again
    let mut previously_linked: HashSet<Vec<Vec<String>>> = HashSet::new();

    // Create a window to examien the text vector 4 words at a time
    let mut chunks = text_vec.windows(4);

    // While there are still viable "windows"...
    while let Some(index) = chunks.next() {
        // Create values to stand in for the true/false results of matches to keep if statements cleaner
        let w = capital_detect.is_match(&index[0]);
        let x = capital_detect.is_match(&index[1]);
        let y = capital_detect.is_match(&index[2]);
        let z = capital_detect.is_match(&index[3]);

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
                chunks.next();
                chunks.next();
                chunks.next();
            // Otherwise...
            } else {
                // For each 
                for (i, _content) in slice.clone().into_iter().enumerate() {
                    // Add the opening linking brackets, [[, to the first word
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    // Add the closing linking brackets, ]], to the last word
                    } else if i == slice.len() - 1 {
                        // If there's a possessive, add the closing linking brackets before the 's
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        // If there's a comma, add the closing linking brackets before the comma
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        // If there's a period, add the closing linking brackets before the period
                        } else if index[i].contains(".") {
                            let stripped_word = index[i].replace(".", "");
                            let linked_wordi = format!("{}]].", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        // Otherwise, link the last word normally
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        }
                    // Otherwise don't add any linking brackets to the word
                    } else {
                        let linked_word = format!("{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
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
                chunks.next();
                chunks.next();
                chunks.next();

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
        } else if w & x & y | w & (index[1] == "of") & y | w & (index[1] == "the") & y | w & (index[1] == "for") & y {
            let mut tester = Vec::new();
            let slice: Vec<String> = vec![
                index[0].to_string(),
                index[1].to_string(),
                index[2].to_string(),
            ];
            tester.push(slice.clone());
            if previously_linked.contains(&tester) {
                chunks.next();
                chunks.next();
            } else {
                for (i, _content) in slice.clone().into_iter().enumerate() {
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    } else if i == slice.len() - 1 {
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        }
                    } else {
                        let linked_word = format!("{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    }
                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }
                previously_linked.insert(tester);
                chunks.next();
                chunks.next();

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
                chunks.next();
            } else {

                for (i, _content) in slice.clone().into_iter().enumerate() {
                    if i == 0 {
                        let linked_word = format!("[[{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    } else if i == slice.len() - 1 {
                        if index[i].contains("'s") {
                            let stripped_word = index[i].replace("'s", "");
                            let linked_wordi = format!("{}]]'s", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else if index[i].contains(",") {
                            let stripped_word = index[i].replace(",", "");
                            let linked_wordi = format!("{}]],", stripped_word);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        } else {
                            let linked_wordi = format!("{}]]", index[i]);
                            let linked_linei = index[i].replacen(index[i], &linked_wordi, i);
                            linked_text.push_str(&(linked_linei + " "));
                        }
                    } else {
                        let linked_word = format!("{}", index[i]);
                        let linked_line = index[i].replace(index[i], &linked_word);
                        linked_text.push_str(&(linked_line + " "));
                    }

                    let strip_comma = index[i].replace(",", "");
                    strip_comma_vec.push(strip_comma);
                    let strip_period = index[i].replace(".", "");
                    strip_period_vec.push(strip_period);
                    let strip_possessive = index[i].replace("'s", "");
                    strip_possessive_vec.push(strip_possessive);
                }

                chunks.next();

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
            tester.push(slice);
            if previously_linked.contains(&tester) {
                chunks.next();
            } else {

                if index[0].contains("'s") {
                    let stripped_word = index[0].replace("'s", "");
                    let linked_word0 = format!("[[{}]]'s", stripped_word);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked_text.push_str(&(linked_line0 + " "));
                } else if index[0].contains(",") {
                    let stripped_word = index[0].replace(",", "");
                    let linked_word0 = format!("[[{}]],", stripped_word);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked_text.push_str(&(linked_line0 + " "));
                } else {
                    let linked_word0 = format!("[[{}]]", index[0]);
                    let linked_line0 = index[0].replacen(index[0], &linked_word0, 1);
                    linked_text.push_str(&(linked_line0 + " "));
                }

                let linked_comma = format!("{},", index[0]);
                comma_vec.push(linked_comma);
                outer_comma.push(comma_vec);

                let strip_comma = index[0].replace(",", "");
                let linked_strip_comma = format!("{}", strip_comma);
                strip_comma_vec.push(linked_strip_comma);
                outer_strip_comma.push(strip_comma_vec);

                let linked_period = format!("{}.", index[0]);
                period_vec.push(linked_period);
                outer_period.push(period_vec);

                let strip_period = index[0].replace(".", "");
                let linked_strip_period = format!("{}", strip_period);
                strip_period_vec.push(linked_strip_period);
                outer_strip_period.push(strip_period_vec);

                let linked_possessive = format!("{}'s", index[0]);
                possessive_vec.push(linked_possessive);
                outer_possessive.push(possessive_vec);

                let strip_possessive = index[0].replace("'s", "");
                let linked_strip_possessive = format!("{}", strip_possessive);
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
            linked_text.push_str(&(index[0].to_owned() + " "));
        }
    }

    // Clean out the "stop words"
    for stop_word in STOP_WORDS {
        let linked_stop = format!("[[{}]]", stop_word);
        let cleaned_text = linked_text.replace(&linked_stop, stop_word);
        linked_text = cleaned_text;
    }

    println!("{}\n", text);

    println!("{}", linked_text);

}

const STOP_WORDS: [&str; 1179] = [ 
    "0o", "0s", "3a", "3b", "3d", "6b", "6o", "A", "A1", "A2", "A3",
    "A4", "Ab", "Able", "About", "Above", "Abst", "Ac", "Accordance", "According", "Accordingly",
    "Across", "Act", "Actually", "Ad", "Added", "Adj", "Ae", "Af", "Affected", "Affecting", "Affects",
    "After", "Afterwards", "Ag", "Again", "Against", "Ah", "Ain", "Ain't", "Aj", "Al", "All", "Allow",
    "Allows", "Almost", "Alone", "Along", "Already", "Also", "Although", "Always", "Am", "Among",
    "Amongst", "Amoungst", "Amount", "An", "And", "Announce", "Another", "Any", "Anybody", "Anyhow",
    "Anymore", "Anyone", "Anything", "Anyway", "Anyways", "Anywhere", "Ao", "Ap", "Apart", "Apparently",
    "Appear", "Appreciate", "Appropriate", "Approximately", "Ar", "Are", "Aren", "Arent", "Aren't",
    "Arise", "Around", "As", "A's", "Aside", "Ask", "Asking", "Associated", "At", "Au", "Auth", "Av",
    "Available", "Aw", "Away", "Awfully", "Ax", "Ay", "Az", "B", "B1", "B2", "B3", "Ba", "Back", "Bc",
    "Bd", "Be", "Became", "Because", "Become", "Becomes", "Becoming", "Been", "Before", "Beforehand",
    "Begin", "Beginning", "Beginnings", "Begins", "Behind", "Being", "Believe", "Below", "Beside",
    "Besides", "Best", "Better", "Between", "Beyond", "Bi", "Bill", "Biol", "Bj", "Bk", "Bl", "Bn",
    "Both", "Bottom", "Bp", "Br", "Brief", "Briefly", "Bs", "Bt", "Bu", "But", "Bx", "By", "C", "C1",
    "C2", "C3", "Ca", "Call", "Came", "Can", "Cannot", "Cant", "Can't", "Cause", "Causes", "Cc", "Cd",
    "Ce", "Certain", "Certainly", "Cf", "Cg", "Ch", "Changes", "Ci", "Cit", "Cj", "Cl", "Clearly", "Cm",
    "C'mon", "Cn", "Co", "Com", "Come", "Comes", "Con", "Concerning", "Consequently", "Consider",
    "Considering", "Contain", "Containing", "Contains", "Corresponding", "Could", "Couldn", "Couldnt",
    "Couldn't", "Course", "Cp", "Cq", "Cr", "Cry", "Cs", "C's", "Ct", "Cu", "Currently", "Cv", "Cx",
    "Cy", "Cz", "D", "D2", "Da", "Date", "Dc", "Dd", "De", "Definitely", "Describe", "Described",
    "Despite", "Detail", "Df", "Di", "Did", "Didn", "Didn't", "Different", "Dj", "Dk", "Dl", "Do",
    "Does", "Doesn", "Doesn't", "Doing", "Don", "Done", "Don't", "Down", "Downwards", "Dp", "Dr", "Ds",
    "Dt", "Du", "Due", "During", "Dx", "Dy", "E", "E2", "E3", "Ea", "Each", "Ec", "Ed", "Edu", "Ee",
    "Ef", "Effect", "Eg", "Ei", "Eight", "Eighty", "Either", "Ej", "El", "Eleven", "Else", "Elsewhere",
    "Em", "Empty", "En", "End", "Ending", "Enough", "Entirely", "Eo", "Ep", "Eq", "Er", "Es",
    "Especially", "Est", "Et", "Et-al", "Etc", "Eu", "Ev", "Even", "Ever", "Every", "Everybody",
    "Everyone", "Everything", "Everywhere", "Ex", "Exactly", "Example", "Except", "Ey", "F", "F2", "Fa",
    "Far", "Fc", "Few", "Ff", "Fi", "Fifteen", "Fifth", "Fify", "Fill", "Find", "Fire", "First", "Five",
    "Fix", "Fj", "Fl", "Fn", "Fo", "Followed", "Following", "Follows", "For", "Former", "Formerly",
    "Forth", "Forty", "Found", "Four", "Fr", "From", "Front", "Fs", "Ft", "Fu", "Full", "Further",
    "Furthermore", "Fy", "G", "Ga", "Gave", "Ge", "Get", "Gets", "Getting", "Gi", "Give", "Given",
    "Gives", "Giving", "Gj", "Gl", "Go", "Goes", "Going", "Gone", "Got", "Gotten", "Gr", "Greetings",
    "Gs", "Gy", "H", "H2", "H3", "Had", "Hadn", "Hadn't", "Happens", "Hardly", "Has", "Hasn", "Hasnt",
    "Hasn't", "Have", "Haven", "Haven't", "Having", "He", "Hed", "He'd", "He'll", "Hello", "Help",
    "Hence", "Her", "Here", "Hereafter", "Hereby", "Herein", "Heres", "Here's", "Hereupon", "Hers",
    "Herself", "Hes", "He's", "Hh", "Hi", "Hid", "Him", "Himself", "His", "Hither", "Hj", "Ho", "Home",
    "Hopefully", "How", "Howbeit", "However", "How's", "Hr", "Hs", "Http", "Hu", "Hundred", "Hy", "I",
    "I2", "I3", "I4", "I6", "I7", "I8", "Ia", "Ib", "Ibid", "Ic", "Id", "I'd", "Ie", "If", "Ig",
    "Ignored", "Ih", "Ii", "Ij", "Il", "I'll", "Im", "I'm", "Immediate", "Immediately", "Importance",
    "Important", "In", "Inasmuch", "Inc", "Indeed", "Index", "Indicate", "Indicated", "Indicates",
    "Information", "Inner", "Insofar", "Instead", "Interest", "Into", "Invention", "Inward", "Io", "Ip",
    "Iq", "Ir", "Is", "Isn", "Isn't", "It", "Itd", "It'd", "It'll", "Its", "It's", "Itself", "Iv",
    "I've", "Ix", "Iy", "Iz", "J", "Jj", "Jr", "Js", "Jt", "Ju", "Just", "K", "Ke", "Keep", "Keeps",
    "Kept", "Kg", "Kj", "Km", "Know", "Known", "Knows", "Ko", "L", "L2", "La", "Largely", "Last",
    "Lately", "Later", "Latter", "Latterly", "Lb", "Lc", "Le", "Least", "Les", "Less", "Lest", "Let",
    "Lets", "Let's", "Lf", "Like", "Liked", "Likely", "Line", "Little", "Lj", "Ll", "Ll", "Ln", "Lo",
    "Look", "Looking", "Looks", "Los", "Lr", "Ls", "Lt", "Ltd", "M", "M2", "Ma", "Made", "Mainly",
    "Make", "Makes", "Many", "May", "Maybe", "Me", "Mean", "Means", "Meantime", "Meanwhile", "Merely",
    "Mg", "Might", "Mightn", "Mightn't", "Mill", "Million", "Mine", "Miss", "Ml", "Mn", "Mo", "More",
    "Moreover", "Most", "Mostly", "Move", "Mr", "Mrs", "Ms", "Mt", "Mu", "Much", "Mug", "Must", "Mustn",
    "Mustn't", "My", "Myself", "N", "N2", "Na", "Name", "Namely", "Nay", "Nc", "Nd", "Ne", "Near",
    "Nearly", "Necessarily", "Necessary", "Need", "Needn", "Needn't", "Needs", "Neither", "Never",
    "Nevertheless", "New", "Next", "Ng", "Ni", "Nine", "Ninety", "Nj", "Nl", "Nn", "No", "Nobody",
    "Non", "None", "Nonetheless", "Noone", "Nor", "Normally", "Nos", "Not", "Noted", "Nothing", "Novel",
    "Now", "Nowhere", "Nr", "Ns", "Nt", "Ny", "O", "Oa", "Ob", "Obtain", "Obtained", "Obviously", "Oc",
    "Od", "Of", "Off", "Often", "Og", "Oh", "Oi", "Oj", "Ok", "Okay", "Ol", "Old", "Om", "Omitted",
    "On", "Once", "One", "Ones", "Only", "Onto", "Oo", "Op", "Oq", "Or", "Ord", "Os", "Ot", "Other",
    "Others", "Otherwise", "Ou", "Ought", "Our", "Ours", "Ourselves", "Out", "Outside", "Over",
    "Overall", "Ow", "Owing", "Own", "Ox", "Oz", "P", "P1", "P2", "P3", "Page", "Pagecount", "Pages",
    "Par", "Part", "Particular", "Particularly", "Pas", "Past", "Pc", "Pd", "Pe", "Per", "Perhaps",
    "Pf", "Ph", "Pi", "Pj", "Pk", "Pl", "Placed", "Please", "Plus", "Pm", "Pn", "Po", "Poorly",
    "Possible", "Possibly", "Potentially", "Pp", "Pq", "Pr", "Predominantly", "Present", "Presumably",
    "Previously", "Primarily", "Probably", "Promptly", "Proud", "Provides", "Ps", "Pt", "Pu", "Put",
    "Py", "Q", "Qj", "Qu", "Que", "Quickly", "Quite", "Qv", "R", "R2", "Ra", "Ran", "Rather", "Rc",
    "Rd", "Re", "Readily", "Really", "Reasonably", "Recent", "Recently", "Ref", "Refs", "Regarding",
    "Regardless", "Regards", "Related", "Relatively", "Research", "Research-articl", "Respectively",
    "Resulted", "Resulting", "Results", "Rf", "Rh", "Ri", "Right", "Rj", "Rl", "Rm", "Rn", "Ro", "Rq",
    "Rr", "Rs", "Rt", "Ru", "Run", "Rv", "Ry", "S", "S2", "Sa", "Said", "Same", "Saw", "Say", "Saying",
    "Says", "Sc", "Sd", "Se", "Sec", "Second", "Secondly", "Section", "See", "Seeing", "Seem", "Seemed",
    "Seeming", "Seems", "Seen", "Self", "Selves", "Sensible", "Sent", "Serious", "Seriously", "Seven",
    "Several", "Sf", "Shall", "Shan", "Shan't", "She", "Shed", "She'd", "She'll", "Shes", "She's",
    "Should", "Shouldn", "Shouldn't", "Should've", "Show", "Showed", "Shown", "Showns", "Shows", "Si",
    "Side", "Significant", "Significantly", "Similar", "Similarly", "Since", "Sincere", "Six", "Sixty",
    "Sj", "Sl", "Slightly", "Sm", "Sn", "So", "Some", "Somebody", "Somehow", "Someone", "Somethan",
    "Something", "Sometime", "Sometimes", "Somewhat", "Somewhere", "Soon", "Sorry", "Sp",
    "Specifically", "Specified", "Specify", "Specifying", "Sq", "Sr", "Ss", "St", "Still", "Stop",
    "Strongly", "Sub", "Substantially", "Successfully", "Such", "Sufficiently", "Suggest", "Sup",
    "Sure", "Sy", "System", "Sz", "T", "T1", "T2", "T3", "Take", "Taken", "Taking", "Tb", "Tc", "Td",
    "Te", "Tell", "Ten", "Tends", "Tf", "Th", "Than", "Thank", "Thanks", "Thanx", "That", "That'll",
    "Thats", "That's", "That've", "The", "Their", "Theirs", "Them", "Themselves", "Then", "Thence",
    "There", "Thereafter", "Thereby", "Thered", "Therefore", "Therein", "There'll", "Thereof",
    "Therere", "Theres", "There's", "Thereto", "Thereupon", "There've", "These", "They", "Theyd",
    "They'd", "They'll", "Theyre", "They're", "They've", "Thickv", "Thin", "Think", "Third", "This",
    "Thorough", "Thoroughly", "Those", "Thou", "Though", "Thoughh", "Thousand", "Three", "Throug",
    "Through", "Throughout", "Thru", "Thus", "Ti", "Til", "Tip", "Tj", "Tl", "Tm", "Tn", "To",
    "Together", "Too", "Took", "Top", "Toward", "Towards", "Tp", "Tq", "Tr", "Tried", "Tries", "Truly",
    "Try", "Trying", "Ts", "T's", "Tt", "Tv", "Twelve", "Twenty", "Twice", "Two", "Tx", "U", "U201d",
    "Ue", "Ui", "Uj", "Uk", "Um", "Un", "Under", "Unfortunately", "Unless", "Unlike", "Unlikely",
    "Until", "Unto", "Uo", "Up", "Upon", "Ups", "Ur", "Us", "Use", "Used", "Useful", "Usefully",
    "Usefulness", "Uses", "Using", "Usually", "Ut", "V", "Va", "Value", "Various", "Vd", "Ve", "Ve",
    "Very", "Via", "Viz", "Vj", "Vo", "Vol", "Vols", "Volumtype", "Vq", "Vs", "Vt", "Vu", "W", "Wa",
    "Want", "Wants", "Was", "Wasn", "Wasnt", "Wasn't", "Way", "We", "Wed", "We'd", "Welcome", "Well",
    "We'll", "Well-b", "Went", "Were", "We're", "Weren", "Werent", "Weren't", "We've", "What",
    "Whatever", "What'll", "Whats", "What's", "When", "Whence", "Whenever", "When's", "Where",
    "Whereafter", "Whereas", "Whereby", "Wherein", "Wheres", "Where's", "Whereupon", "Wherever",
    "Whether", "Which", "While", "Whim", "Whither", "Who", "Whod", "Whoever", "Whole", "Who'll", "Whom",
    "Whomever", "Whos", "Who's", "Whose", "Why", "Why's", "Wi", "Widely", "Will", "Willing", "Wish",
    "With", "Within", "Without", "Wo", "Won", "Wonder", "Wont", "Won't", "Words", "World", "Would",
    "Wouldn", "Wouldnt", "Wouldn't", "Www", "X", "X1", "X2", "X3", "Xf", "Xi", "Xj", "Xk", "Xl", "Xn",
    "Xo", "Xs", "Xt", "Xv", "Xx", "Y", "Y2", "Yes", "Yet", "Yj", "Yl", "You", "Youd", "You'd", "You'll",
    "Your", "Youre", "You're", "Yours", "Yourself", "Yourselves", "You've", "Yr", "Ys", "Yt", "Z",
    "Zero", "Zi", "Zz", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday",
    "January", "February", "March", "April", "May", "June", "July", "August", "September", "October",
    "November", "December", 
    ];
