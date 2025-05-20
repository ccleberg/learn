import argparse

# Create an argument parser to obtain file name
parser = argparse.ArgumentParser(description='cat')
parser.add_argument('-f', '--file', help='File name')
args = parser.parse_args()

# Open the file using the `file` argument
path = args.file
with open(path, 'r') as file:
        # Read and print the file contents
        contents = file.read()
        print(contents)
