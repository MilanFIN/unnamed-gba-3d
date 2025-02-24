import os
import json

levels_dir = './levels'
output_file = './src/levels/levelstore.rs'

level_files = [f for f in os.listdir(levels_dir) if f.endswith('.json')]
level_files.sort()

rust_strings = []

max_length = 0


for i, level_file in enumerate(level_files):

	with open(os.path.join(levels_dir, level_file), 'r') as f:
		level_data = f.read()
		try:
			data = json.loads(level_data)
			if not isinstance(data, list):
				raise ValueError("JSON is not an array")

			max_length = max(max_length, len(data))

		except (json.JSONDecodeError, ValueError) as e:
			print(f"Error in file {level_file}: {e}")
			exit(1)
		rust_string = f'const LEVEL{i+1}: &str = r#"{level_data}"#;'
		rust_strings.append(rust_string)
level_count = len(level_files)
levels_array = f'pub const LEVELS: [&\'static str; {level_count}] = [{", ".join([f"LEVEL{i+1}" for i in range(level_count)])}];'
level_size_var = f'pub const LEVELSIZE: usize = {max_length};'

#completed_levels_array = f'pub static mut COMPLETED_LEVELS: [bool; {level_count}] = [false; {level_count}];'

with open(output_file, 'w') as f:
	f.write('// DO NOT EDIT: This file was automatically generated by bundle_levels.py.\n')
	f.write('// Any modifications will be overwritten during the next build.\n\n')
	f.write('\n\n' + level_size_var)
	f.write('\n\n' + '\n\n'.join(rust_strings))
	f.write('\n\n' + levels_array)
	#f.write('\n\n' + completed_levels_array)

#with open(output_file, 'a') as f:

print(f"Generated {len(level_files)} Rust string declarations in {output_file}")