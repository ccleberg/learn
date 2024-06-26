use bufio;
use fmt;
use fs;
use io;
use os;
use strings;

export fn main() void = {
	// Get user input
	const filename = get_filename();
	const filedata = get_input();

	// Save to file and return the message
	let message = save_input(filename, filedata);
	fmt::printfln("{}", message)!;
};

fn get_filename() str = {
	// Ask user for filename
	fmt::printfln("Enter a file name to create:")!;

	// Read the buffer
	const input = bufio::scanline(os::stdin)! as []u8;

	// Convert to string and return
	return strings::fromutf8(input)!;
};

fn get_input() []u8 = {
	// Ask user for input
	fmt::printfln("Enter some words or data to write to a file:")!;

	// Read the buffer
	const input = bufio::scanline(os::stdin)! as []u8;

	// Return the data
	return input;
};

fn save_input(file_name: str, file_data: []u8) str = {
	// USER_RW = 384, Read and write permissions for the file owner
	let file_mode: fs::mode = 384;

	// Create a file buffer as USER_RW using the provided file name
	let file_buffer = os::create(file_name, file_mode);

	// Handle the tagged union for (1) a successful file buffer, or (2) an
	// error
	match (file_buffer) {
	// (1) Write the data, close the file, and return a success message
	case let file: io::file =>
		io::write(file, file_data)!;
		io::close(file)!;
		return "Data saved successfully. Check the file you created!";
	// (2) Return the error message as a string
	case let e: fs::error =>
		return fs::strerror(e);
	};
};

