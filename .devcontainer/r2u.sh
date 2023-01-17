apt install --yes --no-install-recommends wget  	# to add the key
wget -q -O- https://eddelbuettel.github.io/r2u/assets/dirk_eddelbuettel_key.asc \
    | tee -a /etc/apt/trusted.gpg.d/cranapt_key.asc
echo "deb [arch=amd64] https://r2u.stat.illinois.edu/ubuntu focal main" \
    > /etc/apt/sources.list.d/cranapt.list
apt update