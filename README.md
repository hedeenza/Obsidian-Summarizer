# Obsidian Summarizer

## Use Case:
- The ability to see the connections between sources and ideas is powerful in the information age, but processing the vast amounts of available information can be time-consuming.
- [Obsidian](https://obsidian.md/) is a phenomenal tool for gathering, organizing, distilling, and connecting information. Linking notes together can illuminate otherwise hidden connections and patterns, allowing for new and meaningful perspectives on familiar topics.
- This tool assists the process of transforming collected information into a format ready for an Obsidian vault by utilizing the [Latent Semantic Analysis (LSA) technique](https://github.com/miso-belica/sumy/blob/main/docs/summarizators.md#latent-semantic-analysis-lsa) of extractive summarization using the [sumy](https://github.com/miso-belica/sumy/tree/main) module for Python, detecting entities a user would be likely to want to link in an Obsidian note, and outputing a file containing a YAML properties header, a linked summary of the original text, and the unlinked original input text.

## Get the Tool
- The pre-compiled binary (for Linux and Windows) and source code are available in "Releases".
- macOS users will need to compile from source.

## Create and Activate the Virtual Environment
Create Virtual Environment: `python -m venv venv`

Activate Virtual Environment: 
- Windows Powershell: `.\venv\Scripts\activate`
- macOS + Linux: `source venv/bin/activate`

## Download Modules + Dependencies
Download required modules:
`$ pip install -r requirements.txt`

Download NLTK tokenizers for the Python `summary.py` script
`$ python get_nltk_tokenizers.py`
If there are errors with the SSL Certificate, you can download the zipped tokenizers at [*https://www.nltk.org/nltk_data/*](https://www.nltk.org/nltk_data/). The "punkt_tab" tokenizers are under item *78. Punkt Tokenizer Models*.  
Unzip the file, and place the unzipped folder in `~/nltk_data/tokenizers/`, creating if necessary, such that `~/nltk_data/tokenizers/punkt_tab/`  

You can move the `nltk_data` directory out of your home directory and retain function by setting the `NLTK_DATA` environment variable to point to the top level `nltk_data` directory. [Reference.](https://www.nltk.org/data.html#manual-installation)

## Setting Python Paths
Place the `.obsidian_summarizer_paths.env` file in your home directory, and edit to reflect the absolute paths of the virtual environment and `summary.py`
```
~/.obsidian_summarizer_paths.env
# Ensure the virtual environment path ends with "venv" and the script path ends with "summary.py" (without the quotation marks)
/home/path/to/this/venv
/home/path/to/summary.py
```

## Running the CLI
`$ ./obsidian-summarizer --input <INPUT> --output <OUTPUT> --summary-length <SUMMARY_LENGTH>`

Options:
- `-i, --input <INPUT>`                    Input text file to summarize
- `-o, --output <OUTPUT>`                  Output file name
- `-s, --summary-length <SUMMARY_LENGTH>`  Number of sentences in Summary
- `-h, --help`                             Print help
- `-g, --group-tag <GROUP_TAG>`            Group Tag, in CamelCase [default = None]
- `-V, --version`                          Print version

- Ensure the program has executable permissions.
- Experiment with different summary lengths before saving to a file.
- Group tags allow Obsidian to know that notes with the same tag are connected. By adding the same group tag to each file in a set, a Bases view can be created that allows the summaries for each note in the group to be viewed on one continuous feed. 

## Building from Source
Navigate to the project root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Obsidian Summarizer
export PATH="$PATH:/home/path/to/directory/where/the/rust/binary/lives"

alias osm="obsidian-summarizer"
```

## Companion Script - Count Sentences
`./count_sentences.sh <FILE>`

- Counts the number of sentences (ending in periods, question marks, or exclamation points) in a file.
- May be helpful in determining how long the <SUMMARY_LENGTH> should be.

## Known Issues
- Errors in named entity parsing and linking function version 2 where some stop words and entities are repeatedly linked.
