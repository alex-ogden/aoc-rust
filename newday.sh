
#!/usr/bin/env bash

# Usage: ./newday.sh YEAR DAY [VARIANTS]
# Example: ./newday.sh 2025 11
# Example with variants: ./newday.sh 2025 11 mt vis

YEAR=$1
DAY=$2
shift 2
VARIANTS=("$@")

if [[ -z "$YEAR" || -z "$DAY" ]]; then
    echo "Usage: $0 YEAR DAY [VARIANTS]"
    exit 1
fi

YEAR_DIR="src/_$YEAR"
DAY_FILE="$YEAR_DIR/day$DAY.rs"
MOD_FILE="$YEAR_DIR/mod.rs"

INPUT_DIR="inputs/$YEAR"
INPUT_FILE="$INPUT_DIR/day$DAY.txt"

# --- Step 0: create input file if missing ---
mkdir -p "$INPUT_DIR"
if [[ ! -f "$INPUT_FILE" ]]; then
    touch "$INPUT_FILE"
    echo "Created blank input file $INPUT_FILE"
fi

# --- Step 1: create dayX.rs if it doesn't exist ---
if [[ -f "$DAY_FILE" ]]; then
    echo "$DAY_FILE already exists!"
else
    cat > "$DAY_FILE" <<EOF
use crate::utils;

pub fn part1() {
    let input = utils::read_input($YEAR, $DAY, false);
    // let result = solve_part1(&input);
    // println!("${YEAR} :: Day ${DAY} :: Part 1: {}", result);
}

pub fn part2() {
    let input = utils::read_input($YEAR, $DAY, false);
    // let result = solve_part2(&input);
    // println!("${YEAR} :: Day ${DAY} :: Part 2: {}", result);
}
EOF
    echo "Created $DAY_FILE"
fi

# --- Step 2: add mod declaration to mod.rs if missing ---
if ! grep -q "pub mod day$DAY;" "$MOD_FILE"; then
    # Insert after the last pub mod line, or at end if none
    if grep -q "^pub mod " "$MOD_FILE"; then
        sed -i "/^pub mod /h; \$!d; x; G; s/\n$/\npub mod day$DAY;/" "$MOD_FILE" 2>/dev/null || echo "pub mod day$DAY;" >> "$MOD_FILE"
    else
        echo "pub mod day$DAY;" >> "$MOD_FILE"
    fi
    echo "Added pub mod day$DAY to $MOD_FILE"
fi

# --- Step 3: insert dayX::part1/part2 in order in PART1 and PART2 arrays ---
insert_in_array() {
    local array_name=$1
    local part_num=$2
    local line="    day$DAY::part$part_num,"
    if grep -q "^pub const $array_name:.*\[" "$MOD_FILE"; then
        if ! grep -q "$line" "$MOD_FILE"; then
            sed -i "/^pub const $array_name:.*\[/,/\]/{
                /]/i $line
            }" "$MOD_FILE"
            echo "Inserted $line into $array_name"
        fi
    fi
}

insert_in_array "PART1" 1
insert_in_array "PART2" 2

# --- Step 4: handle optional variants ---
for variant in "${VARIANTS[@]}"; do
    insert_in_array "PART1_$(echo $variant | tr a-z A-Z)" 1
    insert_in_array "PART2_$(echo $variant | tr a-z A-Z)" 2
done

echo "Done! New day $DAY added for $YEAR."
