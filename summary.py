from sumy.parsers.plaintext import PlaintextParser
from sumy.nlp.tokenizers import Tokenizer
from sumy.summarizers.lsa import LsaSummarizer
import sys

# Input
with open(sys.argv[1], 'r') as input_file:
    input = input_file.read()

    # Parse Input
    parser = PlaintextParser.from_string(input, Tokenizer("english"))

    # Create an LSA Summarizer
    summarizer = LsaSummarizer()

    # Generate the summary - 200 sentence document
    summary = summarizer(parser.document, sentences_count = sys.argv[2])

    # Output 
    output = ""
    print("Summary:")
    for sentence in summary:
        string = f'{sentence}'
        output = output + string + ' '
    print(output)

input_file.close()
