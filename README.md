# Image to CSV / Pixels
A Rust-based command-line tool to convert image files into CSV format and vice versa.

# Overview
This CLI tool converts images into a CSV format where each line represents a pixel with its coordinates and RGB values. 

It can also take a CSV file in this format as input and generate an image.

# How It Works
Each line in the CSV file represents a pixel's coordinates (X and Y) and its RGB values:
```
X:Y R G B
```
* `X:Y` - The pixel's coordinates.
* `R G B` - The pixel's RGB color values.


### Image -> CSV
* Input: An image file (e.g., PNG, JPEG).
* Output: CSV format printed to STDOUT.
```sh
pixels to-csv <img_file.jpg>
```

### CSV -> Image
* Input: CSV format from STDIN.
* Output: An image file.
```sh
pixels to-image -o OUTPUT_IMAGE.png < CSV_FILE.csv
# or
cat CSV_FILE.csv | pixels to-image -o OUTPUT_IMAGE.png
```

# How to install
```
git clone https://github.com/talis-fb/img-to-csv.git
cd img-to-csv
cargo build --release
sudo mv target/release/pixels /usr/local/bin/
```
The final binary is called `pixels`.

# How to Install
1. Clone the repository:
```sh
git clone https://github.com/talis-fb/img-to-csv.git
```
2. Navigate to the project directory:
```sh
cd img-to-csv
```
3. Build the project:
```sh 
cargo build --release
```
4. Move the binary to a directory in your PATH, for example:
```sh
sudo mv target/release/pixels /usr/local/bin/
```
5. The final binary is called `pixels`.
```sh
pixels to-csv <...>
pixels to-image <...>
```
