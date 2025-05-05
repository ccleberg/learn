use datetime;
use fmt;
use math::random;

export fn main() void = {
	const rgb = generate_rgb();
	fmt::printfln("RGB: ({}, {}, {})", rgb[0], rgb[1], rgb[2])!;
};

fn generate_rgb() []u64 = {
	// Use the current Unix epoch time as the seed value
	let datetime = datetime::epochunix(&datetime::now());

	// Generate initial pseudo-random value
	// You must cast the datetime from int to u64
	let x = random::init(datetime: u64);

	// Generate RGB values between (0, 255) using pseudo-random init value
	let r = random::u64n(&x, 255);
	let g = random::u64n(&x, 255);
	let b = random::u64n(&x, 255);

	// Structure data as array and return
	let rgb_array: [3]u64 = [r, g, b];
	return rgb_array;
};
