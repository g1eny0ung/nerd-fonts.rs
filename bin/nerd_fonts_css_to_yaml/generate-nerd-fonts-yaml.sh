echo "Getting nerd-fonts-generated.css ..."

curl -sSL https://raw.githubusercontent.com/ryanoasis/nerd-fonts/master/css/nerd-fonts-generated.css -o resources/nerd-fonts-generated.css

echo "Downloaded."

cargo run

mv resources/nerd-fonts-generated.yaml ../../resources/nerd-fonts-generated.yaml
