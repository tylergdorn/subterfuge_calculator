#!/usr/bin/env bash

# Path to the calculator binary
BIN="./target/debug/subterfuge_cal.exe"

# Declare heroes (name attack health)
declare -A ATTACK
declare -A HEALTH

ATTACK[Wraith]=12
HEALTH[Wraith]=1

ATTACK[Elf]=10
HEALTH[Elf]=2

ATTACK[Troll]=9
HEALTH[Troll]=3

ATTACK[Patriot]=8
HEALTH[Patriot]=2

ATTACK[Orc]=7
HEALTH[Orc]=3

ATTACK[Rider]=5
HEALTH[Rider]=3

# Hero list
HEROES=("Wraith" "Elf" "Troll" "Patriot" "Orc" "Rider")

echo "=== HERO vs HERO FIGHTS ==="
for A in "${HEROES[@]}"; do
    for D in "${HEROES[@]}"; do
        A_STR="${ATTACK[$A]},${HEALTH[$A]}"
        D_STR="${ATTACK[$D]},${HEALTH[$D]}"
        echo ""
        echo ">>> $A ($A_STR) vs $D ($D_STR)"
        $BIN "$A_STR" "$D_STR"
    done
done

echo ""
echo "=== HERO vs UNIT FIGHTS (4 and 8 units) ==="
for H in "${HEROES[@]}"; do
    H_STR="${ATTACK[$H]},${HEALTH[$H]}"

    # Hero vs 4 units
    echo ""
    echo ">>> $H ($H_STR) vs 4 units"
    $BIN "$H_STR" 4

    # Hero vs 8 units
    echo ""
    echo ">>> $H ($H_STR) vs 8 units"
    $BIN "$H_STR" 8
done

echo ""
echo "=== ALL FIGHTS COMPLETE ==="