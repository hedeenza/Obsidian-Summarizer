use regex::Regex;
use std::collections::HashSet;

fn main() {

    // Use sample text from Reuters article
    let text = "BEIJING, May 12 (Reuters) - A year ago, U.S. President Donald Trump predicted that towering trade tariffs would bring America's main economic rival to heel. He heads to China this week with that ambition blunted by court rulings, narrowing his goals to a few deals on beans, beef and Boeing (BA.N), opens new tab jets, and enlisting China's help to resolve his unpopular Iran war, political analysts say. The Reuters Iran Briefing newsletter keeps you informed with the latest developments and analysis of the Iran war. Sign up here. The modest expectations for Trump's May 14-15 meetings with Xi Jinping - the first since they ‌paused a bruising trade war in October - underscore how Trump's bombastic approach has failed to deliver an advantage ahead of the talks, according to analysts. Trump *kind of needs China more than China needs him,* said Alejandro Reyes, a professor specialising in Chinese foreign policy at the University of Hong Kong. *He needs a kind of foreign policy victory: a victory that shows that he is looking to ensure stability in the world and that he's not just disrupting global politics,* Reyes added. Since their last brief meeting at an airbase in South Korea where Trump suspended triple-digit tariffs on Chinese goods and Xi backed away from choking global supplies of rare earths, China has quietly sharpened its economic pressure toolkit aimed at Washington. Trump, meanwhile, has been preoccupied fighting U.S. court rulings against his tariffs and a war with Iran that has sapped his approval ratings ahead of November's midterm elections. This week's meeting in the Chinese capital will be a grander occasion, with the leaders set to hold a summit at the Great Hall of the People, tour UNESCO-heritage site Temple of Heaven, dine at a state banquet and take tea and lunch together. But the anticipated economic deliverables amount to a handful of deals and mechanisms to manage future trade, while it remains unclear whether the leaders will even agree to extend their trade truce, officials involved in the planning said. Trump will be joined by CEOs including Tesla's (TSLA.O), opens new tab Elon Musk and Apple's (AAPL.O), opens new tab Tim Cook, though the business delegation is smaller than when he last visited Beijing in 2017. Aside from trade, Trump said on Monday he will discuss arms sales to Taiwan and the case of jailed media tycoon Jimmy Lai with Xi. Families of two Americans imprisoned in China for more than a decade are also urging Trump to seek their release. *We used to be taken advantage of for years with our previous presidents, and now we're doing great with China,* Trump said. *I respect him (Xi) a lot, and hopefully he respects me.* 
    ONE BATTLE AFTER ANOTHER The mood music has changed dramatically since Trump declared in a Truth Social post in April 2025 that his tariffs would make China realise that the *days of ripping off* the United States were over. Those levies prompted Beijing to restrict exports of rare earths, brutally exposing the West's dependency on elements vital to the manufacturing of everything from electric cars to weapons, and eventually led to Trump and Xi's ‌fragile truce. Since then, Trump has faced countless other battles: capturing Venezuela's leader, threatening to annex fellow NATO member Greenland and waging a war on Iran that has plunged the Middle East into chaos and stoked a global energy crisis. More than 60% of Americans disapprove of his Iran war, according to a Reuters/Ipsos survey last month. Now, Trump wants China to convince Tehran to make a deal with Washington to end the conflict. China maintains ties with Iran and remains a major consumer of its oil exports. Matt Pottinger, who served as deputy national security advisor during Trump's first term, told a forum in Taipei last week that while China would like to see an outcome that weakens American power it is not immune to the economic cost of a protracted conflict. But Beijing will want something in return, and top of Xi's agenda is Taiwan, the democratically governed island claimed by China. While some fear a bargain that could embolden China to take Taiwan by force, even a nuanced change in Washington's wording would raise anxiety about the commitment of Taipei's most important backer that would reverberate across other U.S. allies in Asia. Wu Xinbo, a professor at Fudan University in Shanghai who serves on the policy advisory board of China's foreign ministry, said Trump should make clear that he *won't support independence or take actions that encourage a separatist political agenda*.
    'SUPERFICIAL CEASEFIRE' China also wants the Trump administration to commit to not taking future retaliatory trade action such as technology export controls, and to roll back existing controls on chipmaking equipment and advanced memory chips, people briefed on the talks said. And since last October, Beijing has been expanding its own economic leverage, such as enacting laws to punish foreign entities that shift supply chains away from China and tightening its rare earth licensing regime. A majority of Americans (53 percent) now say the United States should undertake friendly cooperation and engagement with China, up from 40 percent in 2024, according to a survey by the Chicago Council on Global Affairs published in October. So just keeping relations on an even keel and extending the trade war truce could be enough for Trump to claim a win. That leaves the main outcome likely to be *a superficial ceasefire that is largely to China's advantage,* said Scott Kennedy of the Center for Strategic and International Studies think tank in Washington.*";

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

        // If all 4 slots contain capital words...
        if w & x & y & z {
        // If the first 3 slots contain capital words...
        } else if w & x & y {
        // If the first 2 slots contain capital words...
        } else if w & x {
        // If only the first slots contains a capital word...
        } else if w {
        // If no slots contain capital words...
        } else {}
    }

}
