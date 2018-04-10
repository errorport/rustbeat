Use the following command to get crazy:
'''sh
cargo run | sox -r 12000 -b 8 -c 1 -t raw -e unsigned-integer - -d
'''
