#!/usr/bin/env -S sh -euo pipefail

echo "Select a speech-to-text model:"
echo "1. English Model with numbers and punctuation (april-english-dev-01110_en.april)"
echo "2. English Model with no numbers or punctuation (aprilv0_en-us.april)"
echo "3. Polish Model (april-polish-dev-2_pl.april)"
echo "4. French Model (april-french-dev-1_fr.april)"

# Set default choice to 1
default_choice=1

read -p "Enter the number corresponding to your choice (default is $default_choice): " model_choice

# Use default choice if input is empty
model_choice=${model_choice:-$default_choice}

case $model_choice in
    1)
        model_url="https://april.sapples.net/april-english-dev-01110_en.april"
        ;;
    2)
        model_url="https://april.sapples.net/aprilv0_en-us.april"
        ;;
    3)
        model_url="https://april.sapples.net/april-polish-dev-2_pl.april"
        ;;
    4)
        model_url="https://april.sapples.net/april-french-dev-1_fr.april"
        ;;
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac

echo "Selected model URL: $model_url"

# Download the selected model using wget
wget "$model_url" -O "./$(basename "$model_url")"

echo "Downloaded model to the current directory."
