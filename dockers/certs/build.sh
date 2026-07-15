#!/bin/bash

domain=grid.local
rm $domain  -Rf
./minica --domains $domain
cat ./$domain/cert.pem ./$domain/key.pem > ./$domain/all.pem
cp ./$domain/* ../etc/lighttpd/conf-enabled/certs/ -Rf
