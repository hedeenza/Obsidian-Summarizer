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
- -i, --input <INPUT>                    Input text file to summarize
- -o, --output <OUTPUT>                  Output file name
- -s, --summary-length <SUMMARY_LENGTH>  Number of sentences in Summary
- -h, --help                             Print help
- -V, --version                          Print version

- Ensure the program has executable permissions.
- Experiment with different summary lengths before saving to a file.

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

## License
This program is distributed under the terms of a GNU GPLv3 license. See LICENSE.md for details.
